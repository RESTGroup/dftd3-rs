//! Flexible parsing of DFT-D3 damping parameters from TOML or JSON input.
//!
//! This module provides functions to parse parameter specifications that can
//! combine method lookups from the database with direct parameter values,
//! overrides, and the `atm` flag for three-body dispersion control.
//!
//! # Supported input formats
//!
//! - **Usual case with method**: `{version = "d3bj", method = "b3lyp"}` Lookup
//!   B3LYP-D3(BJ) parameters from the database.
//! - **Version without d3 prefix**: `{version = "bj", method = "b3lyp"}` The
//!   `d3` prefix is optional. The `version` field is case-insensitive, so `BJ`
//!   works too.
//! - **Direct parameters**: `{version = "d3bj", a1 = 0.3981, s8 = 1.9889, a2 =
//!   4.4211}` Specify parameters directly without using the database.
//! - **ATM flag (no three-body)**: `{version = "bj", method = "b3lyp", atm =
//!   false}` Sets `s9 = 0.0`. Default is `atm = true` (s9 = 1.0).
//! - **Parameter override**: `{version = "d3bj", method = "b3lyp", a1 = 0.5}`
//!   Use database values but override `a1` to 0.5.
//! - **Direct params + ATM**: `{version = "d3bj", a1 = 0.3981, s8 = 1.9889, a2
//!   = 4.4211, atm = false}` Direct parameters with `s9 = 0.0`. If both `s9`
//!   and `atm` are provided, `s9` takes precedence.
//! - **Method name normalization**: `{version = "zero", method = "m06-2x"}`
//!   Separators like `-`, `_` are removed automatically (normalized to `m062x`
//!   for lookup).
//! - **Invalid field error**: `{version = "d3bj", method = "b3lyp", rs6 = 0.5}`
//!   Returns an error because `rs6` is not a valid parameter for the `bj`
//!   variant.
//!
//! # Example
//!
//! ```
//! use dftd3::prelude::*;
//!
//! // B3LYP with Becke-Johnson damping, no overrides, atm = true (default)
//! let input = r#"{version = "d3bj", method = "b3lyp"}"#;
//! // toml parameter type
//! let damping_param = dftd3_parse_damping_param_from_toml(input);
//! // FFI parameter type
//! let dftd3_param = damping_param.new_param();
//!
//! let atom_charges = vec![8, 1, 1];
//! // coordinates in bohr
//! #[rustfmt::skip]
//! let coordinates = vec![
//!     0.000000, 0.000000, 0.000000,
//!     0.000000, 0.000000, 1.807355,
//!     1.807355, 0.000000, -0.452500,
//! ];
//! let model = DFTD3Model::new(&atom_charges, &coordinates, None, None);
//! let res = model.get_dispersion(&dftd3_param, false);
//! let eng = res.energy;
//! println!("Dispersion energy: {eng}");
//! ```

use crate::interface::DFTD3Error;
use crate::parameters::{
    convert_to_damping_param, get_default_param_table, get_merged_param_table, normalize_version,
    DFTD3DampingParam,
};
use toml::Table;

/// Meta-fields that control parsing but are not damping parameters themselves.
const META_FIELDS: &[&str] = &["version", "method", "atm"];

/// Valid damping parameter fields for each version.
#[cfg(feature = "api-v0_4")]
fn valid_fields_for_version(version: &str) -> Result<&[&str], DFTD3Error> {
    match version {
        "bj" => Ok(&["s6", "s8", "s9", "a1", "a2", "alp"]),
        "zero" => Ok(&["s6", "s8", "s9", "rs6", "rs8", "alp"]),
        "bjm" => Ok(&["s6", "s8", "s9", "a1", "a2", "alp"]),
        "zerom" => Ok(&["s6", "s8", "s9", "rs6", "rs8", "alp", "bet"]),
        #[cfg(feature = "api-v0_5")]
        "op" => Ok(&["s6", "s8", "s9", "a1", "a2", "alp", "bet"]),
        #[cfg(feature = "api-v1_3")]
        "cso" => Ok(&["s6", "s9", "a1", "a2", "a3", "a4", "alp"]),
        #[cfg(not(feature = "api-v0_5"))]
        "op" => Err(DFTD3Error::ParametersError(format!(
            "Variant '{version}' requires api-v0_5 feature"
        ))),
        #[cfg(not(feature = "api-v1_3"))]
        "cso" => Err(DFTD3Error::ParametersError(format!(
            "Variant '{version}' requires api-v1_3 feature"
        ))),
        _ => Err(DFTD3Error::ParametersError(format!("Unknown variant: {version}"))),
    }
}

/// Parse damping parameters from a TOML table.
///
/// The table must contain a `version` field specifying the DFT-D3 variant.
/// Optional `method` field triggers a database lookup, and `atm` controls
/// the three-body dispersion term (s9). Remaining fields are treated as
/// damping parameters or overrides.
///
/// # Errors
///
/// Returns an error if:
/// - `version` is missing or unrecognized
/// - `method` is specified but not found in the database
/// - A field not valid for the given variant is present
/// - Required damping parameters are missing
pub fn dftd3_parse_damping_param(input: &Table) -> DFTD3DampingParam {
    dftd3_parse_damping_param_f(input).unwrap()
}

pub fn dftd3_parse_damping_param_f(input: &Table) -> Result<DFTD3DampingParam, DFTD3Error> {
    // 1. Extract version (required)
    let version_raw = input
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| DFTD3Error::ParametersError("Missing required field 'version'".into()))?;
    let version = normalize_version(version_raw);

    // 2. Extract method (optional)
    let method = input.get("method").and_then(|v| v.as_str());

    // 3. Extract atm flag (optional, default true)
    let atm = input.get("atm").and_then(|v| v.as_bool()).unwrap_or(true);

    // 4. Check whether s9 is explicitly provided by user
    let s9_explicit = input.contains_key("s9");

    // 5. Collect user-provided parameter fields (excluding meta-fields)
    #[cfg(feature = "api-v0_4")]
    let valid_fields = valid_fields_for_version(&version)?;
    #[cfg(not(feature = "api-v0_4"))]
    {
        // Validate version even without api-v0_4
        match version.as_str() {
            "bj" | "zero" | "bjm" | "zerom" => {
                return Err(DFTD3Error::ParametersError(format!(
                    "Variant '{version}' requires api-v0_4 feature or higher"
                )))
            },
            _ => return Err(DFTD3Error::ParametersError(format!("Unknown variant: {version}"))),
        }
    }

    let user_param_keys: Vec<&str> =
        input.keys().map(|k| k.as_str()).filter(|k| !META_FIELDS.contains(k)).collect();

    // Validate unknown fields against valid fields for this version
    for key in &user_param_keys {
        if !valid_fields.contains(key) {
            return Err(DFTD3Error::ParametersError(format!(
                "Unknown parameter '{}' for variant '{}' (d3{})",
                key, version, version
            )));
        }
    }

    // 6. Build the merged parameter table
    let mut merged = if let Some(method) = method {
        // Method lookup: get base table from database
        get_merged_param_table(method, &version)?
    } else {
        // No method: start from defaults
        get_default_param_table(&version)?
    };

    // 7. Apply user parameter overrides on top
    for key in &user_param_keys {
        if let Some(value) = input.get(*key) {
            merged.insert((*key).to_string(), value.clone());
        }
    }

    // 8. Handle atm -> s9 relationship If s9 is explicitly provided, it takes
    //    precedence over atm. If s9 is not explicit and atm is false, set s9 = 0.0.
    if !atm && !s9_explicit {
        merged.insert("s9".to_string(), toml::Value::Float(0.0));
    }

    // 9. Remove non-deserializable fields (like "damping" from defaults)
    merged.remove("damping");

    // 10. Convert to DFTD3DampingParam
    convert_to_damping_param(&merged, &version)
}

/// Parse a TOML string to a table. Supports both standard TOML documents
/// and inline table syntax like `{version = "bj", method = "b3lyp"}`.
fn parse_toml_table(input: &str) -> Result<Table, DFTD3Error> {
    let trimmed = input.trim();
    if trimmed.starts_with('{') {
        // Inline table: wrap in dummy key to make valid TOML document
        let wrapped = format!("x = {trimmed}");
        let mut doc: Table = toml::from_str(&wrapped)
            .map_err(|e| DFTD3Error::ParametersError(format!("TOML parsing error: {e}")))?;
        if let Some(toml::Value::Table(table)) = doc.remove("x") {
            Ok(table)
        } else {
            Err(DFTD3Error::ParametersError("Invalid TOML format".into()))
        }
    } else {
        toml::from_str(trimmed)
            .map_err(|e| DFTD3Error::ParametersError(format!("TOML parsing error: {e}")))
    }
}

/// Parse damping parameters from a TOML string (panics on error).
///
/// Supports both standard TOML documents and inline table syntax.
/// See the [module-level documentation](self) for all supported input formats.
///
/// # Panics
///
/// Panics if parsing fails. Use [`dftd3_parse_damping_param_from_toml_f`] for a
/// fallible version.
///
/// # Example
///
/// ```
/// use dftd3::prelude::*;
///
/// let input = r#"{version = "bj", method = "b3lyp"}"#;
/// let param = dftd3_parse_damping_param_from_toml(input);
/// ```
pub fn dftd3_parse_damping_param_from_toml(input: &str) -> DFTD3DampingParam {
    dftd3_parse_damping_param_from_toml_f(input).unwrap()
}

/// Parse damping parameters from a TOML string (fallible version).
///
/// Supports both standard TOML documents and inline table syntax.
/// See the [module-level documentation](self) for all supported input formats.
///
/// # Errors
///
/// Returns an error if:
/// - TOML parsing fails
/// - Required field `version` is missing
/// - Method not found in database
/// - Unknown parameter field for the given variant
/// - Parameter deserialization fails
pub fn dftd3_parse_damping_param_from_toml_f(input: &str) -> Result<DFTD3DampingParam, DFTD3Error> {
    let table = parse_toml_table(input)?;
    dftd3_parse_damping_param_f(&table)
}

/// Parse damping parameters from a JSON string (panics on error).
///
/// Requires the `json` feature.
/// Supports the same input formats as [`dftd3_parse_damping_param_from_toml`].
///
/// # Panics
///
/// Panics if parsing fails. Use [`dftd3_parse_damping_param_from_json_f`] for a
/// fallible version.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "json")]
/// # {
/// use dftd3::prelude::*;
///
/// let input = r#"{"version": "bj", "method": "b3lyp"}"#;
/// let param = dftd3_parse_damping_param_from_json(input);
/// # }
/// ```
#[cfg(feature = "json")]
pub fn dftd3_parse_damping_param_from_json(input: &str) -> DFTD3DampingParam {
    dftd3_parse_damping_param_from_json_f(input).unwrap()
}

/// Parse damping parameters from a JSON string (fallible version).
///
/// Requires the `json` feature.
/// Supports the same input formats as
/// [`dftd3_parse_damping_param_from_toml_f`].
///
/// # Errors
///
/// Returns an error if:
/// - JSON parsing fails
/// - Required field `version` is missing
/// - Method not found in database
/// - Unknown parameter field for the given variant
/// - Parameter deserialization fails
#[cfg(feature = "json")]
pub fn dftd3_parse_damping_param_from_json_f(input: &str) -> Result<DFTD3DampingParam, DFTD3Error> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| DFTD3Error::ParametersError(format!("JSON parsing error: {e}")))?;
    let table = json_value_to_toml_table(&value)?;
    dftd3_parse_damping_param_f(&table)
}

/// Convert a JSON object to a TOML table.
#[cfg(feature = "json")]
fn json_value_to_toml_table(value: &serde_json::Value) -> Result<Table, DFTD3Error> {
    match value {
        serde_json::Value::Object(map) => {
            let mut table = Table::new();
            for (k, v) in map {
                table.insert(k.clone(), json_value_to_toml(v)?);
            }
            Ok(table)
        },
        _ => Err(DFTD3Error::ParametersError("JSON root must be an object".into())),
    }
}

/// Convert a JSON value to a TOML value.
#[cfg(feature = "json")]
fn json_value_to_toml(value: &serde_json::Value) -> Result<toml::Value, DFTD3Error> {
    match value {
        serde_json::Value::Null => {
            Err(DFTD3Error::ParametersError("JSON null is not supported".into()))
        },
        serde_json::Value::Bool(b) => Ok(toml::Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else {
                Err(DFTD3Error::ParametersError("Unsupported JSON number".into()))
            }
        },
        serde_json::Value::String(s) => Ok(toml::Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let vec: Result<Vec<_>, _> = arr.iter().map(json_value_to_toml).collect();
            Ok(toml::Value::Array(vec?))
        },
        serde_json::Value::Object(map) => {
            let mut table = Table::new();
            for (k, v) in map {
                table.insert(k.clone(), json_value_to_toml(v)?);
            }
            Ok(toml::Value::Table(table))
        },
    }
}

#[test]
fn test_dftd3_parse_damping_param_from_toml_doc() {
    use crate::prelude::*;
    // B3LYP with Becke-Johnson damping, no overrides, atm = true (default)
    let input = r#"{version = "d3bj", method = "b3lyp"}"#;
    // toml parameter type
    let damping_param = dftd3_parse_damping_param_from_toml(input);
    // FFI parameter type
    let dftd3_param = damping_param.new_param();

    let atom_charges = vec![8, 1, 1];
    // coordinates in bohr
    #[rustfmt::skip]
    let coordinates = vec![
        0.000000, 0.000000, 0.000000,
        0.000000, 0.000000, 1.807355,
        1.807355, 0.000000, -0.452500,
    ];
    let model = DFTD3Model::new(&atom_charges, &coordinates, None, None);
    let res = model.get_dispersion(&dftd3_param, false);
    let eng = res.energy;
    println!("Dispersion energy: {eng}");

    // custom parameter
    let input = r#"{version = "d3bj", a1 = 0.3981, s8 = 1.9889, a2 = 4.4211, atm = false}"#;
    let damping_param = dftd3_parse_damping_param_from_toml(input);
    let dftd3_param = damping_param.new_param();
    let res = model.get_dispersion(&dftd3_param, false);
    let eng = res.energy;
    println!("Dispersion energy with custom params: {eng}");
}
