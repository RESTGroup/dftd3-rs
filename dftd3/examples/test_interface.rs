//! This file is actually testing the interface of the library.
//!
//! However, perhaps some functions can be captured in documentation.
//! So this file is here.
#![allow(clippy::excessive_precision)]

use approx::assert_abs_diff_eq;
use dftd3::prelude::*;
use rstest::{fixture, rstest};

// Fixtures
#[fixture]
fn numbers() -> Vec<usize> {
    vec![6, 7, 6, 7, 6, 6, 6, 8, 7, 6, 8, 7, 6, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
}

#[fixture]
fn positions() -> Vec<f64> {
    #[rustfmt::skip]
    let positions = vec![
        // Coordinates in Bohr
         2.02799738646442,  0.09231312124713, -0.14310895950963,
         4.75011007621000,  0.02373496014051, -0.14324124033844,
         6.33434307654413,  2.07098865582721, -0.14235306905930,
         8.72860718071825,  1.38002919517619, -0.14265542523943,
         8.65318821103610, -1.19324866489847, -0.14231527453678,
         6.23857175648671, -2.08353643730276, -0.14218299370797,
         5.63266886875962, -4.69950321056008, -0.13940509630299,
         3.44931709749015, -5.48092386085491, -0.14318454855466,
         7.77508917214346, -6.24427872938674, -0.13107140408805,
        10.30229550927022, -5.39739796609292, -0.13672168520430,
        12.07410272485492, -6.91573621641911, -0.13666499342053,
        10.70038521493902, -2.79078533715849, -0.14148379504141,
        13.24597858727017, -1.76969072232377, -0.14218299370797,
         7.40891694074004, -8.95905928176407, -0.11636933482904,
         1.38702118184179,  2.05575746325296, -0.14178615122154,
         1.34622199478497, -0.86356704498496,  1.55590600570783,
         1.34624089204623, -0.86133716815647, -1.84340893849267,
         5.65596919189118,  4.00172183859480, -0.14131371969009,
        14.67430918222276, -3.26230980007732, -0.14344911021228,
        13.50897177220290, -0.60815166181684,  1.54898960808727,
        13.50780014200488, -0.60614855212345, -1.83214617078268,
         5.41408424778406, -9.49239668625902, -0.11022772492007,
         8.31919801555568, -9.74947502841788,  1.56539243085954,
         8.31511620712388, -9.76854236502758, -1.79108242206824,
    ];
    positions
}

#[fixture]
fn model() -> DFTD3Model {
    let numbers = vec![1, 1, 6, 5, 1, 15, 8, 17, 13, 15, 5, 1, 9, 15, 1, 15];
    #[rustfmt::skip]
    let positions = vec![
        // Coordinates in Bohr
         2.79274810283778,  3.82998228828316, -2.79287054959216,
        -1.43447454186833,  0.43418729987882,  5.53854345129809,
        -3.26268343665218, -2.50644032426151, -1.56631149351046,
         2.14548759959147, -0.88798018953965, -2.24592534506187,
        -4.30233097423181, -3.93631518670031, -0.48930754109119,
         0.06107643564880, -3.82467931731366, -2.22333344469482,
         0.41168550401858,  0.58105573172764,  5.56854609916143,
         4.41363836635653,  3.92515871809283,  2.57961724984000,
         1.33707758998700,  1.40194471661647,  1.97530004949523,
         3.08342709834868,  1.72520024666801, -4.42666116106828,
        -3.02346932078505,  0.04438199934191, -0.27636197425010,
         1.11508390868455, -0.97617412809198,  6.25462847718180,
         0.61938955433011,  2.17903547389232, -6.21279842416963,
        -2.67491681346835,  3.00175899761859,  1.05038813614845,
        -4.13181080289514, -2.34226739863660, -3.44356159392859,
         2.85007173009739, -2.64884892757600,  0.71010806424206,
    ];
    DFTD3Model::new(&numbers, &positions, None, None)
}

// Tests for parameter constructors
#[cfg(feature = "api-v0_4")]
#[test]
fn test_rational_damping_noargs() {
    // Check constructor of damping parameters for insufficient arguments
    assert!(DFTD3RationalDampingParamBuilder::default().build().is_err());

    let builder = DFTD3RationalDampingParamBuilder::default().a1(0.4).a2(5.0);
    assert!(builder.build().is_err());

    let builder = DFTD3RationalDampingParamBuilder::default().s8(1.0).a2(5.0);
    assert!(builder.build().is_err());

    let builder = DFTD3RationalDampingParamBuilder::default().s8(1.0).a1(0.4);
    assert!(builder.build().is_err());
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_zero_damping_noargs() {
    // Check constructor of damping parameters for insufficient arguments
    assert!(DFTD3ZeroDampingParamBuilder::default().build().is_err());

    let builder = DFTD3ZeroDampingParamBuilder::default().rs6(1.2);
    assert!(builder.build().is_err());

    let builder = DFTD3ZeroDampingParamBuilder::default().s8(1.0);
    assert!(builder.build().is_err());
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_modified_rational_damping_noargs() {
    // Check constructor of damping parameters for insufficient arguments
    assert!(DFTD3ModifiedRationalDampingParamBuilder::default().build().is_err());

    let builder = DFTD3ModifiedRationalDampingParamBuilder::default().a1(0.4).a2(5.0);
    assert!(builder.build().is_err());

    let builder = DFTD3ModifiedRationalDampingParamBuilder::default().s8(1.0).a2(5.0);
    assert!(builder.build().is_err());

    let builder = DFTD3ModifiedRationalDampingParamBuilder::default().s8(1.0).a1(0.4);
    assert!(builder.build().is_err());
}

#[cfg(feature = "api-v0_4")]
#[test]
fn test_modified_zero_damping_noargs() {
    // Check constructor of damping parameters for insufficient arguments
    assert!(DFTD3ModifiedZeroDampingParamBuilder::default().build().is_err());

    let builder = DFTD3ModifiedZeroDampingParamBuilder::default().rs6(1.2).bet(1.0);
    assert!(builder.build().is_err());

    let builder = DFTD3ModifiedZeroDampingParamBuilder::default().s8(1.0).bet(1.0);
    assert!(builder.build().is_err());

    let builder = DFTD3ModifiedZeroDampingParamBuilder::default().s8(1.0).rs6(1.2);
    assert!(builder.build().is_err());
}

#[cfg(feature = "api-v0_5")]
#[test]
fn test_optimized_power_damping_noargs() {
    // Check constructor of damping parameters for insufficient arguments
    assert!(DFTD3OptimizedPowerDampingParamBuilder::default().build().is_err());

    let builder = DFTD3OptimizedPowerDampingParamBuilder::default().a1(0.3).a2(4.2).bet(1.0);
    assert!(builder.build().is_err());

    let builder = DFTD3OptimizedPowerDampingParamBuilder::default().s8(1.0).a2(4.2).bet(1.0);
    assert!(builder.build().is_err());

    let builder = DFTD3OptimizedPowerDampingParamBuilder::default().s8(1.0).a1(0.3).bet(1.0);
    assert!(builder.build().is_err());

    let builder = DFTD3OptimizedPowerDampingParamBuilder::default().s8(1.0).a1(0.3).a2(4.2);
    assert!(builder.build().is_err());
}

// Structure tests
#[rstest]
fn test_structure(numbers: Vec<usize>, positions: Vec<f64>) {
    // check if the molecular structure data is working as expected.

    // Constructor should raise an error for nuclear fusion input
    let zero_positions = vec![0.0; 24 * 3];
    assert!(DFTD3Model::new_f(&numbers, &zero_positions, None, None).is_err());

    // The Rust struct should protect from garbage input like this
    let bad_numbers = vec![1, 1, 1];
    assert!(DFTD3Model::new_f(&bad_numbers, &positions, None, None).is_err());

    // Also check for sane coordinate input
    let bad_positions = vec![0.0; 7];
    assert!(DFTD3Model::new_f(&numbers, &bad_positions, None, None).is_err());

    // Construct real molecule
    let mut inst = DFTD3Model::new(&numbers, &positions, None, None);

    // Try to update a structure with missmatched coordinates
    let bad_update_positions = vec![0.0; 7];
    assert!(inst.update_f(&bad_update_positions, None).is_err());

    // Try to add a missmatched lattice
    let bad_lattice = vec![0.0; 7];
    assert!(inst.update_f(&positions, Some(&bad_lattice)).is_err());

    // Try to update a structure with nuclear fusion coordinates
    let zero_update_positions = vec![0.0; numbers.len() * 3];
    assert!(inst.update_f(&zero_update_positions, None).is_err());
}

// D3 tests
#[cfg(feature = "api-v0_4")]
#[rstest]
#[case(true, -0.029489232932494884)]
#[case(false, -0.029589132634178342)]
fn test_pbe0_d3_bj(model: DFTD3Model, #[case] atm: bool, #[case] expected: f64) {
    let param = DFTD3RationalDampingParam::load_param("pbe0", atm);
    let res = model.get_dispersion(&param, false);
    assert_abs_diff_eq!(res.energy, expected, epsilon = 1e-8);
}

#[cfg(feature = "api-v0_4")]
#[rstest]
#[case(true, -0.022714272555175656)]
#[case(false, -0.022814172019166058)]
fn test_b3lyp_d3_zero(model: DFTD3Model, #[case] atm: bool, #[case] expected: f64) {
    let param = DFTD3ZeroDampingParam::load_param("b3lyp", atm);
    let res = model.get_dispersion(&param, false);
    assert_abs_diff_eq!(res.energy, expected, epsilon = 1e-8);
}

#[cfg(feature = "api-v0_4")]
#[rstest]
#[case(true, -0.06327406660942464)]
#[case(false, -0.06337396631110809)]
fn test_pbe_d3_bjm(model: DFTD3Model, #[case] atm: bool, #[case] expected: f64) {
    let param = DFTD3ModifiedRationalDampingParam::load_param("pbe", atm);
    let res = model.get_dispersion(&param, false);
    assert_abs_diff_eq!(res.energy, expected, epsilon = 1e-8);
}

#[cfg(feature = "api-v0_4")]
#[rstest]
#[case(true, -0.026013316869036292)]
#[case(false, -0.026113216333026695)]
fn test_bp_d3_zerom(model: DFTD3Model, #[case] atm: bool, #[case] expected: f64) {
    let param = DFTD3ModifiedZeroDampingParam::load_param("bp", atm);
    let res = model.get_dispersion(&param, false);
    assert_abs_diff_eq!(res.energy, expected, epsilon = 1e-8);
}

#[cfg(feature = "api-v0_5")]
#[rstest]
#[case(true, -0.07681029606751344)]
#[case(false, -0.07691018779028679)]
fn test_b97d_d3_op(model: DFTD3Model, #[case] atm: bool, #[case] expected: f64) {
    let param = DFTD3OptimizedPowerDampingParam::load_param("b97d", atm);
    let res = model.get_dispersion(&param, false);
    assert_abs_diff_eq!(res.energy, expected, epsilon = 1e-8);
}

// GCP tests
#[rstest]
#[ignore = "seems dealloc bug in simple-dftd3's delete_gcp_api function"]
#[cfg(feature = "gcp")]
fn test_gcp_empty(numbers: Vec<usize>, positions: Vec<f64>) {
    let gcp = DFTD3GCP::new(&numbers, &positions, None, None, "", "");
    let res = gcp.get_counterpoise(false);
    assert_abs_diff_eq!(res.energy, 0.0, epsilon = 1e-8);
}

#[rstest]
// test ignored due to upstream bug in simple-dftd3's delete_gcp_api function which incorrectly uses
// vp_error type instead of vp_gcp type, causing segfault when deallocating complex parameter sets
// with allocated xv/emiss/slater arrays.
#[cfg(feature = "gcp")]
#[ignore = "seems dealloc bug in simple-dftd3's delete_gcp_api function"]
#[case("b973c", -0.07653225860427701)]
#[ignore = "seems dealloc bug in simple-dftd3's delete_gcp_api function"]
#[case("pbeh3c", 0.04977771585466725)]
fn test_gcp_3c(
    numbers: Vec<usize>,
    positions: Vec<f64>,
    #[case] method: &str,
    #[case] expected: f64,
) {
    let gcp = DFTD3GCP::new(&numbers, &positions, None, None, method, "");
    let res = gcp.get_counterpoise(false);
    println!("GCP energy for method {}: {}", method, res.energy);
    assert_abs_diff_eq!(res.energy, expected, epsilon = 1e-8);
}

#[cfg(feature = "api-v0_5")]
fn test_pair_resolved() {
    let thr = 1.0e-8;

    let numbers = vec![16, 16, 16, 16, 16, 16, 16, 16];
    #[rustfmt::skip]
    let positions = vec![
        -4.15128787379191,  1.71951973863958, -0.93066267097296,
        -4.15128787379191, -1.71951973863958,  0.93066267097296,
        -1.71951973863958, -4.15128787379191, -0.93066267097296,
         1.71951973863958, -4.15128787379191,  0.93066267097296,
         4.15128787379191, -1.71951973863958, -0.93066267097296,
         4.15128787379191,  1.71951973863958,  0.93066267097296,
         1.71951973863958,  4.15128787379191, -0.93066267097296,
        -1.71951973863958,  4.15128787379191,  0.93066267097296,
    ];

    #[rustfmt::skip]
    let pair_disp2 = [
        [-0.00000000, -0.00153111, -0.00108052, -0.00032865, -0.00023796, -0.00032865, -0.00108052, -0.00153111],
        [-0.00153111, -0.00000000, -0.00153111, -0.00108052, -0.00032865, -0.00023796, -0.00032865, -0.00108052],
        [-0.00108052, -0.00153111, -0.00000000, -0.00153111, -0.00108052, -0.00032865, -0.00023796, -0.00032865],
        [-0.00032865, -0.00108052, -0.00153111, -0.00000000, -0.00153111, -0.00108052, -0.00032865, -0.00023796],
        [-0.00023796, -0.00032865, -0.00108052, -0.00153111, -0.00000000, -0.00153111, -0.00108052, -0.00032865],
        [-0.00032865, -0.00023796, -0.00032865, -0.00108052, -0.00153111, -0.00000000, -0.00153111, -0.00108052],
        [-0.00108052, -0.00032865, -0.00023796, -0.00032865, -0.00108052, -0.00153111, -0.00000000, -0.00153111],
        [-0.00153111, -0.00108052, -0.00032865, -0.00023796, -0.00032865, -0.00108052, -0.00153111, -0.00000000],
    ];

    #[rustfmt::skip]
    let pair_disp3 = [
        [0.00000000e-00, 1.08616452e-07, 2.91526483e-07, 3.95872130e-07, 3.18133443e-07, 3.95872130e-07, 2.91526483e-07, 1.08616452e-07],
        [1.08616452e-07, 0.00000000e-00, 1.08616452e-07, 2.91526483e-07, 3.95872130e-07, 3.18133443e-07, 3.95872130e-07, 2.91526483e-07],
        [2.91526483e-07, 1.08616452e-07, 0.00000000e-00, 1.08616452e-07, 2.91526483e-07, 3.95872130e-07, 3.18133443e-07, 3.95872130e-07],
        [3.95872130e-07, 2.91526483e-07, 1.08616452e-07, 0.00000000e-00, 1.08616452e-07, 2.91526483e-07, 3.95872130e-07, 3.18133443e-07],
        [3.18133443e-07, 3.95872130e-07, 2.91526483e-07, 1.08616452e-07, 0.00000000e-00, 1.08616452e-07, 2.91526483e-07, 3.95872130e-07],
        [3.95872130e-07, 3.18133443e-07, 3.95872130e-07, 2.91526483e-07, 1.08616452e-07, 0.00000000e-00, 1.08616452e-07, 2.91526483e-07],
        [2.91526483e-07, 3.95872130e-07, 3.18133443e-07, 3.95872130e-07, 2.91526483e-07, 1.08616452e-07, 0.00000000e-00, 1.08616452e-07],
        [1.08616452e-07, 2.91526483e-07, 3.95872130e-07, 3.18133443e-07, 3.95872130e-07, 2.91526483e-07, 1.08616452e-07, 0.00000000e-00],
    ];

    let model = DFTD3Model::new(&numbers, &positions, None, None);
    let param = DFTD3RationalDampingParamBuilder::default()
        .a1(0.5545)
        .s8(2.2609)
        .a2(3.2297)
        .build()
        .unwrap()
        .new_param();

    let res = model.get_pairwise_dispersion(&param);

    // Flatten the expected matrices for comparison
    let expected_pairs2: Vec<f64> = pair_disp2.into_iter().flatten().collect();
    let expected_pairs3: Vec<f64> = pair_disp3.into_iter().flatten().collect();

    res.pair_energy2.iter().zip(expected_pairs2.iter()).for_each(|(x, y)| {
        assert_abs_diff_eq!(x, y, epsilon = thr);
    });
    res.pair_energy3.iter().zip(expected_pairs3.iter()).for_each(|(x, y)| {
        assert_abs_diff_eq!(x, y, epsilon = thr);
    });
}

#[cfg(feature = "api-v0_5")]
fn main() {
    test_pair_resolved();
}

#[cfg(feature = "api-v0_5")]
#[test]
fn test() {
    test_pair_resolved();
}
