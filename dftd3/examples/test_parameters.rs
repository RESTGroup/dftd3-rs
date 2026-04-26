//! Test the parameters module functionality.
//!
//! Translated from s-dftd3's python/dftd3/test_parameters.py.

#![allow(clippy::excessive_precision)]

#[test]
fn test_list_methods() {
    let methods = list_methods();
    assert!(methods.contains(&"b3lyp".to_string()));
    assert!(methods.contains(&"pbe0".to_string()));
    assert!(methods.len() > 100);
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_get_b3lyp() {
    let param = get_damping_param("b3lyp", "bj").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s6, 1.0);
            assert_abs_diff_eq!(data.s9, 1.0);
            assert_abs_diff_eq!(data.alp, 14.0);
            assert_abs_diff_eq!(data.a1, 0.3981);
            assert_abs_diff_eq!(data.s8, 1.9889);
            assert_abs_diff_eq!(data.a2, 4.4211);
        },
        _ => panic!("Expected Rational variant for b3lyp/bj"),
    }
    assert_eq!(param.doi.as_deref(), Some("10.1002/jcc.21759"));
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_get_m11l() {
    let param = get_damping_param("m11l", "zero").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Zero(data) => {
            assert_abs_diff_eq!(data.s6, 1.0);
            assert_abs_diff_eq!(data.s9, 1.0);
            assert_abs_diff_eq!(data.alp, 14.0);
            assert_abs_diff_eq!(data.rs8, 1.0);
            assert_abs_diff_eq!(data.s8, 1.1129);
            assert_abs_diff_eq!(data.rs6, 2.3933);
        },
        _ => panic!("Expected Zero variant for m11l/zero"),
    }
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_get_pbe0_zero() {
    let param = get_damping_param("pbe0", "zero").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Zero(data) => {
            assert_abs_diff_eq!(data.s6, 1.0);
            assert_abs_diff_eq!(data.s8, 0.928);
            assert_abs_diff_eq!(data.rs6, 1.287);
        },
        _ => panic!("Expected Zero variant for pbe0/zero"),
    }
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_get_pw6b95() {
    let param = get_damping_param("pw6b95", "bj").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s6, 1.0);
            assert_abs_diff_eq!(data.s9, 1.0);
            assert_abs_diff_eq!(data.alp, 14.0);
            assert_abs_diff_eq!(data.a1, 0.2076);
            assert_abs_diff_eq!(data.s8, 0.7257);
            assert_abs_diff_eq!(data.a2, 6.3750);
        },
        _ => panic!("Expected Rational variant for pw6b95/bj"),
    }
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_get_r2scan_bj() {
    let param = get_damping_param("r2scan", "bj").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            assert_abs_diff_eq!(data.s8, 0.78981345);
            assert_abs_diff_eq!(data.a1, 0.49484001);
        },
        _ => panic!("Expected Rational variant for r2scan/bj"),
    }
    assert_eq!(param.doi.as_deref(), Some("10.1063/5.0041008"));
}

#[cfg(feature = "api-v0_5")]
#[test]
fn test_get_b97d_op() {
    let param = get_damping_param("b97d", "op").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::OptimizedPower(data) => {
            assert_abs_diff_eq!(data.s6, 1.0);
            assert_abs_diff_eq!(data.s8, 1.46861);
            assert_abs_diff_eq!(data.bet, 0.0);
        },
        _ => panic!("Expected OptimizedPower variant for b97d/op"),
    }
}

#[cfg(feature = "api-v1_3")]
#[test]
fn test_get_b3lyp_cso() {
    let param = get_damping_param("b3lyp", "cso").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::CSO(data) => {
            assert_abs_diff_eq!(data.a1, 0.86);
        },
        _ => panic!("Expected CSO variant for b3lyp/cso"),
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
    let result = get_damping_param("m05", "bj");
    assert!(result.is_err());
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_all_parameters() {
    let params = get_all_damping_params("bj").unwrap();
    assert!(params.contains_key("b3lyp"));
    assert!(params.contains_key("b2plyp"));
    assert!(params.contains_key("pw6b95"));
    assert!(params.len() > 50);
}

fn main() {
    println!("Run with: cargo test --example test_parameters");
}
