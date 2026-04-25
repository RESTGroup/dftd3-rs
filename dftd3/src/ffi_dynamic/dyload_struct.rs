//! Library struct definition for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime. Runtime panic occurs if a function
//! is not found in the loaded library.

use super::*;

pub struct DyLoadLib {
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub __error: Option<String>,
    pub dftd3_get_version: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub dftd3_new_error: Option<unsafe extern "C" fn() -> dftd3_error>,
    pub dftd3_check_error: Option<unsafe extern "C" fn(arg1: dftd3_error) -> ::core::ffi::c_int>,
    pub dftd3_get_error: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: *const ::core::ffi::c_int,
        ),
    >,
    pub dftd3_delete_error: Option<unsafe extern "C" fn(arg1: *mut dftd3_error)>,
    pub dftd3_new_structure: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: ::core::ffi::c_int,
            arg3: *const ::core::ffi::c_int,
            arg4: *const f64,
            arg5: *const f64,
            arg6: *const bool,
        ) -> dftd3_structure,
    >,
    pub dftd3_delete_structure: Option<unsafe extern "C" fn(arg1: *mut dftd3_structure)>,
    pub dftd3_update_structure: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: dftd3_structure,
            arg3: *const f64,
            arg4: *const f64,
        ),
    >,
    pub dftd3_new_d3_model:
        Option<unsafe extern "C" fn(arg1: dftd3_error, arg2: dftd3_structure) -> dftd3_model>,
    pub dftd3_set_model_realspace_cutoff: Option<
        unsafe extern "C" fn(arg1: dftd3_error, arg2: dftd3_model, arg3: f64, arg4: f64, arg5: f64),
    >,
    pub dftd3_delete_model: Option<unsafe extern "C" fn(arg1: *mut dftd3_model)>,
    pub dftd3_new_zero_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
        ) -> dftd3_param,
    >,
    pub dftd3_load_zero_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: bool,
        ) -> dftd3_param,
    >,
    pub dftd3_new_rational_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
        ) -> dftd3_param,
    >,
    pub dftd3_load_rational_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: bool,
        ) -> dftd3_param,
    >,
    pub dftd3_new_mzero_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
            arg8: f64,
        ) -> dftd3_param,
    >,
    pub dftd3_load_mzero_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: bool,
        ) -> dftd3_param,
    >,
    pub dftd3_new_mrational_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
        ) -> dftd3_param,
    >,
    pub dftd3_load_mrational_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: bool,
        ) -> dftd3_param,
    >,
    pub dftd3_new_optimizedpower_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
            arg8: f64,
        ) -> dftd3_param,
    >,
    pub dftd3_load_optimizedpower_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: bool,
        ) -> dftd3_param,
    >,
    pub dftd3_new_cso_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
            arg8: f64,
        ) -> dftd3_param,
    >,
    pub dftd3_load_cso_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: *mut ::core::ffi::c_char,
            arg3: bool,
        ) -> dftd3_param,
    >,
    pub dftd3_delete_param: Option<unsafe extern "C" fn(arg1: *mut dftd3_param)>,
    pub dftd3_load_gcp_param: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: dftd3_structure,
            arg3: *mut ::core::ffi::c_char,
            arg4: *mut ::core::ffi::c_char,
        ) -> dftd3_gcp,
    >,
    pub dftd3_set_gcp_realspace_cutoff:
        Option<unsafe extern "C" fn(arg1: dftd3_error, arg2: dftd3_gcp, arg3: f64, arg4: f64)>,
    pub dftd3_delete_gcp: Option<unsafe extern "C" fn(arg1: *mut dftd3_gcp)>,
    pub dftd3_get_dispersion: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: dftd3_structure,
            arg3: dftd3_model,
            arg4: dftd3_param,
            arg5: *mut f64,
            arg6: *mut f64,
            arg7: *mut f64,
        ),
    >,
    pub dftd3_get_pairwise_dispersion: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: dftd3_structure,
            arg3: dftd3_model,
            arg4: dftd3_param,
            arg5: *mut f64,
            arg6: *mut f64,
        ),
    >,
    pub dftd3_get_counterpoise: Option<
        unsafe extern "C" fn(
            arg1: dftd3_error,
            arg2: dftd3_structure,
            arg3: dftd3_gcp,
            arg4: *mut f64,
            arg5: *mut f64,
            arg6: *mut f64,
        ),
    >,
}
