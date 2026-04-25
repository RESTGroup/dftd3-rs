//! FFI module for dftd3 (dynamic loading).
//!
//! This module provides dynamic loading support.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub const MOD_NAME: &str = module_path!();
pub const LIB_NAME: &str = "DFTD3";
pub const LIB_NAME_SHOW: &str = "s-dftd3";
pub const LIB_NAME_LINK: &str = "s-dftd3";

#[cfg(feature = "dynamic_loading")]
mod dynamic_loading_specific {
    use super::*;
    use libloading::Library;
    use std::fmt::Debug;
    use std::sync::OnceLock;

    use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};

    /// Detect Python interpreter path and return the corresponding lib
    /// directory. Uses OnceLock pattern for lazy initialization.
    static PYTHON_LIB_PATH: OnceLock<Option<String>> = OnceLock::new();

    fn detect_python_lib_path() -> Option<String> {
        PYTHON_LIB_PATH
            .get_or_init(|| {
                // 1. Check explicit environment variable first
                if let Ok(python_path) = std::env::var("DFTD3_PYTHON_PATH") {
                    if let Some(lib_path) = extract_lib_from_python_bin(&python_path) {
                        return Some(lib_path);
                    }
                }

                // 2. Try to find python in PATH
                if let Ok(paths) = std::env::var("PATH") {
                    for path in paths.split(":") {
                        for python_name in ["python3", "python"] {
                            let python_bin = format!("{path}/{python_name}");
                            if std::path::Path::new(&python_bin).exists() {
                                if let Some(lib_path) = extract_lib_from_python_bin(&python_bin) {
                                    return Some(lib_path);
                                }
                            }
                        }
                    }
                }

                None
            })
            .clone()
    }

    fn extract_lib_from_python_bin(python_bin: &str) -> Option<String> {
        // If python is at /path/to/bin/python, library should be at /path/to/lib/
        let bin_path = std::path::Path::new(python_bin);
        if let Some(parent) = bin_path.parent() {
            if let Some(base) = parent.parent() {
                let lib_path = base.join("lib");
                if lib_path.exists() {
                    return Some(lib_path.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    fn get_lib_candidates() -> Vec<String> {
        let mut candidates = vec![];

        // User-defined candidates via environment variables
        for env_var in [format!("DFTD3_DYLOAD_{LIB_NAME}").as_str(), "DFTD3_DYLOAD"] {
            if let Ok(path) = std::env::var(env_var) {
                candidates.extend(path.split(":").map(|s| s.to_string()));
            }
        }

        // LD_LIBRARY_PATH style discovery
        for env_var in ["LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"] {
            if let Ok(paths) = std::env::var(env_var) {
                for path in paths.split(":") {
                    candidates.push(format!("{path}/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"));
                }
            }
        }

        // Python interpreter path discovery (cached)
        if let Some(lib_path) = detect_python_lib_path() {
            candidates.push(format!("{lib_path}/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"));
        }

        // Standard system candidates
        candidates.extend(vec![
            format!("{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}dftd3{DLL_SUFFIX}"),
            format!("/usr/lib/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("/usr/local/lib/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("/lib/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
        ]);
        candidates
    }

    fn check_lib_loaded(lib: &DyLoadLib) -> bool {
        lib.dftd3_get_version.is_some()
    }

    fn panic_no_lib_found<S: Debug>(candidates: &[S], err_msg: &str) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Unable to dynamically load the {LIB_NAME_SHOW} (`{LIB_NAME_LINK}`) shared library.
Candidates: {candidates:#?}

Please check:
- If dynamic-loading is not desired, disable the `dynamic_loading` feature in Cargo.toml.
- Use environment variable `DFTD3_DYLOAD_{LIB_NAME}` or `DFTD3_DYLOAD` to specify the library path.
- If `lib{LIB_NAME_LINK}.so` is installed on your system.
- If `LD_LIBRARY_PATH` is set correctly.
- Python interpreter path discovery: if Python is at `/path/bin/python`,
  the library is expected at `/path/lib/libs-dftd3.so`.

Error message(s):
{err_msg}
"#
        )
    }

    fn panic_condition_not_met<S: Debug>(candidates: &[S]) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Library loaded but condition not met: `dftd3_get_version` not found.
Found libraries: {candidates:#?}

Please check that the loaded library is a valid s-dftd3 library.
"#
        )
    }

    pub unsafe fn dyload_lib() -> &'static DyLoadLib {
        static LIB: OnceLock<DyLoadLib> = OnceLock::new();

        LIB.get_or_init(|| {
            let candidates = get_lib_candidates();
            let (mut libraries, mut libraries_path) = (vec![], vec![]);
            let mut err_msg = String::new();
            for candidate in &candidates {
                match Library::new(candidate) {
                    Ok(l) => {
                        libraries.push(l);
                        libraries_path.push(candidate.to_string());
                    },
                    Err(e) => err_msg.push_str(&format!(
                        "Failed to load `{candidate}`: {e}
"
                    )),
                }
            }
            let lib = DyLoadLib::new(libraries, libraries_path);
            if lib.__libraries.is_empty() {
                panic_no_lib_found(&candidates, &err_msg);
            }
            if !check_lib_loaded(&lib) {
                panic_condition_not_met(&lib.__libraries_path);
            }
            lib
        })
    }
}

#[cfg(feature = "dynamic_loading")]
pub use dynamic_loading_specific::*;

/* #region general configuration */

pub(crate) mod ffi_base;
pub use ffi_base::*;

#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_compatible;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_initializer;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_struct;

#[cfg(feature = "dynamic_loading")]
pub use dyload_compatible::*;
#[cfg(feature = "dynamic_loading")]
pub use dyload_struct::*;

/* #endregion */
