//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime.

use super::*;

pub unsafe fn dftd3_get_version() -> ::core::ffi::c_int {
    dyload_lib().dftd3_get_version.unwrap()()
}

pub unsafe fn dftd3_new_error() -> dftd3_error {
    dyload_lib().dftd3_new_error.unwrap()()
}

pub unsafe fn dftd3_check_error(arg1: dftd3_error) -> ::core::ffi::c_int {
    dyload_lib().dftd3_check_error.unwrap()(arg1)
}

pub unsafe fn dftd3_get_error(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: *const ::core::ffi::c_int,
) {
    dyload_lib().dftd3_get_error.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_delete_error(arg1: *mut dftd3_error) {
    dyload_lib().dftd3_delete_error.unwrap()(arg1)
}

pub unsafe fn dftd3_new_structure(
    arg1: dftd3_error,
    arg2: ::core::ffi::c_int,
    arg3: *const ::core::ffi::c_int,
    arg4: *const f64,
    arg5: *const f64,
    arg6: *const bool,
) -> dftd3_structure {
    dyload_lib().dftd3_new_structure.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dftd3_delete_structure(arg1: *mut dftd3_structure) {
    dyload_lib().dftd3_delete_structure.unwrap()(arg1)
}

pub unsafe fn dftd3_update_structure(
    arg1: dftd3_error,
    arg2: dftd3_structure,
    arg3: *const f64,
    arg4: *const f64,
) {
    dyload_lib().dftd3_update_structure.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dftd3_new_d3_model(arg1: dftd3_error, arg2: dftd3_structure) -> dftd3_model {
    dyload_lib().dftd3_new_d3_model.unwrap()(arg1, arg2)
}

pub unsafe fn dftd3_set_model_realspace_cutoff(
    arg1: dftd3_error,
    arg2: dftd3_model,
    arg3: f64,
    arg4: f64,
    arg5: f64,
) {
    dyload_lib().dftd3_set_model_realspace_cutoff.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dftd3_delete_model(arg1: *mut dftd3_model) {
    dyload_lib().dftd3_delete_model.unwrap()(arg1)
}

pub unsafe fn dftd3_new_zero_damping(
    arg1: dftd3_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
) -> dftd3_param {
    dyload_lib().dftd3_new_zero_damping.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd3_load_zero_damping(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: bool,
) -> dftd3_param {
    dyload_lib().dftd3_load_zero_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_new_rational_damping(
    arg1: dftd3_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
) -> dftd3_param {
    dyload_lib().dftd3_new_rational_damping.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd3_load_rational_damping(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: bool,
) -> dftd3_param {
    dyload_lib().dftd3_load_rational_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_new_mzero_damping(
    arg1: dftd3_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
    arg8: f64,
) -> dftd3_param {
    dyload_lib().dftd3_new_mzero_damping.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dftd3_load_mzero_damping(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: bool,
) -> dftd3_param {
    dyload_lib().dftd3_load_mzero_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_new_mrational_damping(
    arg1: dftd3_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
) -> dftd3_param {
    dyload_lib().dftd3_new_mrational_damping.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd3_load_mrational_damping(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: bool,
) -> dftd3_param {
    dyload_lib().dftd3_load_mrational_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_new_optimizedpower_damping(
    arg1: dftd3_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
    arg8: f64,
) -> dftd3_param {
    dyload_lib().dftd3_new_optimizedpower_damping.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8,
    )
}

pub unsafe fn dftd3_load_optimizedpower_damping(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: bool,
) -> dftd3_param {
    dyload_lib().dftd3_load_optimizedpower_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_new_cso_damping(
    arg1: dftd3_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
    arg8: f64,
) -> dftd3_param {
    dyload_lib().dftd3_new_cso_damping.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dftd3_load_cso_damping(
    arg1: dftd3_error,
    arg2: *mut ::core::ffi::c_char,
    arg3: bool,
) -> dftd3_param {
    dyload_lib().dftd3_load_cso_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd3_delete_param(arg1: *mut dftd3_param) {
    dyload_lib().dftd3_delete_param.unwrap()(arg1)
}

pub unsafe fn dftd3_load_gcp_param(
    arg1: dftd3_error,
    arg2: dftd3_structure,
    arg3: *mut ::core::ffi::c_char,
    arg4: *mut ::core::ffi::c_char,
) -> dftd3_gcp {
    dyload_lib().dftd3_load_gcp_param.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dftd3_set_gcp_realspace_cutoff(
    arg1: dftd3_error,
    arg2: dftd3_gcp,
    arg3: f64,
    arg4: f64,
) {
    dyload_lib().dftd3_set_gcp_realspace_cutoff.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dftd3_delete_gcp(arg1: *mut dftd3_gcp) {
    dyload_lib().dftd3_delete_gcp.unwrap()(arg1)
}

pub unsafe fn dftd3_get_dispersion(
    arg1: dftd3_error,
    arg2: dftd3_structure,
    arg3: dftd3_model,
    arg4: dftd3_param,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut f64,
) {
    dyload_lib().dftd3_get_dispersion.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd3_get_pairwise_dispersion(
    arg1: dftd3_error,
    arg2: dftd3_structure,
    arg3: dftd3_model,
    arg4: dftd3_param,
    arg5: *mut f64,
    arg6: *mut f64,
) {
    dyload_lib().dftd3_get_pairwise_dispersion.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dftd3_get_counterpoise(
    arg1: dftd3_error,
    arg2: dftd3_structure,
    arg3: dftd3_gcp,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut f64,
) {
    dyload_lib().dftd3_get_counterpoise.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}
