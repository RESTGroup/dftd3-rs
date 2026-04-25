//! Test the parameters module functionality.
//!
//! This example demonstrates the usage of the parameters module
//! and verifies that the Rust implementation produces correct results.

use dftd3::parameters::{
    get_all_damping_params, get_damping_param, list_methods, DFTD3DampingParamEnum,
};

fn main() {
    println!("=== Testing parameters module ===\n");

    // Test list_methods
    println!("Testing list_methods...");
    let methods = list_methods();
    println!("Found {} methods", methods.len());
    assert!(methods.contains(&"b3lyp".to_string()));
    assert!(methods.contains(&"pbe0".to_string()));
    println!("✓ list_methods works correctly\n");

    // Test B3LYP-D3(BJ)
    println!("Testing get_damping_param for B3LYP-D3(BJ)...");
    let param = get_damping_param("b3lyp", "bj").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            println!("  s6: {}, s8: {}, a1: {}, a2: {}", data.s6, data.s8, data.a1, data.a2);
            assert_eq!(data.s6, 1.0);
            assert!((data.s8 - 1.9889).abs() < 1e-6);
            assert!((data.a1 - 0.3981).abs() < 1e-6);
            assert!((data.a2 - 4.4211).abs() < 1e-6);
        },
        _ => panic!("Expected Rational variant"),
    }
    println!("  DOI: {}", param.doi.clone().unwrap_or_else(|| "N/A".to_string()));
    println!("✓ B3LYP-D3(BJ) parameters verified\n");

    // Test r2scan
    println!("Testing get_damping_param for r2scan-D3(BJ)...");
    let param = get_damping_param("r2scan", "bj").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Rational(data) => {
            println!("  s6: {}, s8: {}, a1: {}, a2: {}", data.s6, data.s8, data.a1, data.a2);
            assert!((data.s8 - 0.78981345).abs() < 1e-6);
        },
        _ => panic!("Expected Rational variant"),
    }
    println!("  DOI: {}", param.doi.clone().unwrap_or_else(|| "N/A".to_string()));
    println!("✓ r2scan-D3(BJ) parameters verified\n");

    // Test PBE0-D3(zero)
    println!("Testing get_damping_param for PBE0-D3(zero)...");
    let param = get_damping_param("pbe0", "zero").unwrap();
    match &param.param {
        DFTD3DampingParamEnum::Zero(data) => {
            println!("  s6: {}, s8: {}, rs6: {}", data.s6, data.s8, data.rs6);
            assert!((data.rs6 - 1.287).abs() < 1e-6);
        },
        _ => panic!("Expected Zero variant"),
    }
    println!("✓ PBE0-D3(zero) parameters verified\n");

    // Test case-insensitive
    println!("Testing case-insensitive lookup...");
    let param_lower = get_damping_param("b3lyp", "bj").unwrap();
    let param_upper = get_damping_param("B3LYP", "BJ").unwrap();
    assert_eq!(param_lower.param.s6(), param_upper.param.s6());
    println!("✓ Case-insensitive lookup works\n");

    // Test all_parameters
    println!("Testing get_all_damping_params...");
    let params = get_all_damping_params("bj").unwrap();
    println!("Found {} methods with BJ parameters", params.len());
    assert!(params.contains_key(&"b3lyp".to_string()));
    assert!(params.contains_key(&"pbe".to_string()));
    println!("✓ get_all_damping_params works\n");

    println!("=== All tests passed ===");
}
