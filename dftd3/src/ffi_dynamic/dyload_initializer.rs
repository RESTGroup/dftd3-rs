//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{Library, Symbol};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}

impl DyLoadLib {
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {
        let mut result = DyLoadLib {
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
            dftd3_get_version: get_symbol(&libs, b"dftd3_get_version\0").map(|sym| *sym),
            dftd3_new_error: get_symbol(&libs, b"dftd3_new_error\0").map(|sym| *sym),
            dftd3_check_error: get_symbol(&libs, b"dftd3_check_error\0").map(|sym| *sym),
            dftd3_get_error: get_symbol(&libs, b"dftd3_get_error\0").map(|sym| *sym),
            dftd3_delete_error: get_symbol(&libs, b"dftd3_delete_error\0").map(|sym| *sym),
            dftd3_new_structure: get_symbol(&libs, b"dftd3_new_structure\0").map(|sym| *sym),
            dftd3_delete_structure: get_symbol(&libs, b"dftd3_delete_structure\0").map(|sym| *sym),
            dftd3_update_structure: get_symbol(&libs, b"dftd3_update_structure\0").map(|sym| *sym),
            dftd3_new_d3_model: get_symbol(&libs, b"dftd3_new_d3_model\0").map(|sym| *sym),
            dftd3_set_model_realspace_cutoff: get_symbol(
                &libs,
                b"dftd3_set_model_realspace_cutoff\0",
            )
            .map(|sym| *sym),
            dftd3_delete_model: get_symbol(&libs, b"dftd3_delete_model\0").map(|sym| *sym),
            dftd3_new_zero_damping: get_symbol(&libs, b"dftd3_new_zero_damping\0").map(|sym| *sym),
            dftd3_load_zero_damping: get_symbol(&libs, b"dftd3_load_zero_damping\0")
                .map(|sym| *sym),
            dftd3_new_rational_damping: get_symbol(&libs, b"dftd3_new_rational_damping\0")
                .map(|sym| *sym),
            dftd3_load_rational_damping: get_symbol(&libs, b"dftd3_load_rational_damping\0")
                .map(|sym| *sym),
            dftd3_new_mzero_damping: get_symbol(&libs, b"dftd3_new_mzero_damping\0")
                .map(|sym| *sym),
            dftd3_load_mzero_damping: get_symbol(&libs, b"dftd3_load_mzero_damping\0")
                .map(|sym| *sym),
            dftd3_new_mrational_damping: get_symbol(&libs, b"dftd3_new_mrational_damping\0")
                .map(|sym| *sym),
            dftd3_load_mrational_damping: get_symbol(&libs, b"dftd3_load_mrational_damping\0")
                .map(|sym| *sym),
            dftd3_new_optimizedpower_damping: get_symbol(
                &libs,
                b"dftd3_new_optimizedpower_damping\0",
            )
            .map(|sym| *sym),
            dftd3_load_optimizedpower_damping: get_symbol(
                &libs,
                b"dftd3_load_optimizedpower_damping\0",
            )
            .map(|sym| *sym),
            dftd3_new_cso_damping: get_symbol(&libs, b"dftd3_new_cso_damping\0").map(|sym| *sym),
            dftd3_load_cso_damping: get_symbol(&libs, b"dftd3_load_cso_damping\0").map(|sym| *sym),
            dftd3_delete_param: get_symbol(&libs, b"dftd3_delete_param\0").map(|sym| *sym),
            dftd3_load_gcp_param: get_symbol(&libs, b"dftd3_load_gcp_param\0").map(|sym| *sym),
            dftd3_set_gcp_realspace_cutoff: get_symbol(&libs, b"dftd3_set_gcp_realspace_cutoff\0")
                .map(|sym| *sym),
            dftd3_delete_gcp: get_symbol(&libs, b"dftd3_delete_gcp\0").map(|sym| *sym),
            dftd3_get_dispersion: get_symbol(&libs, b"dftd3_get_dispersion\0").map(|sym| *sym),
            dftd3_get_pairwise_dispersion: get_symbol(&libs, b"dftd3_get_pairwise_dispersion\0")
                .map(|sym| *sym),
            dftd3_get_counterpoise: get_symbol(&libs, b"dftd3_get_counterpoise\0").map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
