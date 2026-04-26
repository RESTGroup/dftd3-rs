//! DFTD3 damping parameter database loaded from TOML.
//!
//! This module provides access to DFT-D3 damping parameters by reading from an
//! embedded TOML database. Unlike the FFI-based parameter loading
//! (`DFTD3Param`), this module exposes the actual parameter values and allows
//! inspection of damping parameters for various XC functionals.

use crate::interface::*;
use serde::de::IntoDeserializer;
use serde::Deserialize;
use std::collections::HashMap;
use toml::Table;

// Embed TOML at compile time
const PARAMETERS_TOML: &str = include_str!("parameters.toml");

/* #region TOML data structures */

/// D3 variants under a method (e.g., d3.bj, d3.zero under [parameter.b3lyp]).
/// TOML creates nested structure: parameter.b3lyp.d3.bj
#[derive(Debug, Clone, Deserialize, Default)]
struct D3Variants {
    d3: D3MethodParams,
}

/// D3 parameters for a specific method.
/// Each variant is stored as a raw TOML table for direct deserialization.
#[derive(Debug, Clone, Deserialize, Default)]
struct D3MethodParams {
    bj: Option<Table>,
    zero: Option<Table>,
    bjm: Option<Table>,
    zerom: Option<Table>,
    #[serde(default)]
    op: Option<Table>,
    #[serde(default)]
    cso: Option<Table>,
}

/// Full TOML structure.
#[derive(Debug, Clone, Deserialize)]
struct ParameterDataBase {
    default: DefaultSection,
    parameter: HashMap<String, D3Variants>,
}

/// Default section with default damping types and base parameters.
/// Note: TOML `[default.parameter]` with `d3.bj = {...}` creates nested
/// structure.
#[derive(Debug, Clone, Deserialize)]
struct DefaultSection {
    #[allow(dead_code)]
    d3: Vec<String>,
    parameter: DefaultParameterSection,
}

/// Nested section for default parameters under [default.parameter].
/// TOML creates `default.parameter.d3` as a nested table.
#[derive(Debug, Clone, Deserialize)]
struct DefaultParameterSection {
    d3: D3DefaultParams,
}

/// Default D3 parameters for each damping variant.
/// Each variant is stored as a raw TOML table.
#[derive(Debug, Clone, Deserialize)]
struct D3DefaultParams {
    bj: Table,
    zero: Table,
    bjm: Table,
    zerom: Table,
    #[serde(default)]
    op: Option<Table>,
    #[serde(default)]
    cso: Option<Table>,
}

/* #endregion */

/* #region Public parameter structs */

/// Damping parameters with actual values exposed, plus optional metadata like
/// DOI. This wraps a damping-type-specific enum and provides DOI reference.
#[derive(Debug, Clone)]
pub struct DFTD3DampingParam {
    /// The actual damping parameters (variant-specific)
    pub param: DFTD3DampingParamEnum,
    /// Reference DOI if available
    pub doi: Option<String>,
}

/// Enum holding variant-specific damping parameters.
/// Each variant uses the corresponding struct from interface.rs.
#[derive(Debug, Clone)]
pub enum DFTD3DampingParamEnum {
    #[cfg(feature = "api-v0_4")]
    Rational(DFTD3RationalDampingParam),
    #[cfg(feature = "api-v0_4")]
    Zero(DFTD3ZeroDampingParam),
    #[cfg(feature = "api-v0_4")]
    ModifiedRational(DFTD3ModifiedRationalDampingParam),
    #[cfg(feature = "api-v0_4")]
    ModifiedZero(DFTD3ModifiedZeroDampingParam),
    #[cfg(feature = "api-v0_5")]
    OptimizedPower(DFTD3OptimizedPowerDampingParam),
    #[cfg(feature = "api-v1_3")]
    CSO(DFTD3CSODampingParam),
}

impl DFTD3DampingParamEnum {
    /// Get s6 value (scaling for C6 term).
    pub fn s6(&self) -> f64 {
        match self {
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Rational(data) => data.s6,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Zero(data) => data.s6,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedRational(data) => data.s6,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedZero(data) => data.s6,
            #[cfg(feature = "api-v0_5")]
            DFTD3DampingParamEnum::OptimizedPower(data) => data.s6,
            #[cfg(feature = "api-v1_3")]
            DFTD3DampingParamEnum::CSO(data) => data.s6,
        }
    }

    /// Get s9 value (scaling for three-body ATM term).
    pub fn s9(&self) -> f64 {
        match self {
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Rational(data) => data.s9,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Zero(data) => data.s9,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedRational(data) => data.s9,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedZero(data) => data.s9,
            #[cfg(feature = "api-v0_5")]
            DFTD3DampingParamEnum::OptimizedPower(data) => data.s9,
            #[cfg(feature = "api-v1_3")]
            DFTD3DampingParamEnum::CSO(data) => data.s9,
        }
    }

    /// Get alp value (damping exponent).
    pub fn alp(&self) -> f64 {
        match self {
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Rational(data) => data.alp,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Zero(data) => data.alp,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedRational(data) => data.alp,
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedZero(data) => data.alp,
            #[cfg(feature = "api-v0_5")]
            DFTD3DampingParamEnum::OptimizedPower(data) => data.alp,
            #[cfg(feature = "api-v1_3")]
            DFTD3DampingParamEnum::CSO(data) => data.alp,
        }
    }

    /// Get s8 value (scaling for C8 term), if present.
    pub fn s8(&self) -> Option<f64> {
        match self {
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Rational(data) => Some(data.s8),
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::Zero(data) => Some(data.s8),
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedRational(data) => Some(data.s8),
            #[cfg(feature = "api-v0_4")]
            DFTD3DampingParamEnum::ModifiedZero(data) => Some(data.s8),
            #[cfg(feature = "api-v0_5")]
            DFTD3DampingParamEnum::OptimizedPower(data) => Some(data.s8),
            #[cfg(feature = "api-v1_3")]
            DFTD3DampingParamEnum::CSO(_) => None, // CSO doesn't have s8
        }
    }
}

/* #endregion */

/* #region DFTD3ParamAPI implementation */

#[cfg(feature = "api-v0_4")]
impl DFTD3ParamAPI for DFTD3DampingParamEnum {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        match self {
            DFTD3DampingParamEnum::Rational(data) => data.new_param_f(),
            DFTD3DampingParamEnum::Zero(data) => data.new_param_f(),
            DFTD3DampingParamEnum::ModifiedRational(data) => data.new_param_f(),
            DFTD3DampingParamEnum::ModifiedZero(data) => data.new_param_f(),
            #[cfg(feature = "api-v0_5")]
            DFTD3DampingParamEnum::OptimizedPower(data) => data.new_param_f(),
            #[cfg(feature = "api-v1_3")]
            DFTD3DampingParamEnum::CSO(data) => data.new_param_f(),
        }
    }
}

#[cfg(feature = "api-v0_4")]
impl DFTD3ParamAPI for DFTD3DampingParam {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        self.param.new_param_f()
    }
}

/* #endregion */

/* #region Public API functions */

/// Load the parameter database from embedded TOML.
fn load_data_base() -> Result<ParameterDataBase, DFTD3Error> {
    toml::from_str(PARAMETERS_TOML)
        .map_err(|e| DFTD3Error::ParametersError(format!("TOML parsing error: {}", e)))
}

/// Get damping parameters for a specific method and variant.
///
/// # Arguments
///
/// - `method`: XC functional name (e.g., "b3lyp", "pbe0", "r2scan")
/// - `version`: DFT-D3 variant ("bj", "zero", "bjm", "zerom", "op", "cso")
///
/// # Returns
///
/// A `DFTD3DampingParam` containing the damping parameters and DOI reference.
pub fn get_damping_param(method: &str, version: &str) -> Result<DFTD3DampingParam, DFTD3Error> {
    let db = load_data_base()?;
    let method_lower = method.to_lowercase();
    let version_normalized = normalize_version(version);

    // Get method entry
    let method_entry = db.parameter.get(&method_lower).ok_or_else(|| {
        DFTD3Error::ParametersError(format!("Method '{}' not found in database", method))
    })?;

    // Get variant entry
    let (entry_raw, default_entry) = get_variant_entry(method_entry, &version_normalized, &db)?;

    // Merge with defaults
    let merged = merge_tables(&entry_raw, &default_entry);

    // Convert to public struct
    convert_to_damping_param(&merged, &version_normalized)
}

/// Get all damping parameters for all methods for a given variant.
///
/// # Arguments
///
/// - `version`: DFT-D3 variant ("bj", "zero", "bjm", "zerom", "op", "cso")
///
/// # Returns
///
/// A HashMap mapping method names to their damping parameters.
pub fn get_all_damping_params(
    version: &str,
) -> Result<HashMap<String, DFTD3DampingParam>, DFTD3Error> {
    let db = load_data_base()?;
    let version_normalized = normalize_version(version);
    let (_, default_entry) = get_variant_entry_for_defaults(&version_normalized, &db)?;

    let mut result = HashMap::new();
    for (method, method_entry) in &db.parameter {
        if let Ok((entry_raw, _)) = get_variant_entry(method_entry, &version_normalized, &db) {
            let merged = merge_tables(&entry_raw, &default_entry);
            if let Ok(param) = convert_to_damping_param(&merged, &version_normalized) {
                result.insert(method.clone(), param);
            }
        }
    }
    Ok(result)
}

/// List all available methods in the database.
pub fn list_methods() -> Vec<String> {
    let db = load_data_base().unwrap_or_else(|_| {
        // Return empty if parsing fails (shouldn't happen with embedded TOML)
        ParameterDataBase {
            default: DefaultSection {
                d3: vec!["bj".to_string(), "zero".to_string()],
                parameter: DefaultParameterSection {
                    d3: D3DefaultParams {
                        bj: Table::new(),
                        zero: Table::new(),
                        bjm: Table::new(),
                        zerom: Table::new(),
                        op: None,
                        cso: None,
                    },
                },
            },
            parameter: HashMap::new(),
        }
    });
    db.parameter.keys().cloned().collect()
}

/* #endregion */

/* #region Internal helper functions */

/// Normalize version string (handle aliases like "d3bj" -> "bj").
fn normalize_version(version: &str) -> String {
    let version_lower = version.to_lowercase().replace(['-', '_', ' '], "");
    // Handle aliases
    match version_lower.as_str() {
        "d3bj" | "bj" => "bj",
        "d3zero" | "zero" => "zero",
        "d3bjm" | "bjm" | "d3mbj" | "mbj" => "bjm",
        "d3zerom" | "zerom" | "d3mzero" | "mzero" => "zerom",
        "d3op" | "op" => "op",
        "d3cso" | "cso" => "cso",
        _ => &version_lower,
    }
    .to_string()
}

/// Get variant entry from method and database.
fn get_variant_entry(
    method_entry: &D3Variants,
    version: &str,
    db: &ParameterDataBase,
) -> Result<(Table, Table), DFTD3Error> {
    let d3_params = &method_entry.d3;
    let entry = match version {
        "bj" => d3_params.bj.clone(),
        "zero" => d3_params.zero.clone(),
        "bjm" => d3_params.bjm.clone(),
        "zerom" => d3_params.zerom.clone(),
        "op" => d3_params.op.clone(),
        "cso" => d3_params.cso.clone(),
        _ => None,
    };

    let entry = entry.ok_or_else(|| {
        DFTD3Error::ParametersError(format!("Variant '{}' not found for this method", version))
    })?;

    let default_entry = match version {
        "bj" => db.default.parameter.d3.bj.clone(),
        "zero" => db.default.parameter.d3.zero.clone(),
        "bjm" => db.default.parameter.d3.bjm.clone(),
        "zerom" => db.default.parameter.d3.zerom.clone(),
        "op" => db.default.parameter.d3.op.clone().unwrap_or_default(),
        "cso" => db.default.parameter.d3.cso.clone().unwrap_or_default(),
        _ => Table::new(),
    };

    Ok((entry, default_entry))
}

/// Get default entry for a variant.
fn get_variant_entry_for_defaults(
    version: &str,
    db: &ParameterDataBase,
) -> Result<(Option<Table>, Table), DFTD3Error> {
    let default_entry = match version {
        "bj" => db.default.parameter.d3.bj.clone(),
        "zero" => db.default.parameter.d3.zero.clone(),
        "bjm" => db.default.parameter.d3.bjm.clone(),
        "zerom" => db.default.parameter.d3.zerom.clone(),
        "op" => db.default.parameter.d3.op.clone().unwrap_or_default(),
        "cso" => db.default.parameter.d3.cso.clone().unwrap_or_default(),
        _ => {
            return Err(DFTD3Error::ParametersError(format!(
                "Variant '{}' not found in defaults",
                version
            )))
        },
    };
    Ok((None, default_entry))
}

/// Merge method-specific entry table with defaults table.
/// Method values override defaults.
fn merge_tables(entry: &Table, defaults: &Table) -> Table {
    let mut merged = defaults.clone();
    for (key, value) in entry {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

/// Extract DOI from merged table (DOI is method-specific, not from defaults).
fn extract_doi(table: &Table) -> Option<String> {
    table.get("doi").and_then(|v| v.as_str()).map(|s| s.to_string())
}

/// Convert merged TOML table directly to DFTD3DampingParam via serde.
#[cfg(feature = "api-v0_4")]
fn convert_to_damping_param(
    merged: &Table,
    version: &str,
) -> Result<DFTD3DampingParam, DFTD3Error> {
    let doi = extract_doi(merged);

    let param = match version {
        "bj" => {
            let param: DFTD3RationalDampingParam = deserialize_table(merged)?;
            DFTD3DampingParamEnum::Rational(param)
        },
        "zero" => {
            let param: DFTD3ZeroDampingParam = deserialize_table(merged)?;
            DFTD3DampingParamEnum::Zero(param)
        },
        "bjm" => {
            let param: DFTD3ModifiedRationalDampingParam = deserialize_table(merged)?;
            DFTD3DampingParamEnum::ModifiedRational(param)
        },
        "zerom" => {
            let param: DFTD3ModifiedZeroDampingParam = deserialize_table(merged)?;
            DFTD3DampingParamEnum::ModifiedZero(param)
        },
        #[cfg(feature = "api-v0_5")]
        "op" => {
            let param: DFTD3OptimizedPowerDampingParam = deserialize_table(merged)?;
            DFTD3DampingParamEnum::OptimizedPower(param)
        },
        #[cfg(feature = "api-v1_3")]
        "cso" => {
            let param: DFTD3CSODampingParam = deserialize_table(merged)?;
            DFTD3DampingParamEnum::CSO(param)
        },
        #[cfg(not(feature = "api-v0_5"))]
        "op" => {
            return Err(DFTD3Error::ParametersError(format!(
                "Variant '{}' requires api-v0_5 feature",
                version
            )))
        },
        #[cfg(not(feature = "api-v1_3"))]
        "cso" => {
            return Err(DFTD3Error::ParametersError(format!(
                "Variant '{}' requires api-v1_3 feature",
                version
            )))
        },
        _ => return Err(DFTD3Error::ParametersError(format!("Unknown variant: {}", version))),
    };

    Ok(DFTD3DampingParam { param, doi })
}

/// Deserialize a TOML table directly into a serde-deserializable type.
fn deserialize_table<T: for<'de> Deserialize<'de>>(table: &Table) -> Result<T, DFTD3Error> {
    // Use toml::Value::Table as deserializer via IntoDeserializer
    let value = toml::Value::Table(table.clone());
    T::deserialize(value.into_deserializer()).map_err(|e: toml::de::Error| {
        DFTD3Error::ParametersError(format!("Deserialization error: {}", e))
    })
}

// Non-feature-gated version that returns an error for unsupported variants
#[cfg(not(feature = "api-v0_4"))]
fn convert_to_damping_param(
    _merged: &Table,
    version: &str,
) -> Result<DFTD3DampingParam, DFTD3Error> {
    Err(DFTD3Error::ParametersError(format!(
        "Variant '{}' requires api-v0_4 feature or higher",
        version
    )))
}

/* #endregion */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data_base() {
        let db = load_data_base();
        match &db {
            Ok(db) => {
                assert!(db.parameter.contains_key("b3lyp"));
                assert!(db.parameter.contains_key("pbe"));
                assert!(db.parameter.contains_key("r2scan"));
            },
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("TOML parsing failed");
            },
        }
    }

    #[test]
    fn test_list_methods() {
        let methods = list_methods();
        assert!(methods.contains(&"b3lyp".to_string()));
        assert!(methods.contains(&"pbe0".to_string()));
        assert!(methods.len() > 100); // Should have many methods
    }

    #[cfg(feature = "api-v0_4")]
    #[test]
    fn test_get_b3lyp_bj() {
        let param = get_damping_param("b3lyp", "bj").unwrap();
        match param.param {
            DFTD3DampingParamEnum::Rational(data) => {
                assert_eq!(data.s6, 1.0);
                assert!((data.s8 - 1.9889).abs() < 1e-6);
                assert!((data.a1 - 0.3981).abs() < 1e-6);
                assert!((data.a2 - 4.4211).abs() < 1e-6);
                assert_eq!(data.alp, 14.0);
            },
            _ => panic!("Expected Rational variant"),
        }
        assert!(param.doi.is_some());
        assert_eq!(param.doi.unwrap(), "10.1002/jcc.21759");
    }

    #[cfg(feature = "api-v0_4")]
    #[test]
    fn test_get_pbe0_zero() {
        let param = get_damping_param("pbe0", "zero").unwrap();
        match param.param {
            DFTD3DampingParamEnum::Zero(data) => {
                assert_eq!(data.s6, 1.0);
                assert!((data.s8 - 0.928).abs() < 1e-6);
                assert!((data.rs6 - 1.287).abs() < 1e-6);
            },
            _ => panic!("Expected Zero variant"),
        }
    }

    #[cfg(feature = "api-v0_4")]
    #[test]
    fn test_get_r2scan_bj() {
        let param = get_damping_param("r2scan", "bj").unwrap();
        match param.param {
            DFTD3DampingParamEnum::Rational(data) => {
                assert!((data.s8 - 0.78981345).abs() < 1e-6);
                assert!((data.a1 - 0.49484001).abs() < 1e-6);
            },
            _ => panic!("Expected Rational variant"),
        }
        assert_eq!(param.doi.unwrap(), "10.1063/5.0041008");
    }

    #[cfg(feature = "api-v0_5")]
    #[test]
    fn test_get_b97d_op() {
        let param = get_damping_param("b97d", "op").unwrap();
        match param.param {
            DFTD3DampingParamEnum::OptimizedPower(data) => {
                assert_eq!(data.s6, 1.0);
                assert!((data.s8 - 1.46861).abs() < 1e-6);
                assert!((data.bet - 0.0).abs() < 1e-6);
            },
            _ => panic!("Expected OptimizedPower variant"),
        }
    }

    #[cfg(feature = "api-v1_3")]
    #[test]
    fn test_get_b3lyp_cso() {
        let param = get_damping_param("b3lyp", "cso").unwrap();
        match param.param {
            DFTD3DampingParamEnum::CSO(data) => {
                assert!((data.a1 - 0.86).abs() < 1e-6);
            },
            _ => panic!("Expected CSO variant"),
        }
    }

    #[cfg(feature = "api-v0_4")]
    #[test]
    fn test_method_not_found() {
        let result = get_damping_param("nonexistent", "bj");
        assert!(result.is_err());
        match &result.unwrap_err() {
            DFTD3Error::ParametersError(msg) => assert!(msg.contains("nonexistent")),
            _ => panic!("Expected ParametersError"),
        }
    }

    #[cfg(feature = "api-v0_4")]
    #[test]
    fn test_variant_not_found() {
        let result = get_damping_param("m05", "bj"); // m05 only has zero damping
        assert!(result.is_err());
    }

    #[cfg(feature = "api-v0_4")]
    #[test]
    fn test_get_all_damping_params() {
        let params = get_all_damping_params("bj").unwrap();
        assert!(params.contains_key("b3lyp"));
        assert!(params.contains_key("pbe"));
        assert!(params.len() > 50);
    }

    #[test]
    fn test_normalize_version() {
        assert_eq!(normalize_version("d3bj"), "bj");
        assert_eq!(normalize_version("d3zero"), "zero");
        assert_eq!(normalize_version("bj"), "bj");
        assert_eq!(normalize_version("d3op"), "op");
    }
}
