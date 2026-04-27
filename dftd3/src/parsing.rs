//! Flexible parsing of DFT-D3 damping parameters from TOML or JSON input.
//!
//! This module provides functions to parse parameter specifications that can
//! combine method lookups from the database with direct parameter values,
//! overrides, and the `atm` flag for three-body dispersion control.
//!
//! # Supported input formats
//!
//! - Method lookup: `{version = "bj", method = "b3lyp"}`
//! - Direct parameters: `{version = "bj", a1 = 0.5, s8 = 2.0, a2 = 4.0}`
//! - Method with override: `{version = "bj", method = "b3lyp", a1 = 0.5}`
//! - ATM flag: `{version = "bj", method = "b3lyp", atm = false}` (sets s9 =
//!   0.0)
//! - Combined: `{version = "bj", a1 = 0.5, s8 = 2.0, a2 = 4.0, atm = false}`

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
pub fn parse_damping_param(input: &Table) -> Result<DFTD3DampingParam, DFTD3Error> {
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
        toml::from_str(trimmed).map_err(|e| DFTD3Error::ParametersError(format!("TOML parsing error: {e}")))
    }
}

/// Parse damping parameters from a TOML string.
///
/// Supports both standard TOML documents and inline table syntax like:
/// - `{version = "bj", method = "b3lyp"}`
/// - `{version = "bj", a1 = 0.5, s8 = 2.0, a2 = 4.0}`
///
/// # Example
///
/// ```ignore
/// let input = r#"{version = "bj", method = "b3lyp"}"#;
/// let param = parse_damping_param_from_toml(input)?;
/// ```
pub fn parse_damping_param_from_toml(input: &str) -> Result<DFTD3DampingParam, DFTD3Error> {
    let table = parse_toml_table(input)?;
    parse_damping_param(&table)
}

/// Parse damping parameters from a JSON string.
///
/// Requires the `json` feature.
///
/// # Example
///
/// ```ignore
/// let input = r#"{"version": "bj", "method": "b3lyp"}"#;
/// let param = parse_damping_param_from_json(input)?;
/// ```
#[cfg(feature = "json")]
pub fn parse_damping_param_from_json(input: &str) -> Result<DFTD3DampingParam, DFTD3Error> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| DFTD3Error::ParametersError(format!("JSON parsing error: {e}")))?;
    let table = json_value_to_toml_table(&value)?;
    parse_damping_param(&table)
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
