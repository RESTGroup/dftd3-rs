//! FFI module for dftd3 (dynamic loading).
//!
//! This module provides dynamic loading support.
//!
//! # Rule and guide of dynamic loading
//!
//! By enabling the `dynamic_loading` feature (which is the default), the crate
//! will attempt to load the s-dftd3 shared library at runtime. The loading
//! process will search for the library in multiple locations, by the following
//! order:
//!
//! 1. User-defined candidates via environment variables `DFTD3_DYLOAD`.
//!    Multiple paths can be separated by `:` (Unix/macOS) or `;` (Windows).
//! 2. LD_LIBRARY_PATH style discovery via environment variables
//!    `LD_LIBRARY_PATH` (Linux), `DYLD_LIBRARY_PATH` and
//!    `DYLD_FALLBACK_LIBRARY_PATH` (macOS), `PATH` (Windows). Note we are not
//!    distinguishing different operating systems, so all these environment
//!    variables will be checked on all platforms. Path lists are split by `:`
//!    on Unix/macOS and `;` on Windows.
//! 3. Python interpreter path discovery: For each python interpreter found, the
//!    library is expected to be at the `lib` directory of the python
//!    installation. For example, if python is at `/path/bin/python`, the
//!    library is expected at `/path/lib/libs-dftd3.so`.
//!    - The python interpreter path of `DFTD3_PYTHON_PATH` environment
//!      variable, if set.
//!    - The conda prefix path of `CONDA_PREFIX` environment variable, if set.
//!      On Windows also checks `{CONDA_PREFIX}/Library/bin` and
//!      `{CONDA_PREFIX}/Library/lib`.
//!    - The python interpreter path in `PATH` environment variable, if exists.
//!      Will first check `python`, then `python3`.
//! 4. Standard system candidates, such as `lib{LIB_NAME_LINK}.so` in some
//!    common library directories such as `/usr/lib`, `/usr/local/lib`, and
//!    `/lib`. These are Unix-specific and silently skipped when absent on
//!    Windows.
//!
//! For API developer, if you want to check the library `libs-dftd3.so` loading
//! sequence, you can try the following code:
//! ```rust
//! let candidates = unsafe { &dftd3::ffi::dyload_lib().__libraries_path };
//! println!("Library loading candidates: {candidates:#?}");
//! ```

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub const MOD_NAME: &str = module_path!();
pub const LIB_NAME: &str = "DFTD3";
pub const LIB_NAME_SHOW: &str = "s-dftd3";
#[cfg(windows)]
/// On Windows (MinGW), the library includes a "-1" SO version suffix.
/// The "-1" may need updating when upstream bumps ABI.
pub const LIB_NAME_LINK: &str = "s-dftd3-1";
#[cfg(not(windows))]
pub const LIB_NAME_LINK: &str = "s-dftd3";

#[cfg(feature = "dynamic_loading")]
mod dynamic_loading_specific {
    use super::*;
    use libloading::Library;
    use std::fmt::Debug;
    use std::sync::OnceLock;

    #[cfg(not(windows))]
    use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};

    #[cfg(windows)]
    use std::env::consts::DLL_SUFFIX;
    #[cfg(windows)]
    /// MinGW convention uses "lib" prefix, but std::env::consts::DLL_PREFIX
    /// returns "" on all Windows targets regardless of toolchain.
    const DLL_PREFIX: &str = "lib";

    #[cfg(windows)]
    const PATH_LIST_SEPARATOR: char = ';';
    #[cfg(not(windows))]
    const PATH_LIST_SEPARATOR: char = ':';

    /// Detect Python interpreter path and return the corresponding lib
    /// directory. Uses OnceLock pattern for lazy initialization.
    static PYTHON_LIB_PATH: OnceLock<Vec<String>> = OnceLock::new();

    fn detect_python_lib_paths() -> Vec<String> {
        PYTHON_LIB_PATH
            .get_or_init(|| {
                let mut lib_paths = vec![];

                // 1. Check explicit environment variable first
                if let Ok(python_path) = std::env::var("DFTD3_PYTHON_PATH") {
                    if let Some(lib_path) = extract_lib_from_python_bin(&python_path) {
                        lib_paths.push(lib_path);
                    }
                }

                // 2. Check conda prefix exists (on Windows also checks Library/{bin,lib})
                if let Ok(conda_prefix) = std::env::var("CONDA_PREFIX") {
                    let conda_lib_path = format!("{conda_prefix}/lib");
                    if std::path::Path::new(&conda_lib_path).exists() {
                        lib_paths.push(conda_lib_path);
                    }
                    #[cfg(windows)]
                    {
                        let conda_library_bin = format!("{conda_prefix}/Library/bin");
                        if std::path::Path::new(&conda_library_bin).exists() {
                            lib_paths.push(conda_library_bin);
                        }
                        let conda_library_lib = format!("{conda_prefix}/Library/lib");
                        if std::path::Path::new(&conda_library_lib).exists() {
                            lib_paths.push(conda_library_lib);
                        }
                    }
                }

                // 3. Try to find python in PATH
                if let Ok(paths) = std::env::var("PATH") {
                    // first check python, then python3
                    for path in paths.split(PATH_LIST_SEPARATOR) {
                        let python_bin = format!("{path}/python");
                        if std::path::Path::new(&python_bin).exists() {
                            if let Some(lib_path) = extract_lib_from_python_bin(&python_bin) {
                                lib_paths.push(lib_path);
                            }
                        }
                    }
                    for path in paths.split(PATH_LIST_SEPARATOR) {
                        let python_bin = format!("{path}/python3");
                        if std::path::Path::new(&python_bin).exists() {
                            if let Some(lib_path) = extract_lib_from_python_bin(&python_bin) {
                                lib_paths.push(lib_path);
                            }
                        }
                    }
                }

                lib_paths
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

        // User-defined candidates via environment variables (paths split by platform
        // separator)
        for env_var in [format!("DFTD3_DYLOAD_{LIB_NAME}").as_str(), "DFTD3_DYLOAD"] {
            if let Ok(path) = std::env::var(env_var) {
                candidates.extend(path.split(PATH_LIST_SEPARATOR).map(|s| s.to_string()));
            }
        }

        // LD_LIBRARY_PATH style discovery (paths split by platform separator)
        for env_var in [
            "LD_LIBRARY_PATH",            // linux
            "DYLD_LIBRARY_PATH",          // macos
            "DYLD_FALLBACK_LIBRARY_PATH", // macos
            "PATH",                       // windows
        ] {
            if let Ok(paths) = std::env::var(env_var) {
                for path in paths.split(PATH_LIST_SEPARATOR) {
                    candidates.push(format!("{path}/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"));
                }
            }
        }

        // Python interpreter path discovery (cached)
        for lib_path in detect_python_lib_paths() {
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
- If `lib{LIB_NAME_LINK}.so` (or `{LIB_NAME_LINK}.dll` on Windows) is installed on your system.
- If `LD_LIBRARY_PATH` (Unix/macOS) or `PATH` (Windows) is set correctly.
- Python interpreter path discovery: if Python is at `/path/bin/python`,
  the library is expected at `/path/lib/libs-dftd3.so`.
- On Windows with conda, try `{LIB_NAME_LINK}` from `<CONDA_PREFIX>/Library/bin`.

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
