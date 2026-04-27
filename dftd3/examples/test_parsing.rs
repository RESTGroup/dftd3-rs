//! Test the parsing module functionality.
//!
//! Tests various ways to parse TOML/JSON input into DFTD3DampingParamEnum.

#![allow(clippy::excessive_precision, unused_imports)]

use approx::assert_abs_diff_eq;
use dftd3::prelude::*;

// --- Use case 1: Method lookup ---
#[test]
fn test_method_lookup() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "d3bj", method = "b3lyp"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.a1, 0.3981);
            assert_abs_diff_eq!(data.s8, 1.9889);
            assert_abs_diff_eq!(data.a2, 4.4211);
        },
        _ => panic!("Expected Rational variant"),
    }
    assert_eq!(param.doi.as_deref(), Some("10.1002/jcc.21759"));
}

// --- Use case 2: Version without d3 prefix ---
#[test]
fn test_version_without_prefix() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "bj", method = "b3lyp"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.a1, 0.3981);
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- Version is case-insensitive ---
#[test]
fn test_version_case_insensitive() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "BJ", method = "b3lyp"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.a1, 0.3981);
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- Use case 3: Direct parameters ---
#[test]
fn test_direct_params() {
    let param = dftd3_parse_damping_param_from_toml_f(
        r#"{version = "d3bj", a1 = 0.3981, s8 = 1.9889, a2 = 4.4211}"#,
    )
    .unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s6, 1.0);
            assert_abs_diff_eq!(data.a1, 0.3981);
            assert_abs_diff_eq!(data.s8, 1.9889);
            assert_abs_diff_eq!(data.a2, 4.4211);
            assert_abs_diff_eq!(data.alp, 14.0);
            assert_abs_diff_eq!(data.s9, 1.0);
        },
        _ => panic!("Expected Rational variant"),
    }
    assert!(param.doi.is_none());
}

// --- Use case 4: atm = false sets s9 = 0.0 ---
#[test]
fn test_atm_false() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "bj", method = "b3lyp", atm = false}"#)
            .unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.a1, 0.3981);
            assert_abs_diff_eq!(data.s9, 0.0);
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- atm = true is default (s9 = 1.0) ---
#[test]
fn test_atm_true_default() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "bj", method = "b3lyp"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s9, 1.0);
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- Use case 5: Method lookup with parameter override ---
#[test]
fn test_method_override() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "d3bj", method = "b3lyp", a1 = 0.5}"#)
            .unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.a1, 0.5);
            assert_abs_diff_eq!(data.s8, 1.9889); // unchanged from database
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- Use case 6: Direct parameters with atm = false ---
#[test]
fn test_direct_params_atm_false() {
    let param = dftd3_parse_damping_param_from_toml_f(
        r#"{version = "d3bj", a1 = 0.3981, s8 = 1.9889, a2 = 4.4211, atm = false}"#,
    )
    .unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s9, 0.0);
            assert_abs_diff_eq!(data.a1, 0.3981);
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- s9 takes precedence over atm when both specified ---
#[test]
fn test_s9_precedence_over_atm() {
    let param = dftd3_parse_damping_param_from_toml_f(
        r#"{version = "bj", method = "b3lyp", atm = false, s9 = 1.0}"#,
    )
    .unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s9, 1.0); // s9 takes precedence
        },
        _ => panic!("Expected Rational variant"),
    }
}

// --- Use case 7: Unknown field should raise error ---
#[test]
fn test_unknown_field_error() {
    let result =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "d3bj", method = "b3lyp", rs6 = 0.5}"#);
    assert!(result.is_err());
    match result.unwrap_err() {
        DFTD3Error::ParametersError(ref msg) => {
            assert!(msg.contains("rs6"), "Error should mention 'rs6': {msg}")
        },
        e => panic!("Expected ParametersError, got: {e:?}"),
    }
}

// --- Use case 8: Method name normalization (remove separators) ---
#[test]
fn test_method_name_normalization() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "zero", method = "m06-2x"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Zero(data) => {
            assert_abs_diff_eq!(data.rs6, 1.619);
            assert_abs_diff_eq!(data.s8, 0.0);
        },
        _ => panic!("Expected Zero variant"),
    }
}

#[test]
fn test_not_sufficient_parameters() {
    let result = dftd3_parse_damping_param_from_toml_f(r#"{version = "d3bj", a1 = 0.3981}"#);
    assert!(result.is_err());
    match result.unwrap_err() {
        DFTD3Error::ParametersError(ref msg) => {
            assert!(msg.contains("missing"), "Error should mention missing parameters: {msg}")
        },
        e => panic!("Expected ParametersError, got: {e:?}"),
    }
}

#[test]
fn test_method_name_with_underscores() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "bj", method = "r2_scan"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s8, 0.78981345);
        },
        _ => panic!("Expected Rational variant"),
    }
}

#[test]
fn test_missing_version_error() {
    let result = dftd3_parse_damping_param_from_toml_f(r#"{method = "b3lyp"}"#);
    assert!(result.is_err());
}

// --- dftd3_parse_damping_param_from_toml_f with standard TOML ---
#[test]
fn test_parse_from_toml_standard() {
    let input = r#"version = "bj"
method = "b3lyp""#;
    let param = dftd3_parse_damping_param_from_toml_f(input).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => assert_abs_diff_eq!(data.a1, 0.3981),
        _ => panic!("Expected Rational variant"),
    }
}

// --- JSON parsing (requires json feature) ---
#[cfg(feature = "json")]
#[test]
fn test_parse_from_json() {
    let input = r#"{"version": "bj", "method": "b3lyp"}"#;
    let param = dftd3_parse_damping_param_from_json(input).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => assert_abs_diff_eq!(data.a1, 0.3981),
        _ => panic!("Expected Rational variant"),
    }
}

#[cfg(feature = "json")]
#[test]
fn test_parse_from_json_atm() {
    let input = r#"{"version": "bj", "method": "b3lyp", "atm": false}"#;
    let param = dftd3_parse_damping_param_from_json(input).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => assert_abs_diff_eq!(data.s9, 0.0),
        _ => panic!("Expected Rational variant"),
    }
}

// --- OP variant (requires api-v0_5) ---
#[cfg(feature = "api-v0_5")]
#[test]
fn test_op_variant() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "op", method = "b97d"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::OptimizedPower(data) => {
            assert_abs_diff_eq!(data.s8, 1.46861);
            assert_abs_diff_eq!(data.bet, 0.0);
        },
        _ => panic!("Expected OptimizedPower variant"),
    }
}

#[cfg(feature = "api-v0_5")]
#[test]
fn test_op_atm_false() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "op", method = "b97d", atm = false}"#)
            .unwrap();
    match &param.param {
        DFTD3DampingParamEnum::OptimizedPower(data) => {
            assert_abs_diff_eq!(data.s9, 0.0);
            assert_abs_diff_eq!(data.s8, 1.46861);
        },
        _ => panic!("Expected OptimizedPower variant"),
    }
}

// --- CSO variant (requires api-v1_3) ---
#[cfg(feature = "api-v1_3")]
#[test]
fn test_cso_variant() {
    let param =
        dftd3_parse_damping_param_from_toml_f(r#"{version = "cso", method = "b3lyp"}"#).unwrap();
    match &param.param {
        DFTD3DampingParamEnum::CSO(data) => assert_abs_diff_eq!(data.a1, 0.86),
        _ => panic!("Expected CSO variant"),
    }
}

fn main() {
    println!("Run with: cargo test --example test_parsing --features=\"dynamic_loading\"");
}
