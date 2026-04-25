"""
Python script to handle dynamic loading

This is a utility package for dftd3-rs FFI bindings.
It handles:
- Splitting `unsafe extern` block from FFI file
- Creating library struct for dynamic loading
- Creating FFI-compatible functions for dynamic loading

This file can also be opened by Jupyter notebook with jupytext extension.
"""

from tree_sitter import Language, Parser
import tree_sitter_rust
import re
import os
import shutil
import subprocess


def dyload_parse_file(token):
    """Parse the FFI file and extract the extern block."""
    parser = Parser(Language(tree_sitter_rust.language()))
    token_transformed = token.replace("unsafe extern \"C\"", "extern \"C\"")
    parsed = parser.parse(bytes(token_transformed, "utf8"))
    parsed_ffi = []
    for node in parsed.root_node.children:
        if node.type == "foreign_mod_item":
            parsed_ffi.append(node)
    return parsed, parsed_ffi


def dyload_remove_extern(parsed, node_extern):
    """Remove the extern block from the parsed file."""
    return parsed.root_node.text.decode("utf8").replace(node_extern.text.decode("utf8"), "")


def dyload_get_ffi_fn(node):
    """Get all function signatures from an extern block."""
    assert node.type == "foreign_mod_item"
    return [n for n in node.children[-1].children if n.type == "function_signature_item"]


def dyload_fn_split(node):
    """Split a function signature into its components."""
    assert node.type == "function_signature_item"
    keys = ["visibility_modifier", "identifier", "parameters", "return_type"]
    result = {key: None for key in keys}
    for (idx, child) in enumerate(node.children):
        if child.type == "->":
            result["return_type"] = node.children[idx + 1]
        elif child.type in keys:
            result[child.type] = child
    assert result["identifier"] is not None
    assert result["parameters"] is not None
    return result


def dyload_get_cfg_attribute(node, full_token):
    """Extract cfg attribute preceding a function if present."""
    node_start = node.start_byte
    lines_before = full_token[:node_start]
    cfg_matches = re.findall(r'#\[cfg\(feature = "([^"]+)"\)\]', lines_before)
    if cfg_matches:
        return cfg_matches[-1]
    return None


def dyload_main(token):
    """
    Main function to generate all dynamic loading files.

    For dynamic loading, API version features are ignored - all functions are available.
    Runtime panic occurs if a function is not found in the loaded library.

    Returns a dict with keys:
    - ffi_base: base types and imports
    - ffi_extern: extern function declarations (for reference)
    - dyload_struct: struct with Option<extern fn> fields
    - dyload_initializer: DyLoadLib::new implementation
    - dyload_compatible: wrapper functions calling through dyload_lib()
    """
    parsed, parsed_ffi = dyload_parse_file(token)

    token_ffi_base = token

    # Collect all function nodes from all extern blocks
    nodes_fn = []
    for node_extern in parsed_ffi:
        nodes_fn.extend(dyload_get_ffi_fn(node_extern))

    token_dyload_struct = ""
    token_dyload_initializer = ""
    token_dyload_compatible = ""

    for node_fn in nodes_fn:
        dict_fn = dyload_fn_split(node_fn)

        visibility_modifier = dict_fn["visibility_modifier"].text.decode("utf8") if dict_fn["visibility_modifier"] else "pub"
        identifier = dict_fn["identifier"].text.decode("utf8")

        return_type_string = ""
        if dict_fn["return_type"] is not None:
            return_type_string = " -> " + dict_fn["return_type"].text.decode("utf8")

        nodes_para = [n for n in dict_fn["parameters"].children if n.type == "parameter"]
        parameters = "(" + ", ".join([n.text.decode("utf8") for n in nodes_para]) + ")"
        parameters_called = ", ".join([n.children[0].text.decode("utf8") for n in nodes_para])

        part_dyload_struct = f"""
            {visibility_modifier} {identifier}: Option<unsafe extern "C" fn{parameters}{return_type_string}>,
        """.strip()

        part_dyload_initializer = f"""
            {identifier}: get_symbol(&libs, b"{identifier}\\0").map(|sym| *sym),
        """.strip()

        part_dyload_compatible = f"""
            {visibility_modifier} unsafe fn {identifier}{parameters}{return_type_string} {{
                dyload_lib().{identifier}.unwrap()({parameters_called})
            }}
        """.strip()

        token_dyload_struct += part_dyload_struct + "\n"
        token_dyload_initializer += part_dyload_initializer + "\n"
        token_dyload_compatible += part_dyload_compatible + "\n\n"

    # Generate ffi_base.rs - the base types without extern functions
    for node_extern in parsed_ffi:
        token_ffi_base = dyload_remove_extern(parsed, node_extern)

    # Remove unused imports from ffi_base
    import_pattern = r'use core::ffi::\{c_char, c_int\};'
    token_ffi_base = re.sub(import_pattern, '', token_ffi_base)

    output_ffi_base = f"""//! Base types and imports for FFI.
//!
//! This file is generated automatically.

#![allow(non_camel_case_types)]

{token_ffi_base}
    """

    output_dyload_struct = f"""//! Library struct definition for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime. Runtime panic occurs if a function
//! is not found in the loaded library.

use super::*;
use core::ffi::{{c_char, c_int}};

pub struct DyLoadLib {{
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub __error: Option<String>,
{token_dyload_struct}
}}
    """

    output_dyload_initializer = f"""//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{{Library, Symbol}};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {{
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}}

impl DyLoadLib {{
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {{
        let mut result = DyLoadLib {{
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
{token_dyload_initializer}
        }};
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }}
}}
    """

    output_dyload_compatible = f"""//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime.

use super::*;
use core::ffi::{{c_char, c_int}};

{token_dyload_compatible}
    """

    return {
        "ffi_base": output_ffi_base,
        "dyload_struct": output_dyload_struct,
        "dyload_initializer": output_dyload_initializer,
        "dyload_compatible": output_dyload_compatible,
    }


DYLOAD_MOD_TEMPLATE = """//! FFI module for dftd3 (dynamic loading).
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

    /// Detect Python interpreter path and return the corresponding lib directory.
    /// Uses OnceLock pattern for lazy initialization.
    static PYTHON_LIB_PATH: OnceLock<Option<String>> = OnceLock::new();

    fn detect_python_lib_path() -> Option<String> {
        PYTHON_LIB_PATH.get_or_init(|| {
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
        }).clone()
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
                    Err(e) => err_msg.push_str(&format!("Failed to load `{candidate}`: {e}\n")),
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
"""


# ## Executable script section

if __name__ == "__main__":
    # This script reads ffi_static.rs and generates ffi_dynamic module files.

    path_cwd = os.path.abspath(os.getcwd())
    path_out = f"{path_cwd}/.."

    # Read ffi_static.rs (generated by perform_bindgen.py)
    ffi_static_path = f"{path_out}/src/ffi_static.rs"
    if not os.path.exists(ffi_static_path):
        print(f"Error: ffi_static.rs not found at {ffi_static_path}")
        print("Please run perform_bindgen.py first to generate ffi_static.rs")
        exit(1)

    with open(ffi_static_path, "r") as f:
        token = f.read()

    # Generate dynamic loading files
    dyload_files = dyload_main(token)

    # Create ffi_dynamic directory
    ffi_dynamic_dir = f"{path_out}/src/ffi_dynamic"
    os.makedirs(ffi_dynamic_dir, exist_ok=True)

    # Write the files
    with open(f"{ffi_dynamic_dir}/ffi_base.rs", "w") as f:
        f.write(dyload_files["ffi_base"])

    with open(f"{ffi_dynamic_dir}/dyload_struct.rs", "w") as f:
        f.write(dyload_files["dyload_struct"])

    with open(f"{ffi_dynamic_dir}/dyload_initializer.rs", "w") as f:
        f.write(dyload_files["dyload_initializer"])

    with open(f"{ffi_dynamic_dir}/dyload_compatible.rs", "w") as f:
        f.write(dyload_files["dyload_compatible"])

    with open(f"{ffi_dynamic_dir}/mod.rs", "w") as f:
        f.write(DYLOAD_MOD_TEMPLATE)

    # Cargo fmt
    os.chdir(path_out)
    subprocess.run(["cargo", "fmt"])

    print(f"Generated ffi_dynamic module at {ffi_dynamic_dir}")