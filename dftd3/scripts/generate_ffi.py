# # Generate FFI bindings for simple-dftd3

# This python file can also be opened by Jupyter notebook with jupytext extension.

# This script generates:
# - ffi_static.rs: static linking FFI bindings
# - ffi_dynamic/: dynamic loading module files

# User must change `path_repo` to the local path of simple-dftd3 repository.

import subprocess
import os
import shutil
import re
from collections import defaultdict
from tree_sitter import Language, Parser
import tree_sitter_rust

path_cwd = os.path.abspath(os.getcwd())

# ## Bindgen configuration

# Users may change the following fields for their needs.

# Source code of simple-dftd3
path_repo = f"{os.getenv('HOME')}/Git-Others/simple-dftd3"

# Path for storing useful header files
path_header = f"{path_cwd}/../header"

# Path for temporary files
path_temp = f"{path_cwd}/tmp"

# Path for bindgen crate root
path_out = f"{path_cwd}/.."

# ## API version configuration

# Available API versions and their cargo feature names
# Versions are cumulative: api-v0_5 includes api-v0_2, api-v0_3, api-v0_4, api-v0_5 functions
api_versions = [
    ("V_0_2", "api-v0_2"),
    ("V_0_3", "api-v0_3"),
    ("V_0_4", "api-v0_4"),
    ("V_0_5", "api-v0_5"),
    ("V_1_3", "api-v1_3"),
]

# Default API version (used when no features are specified)
default_api_version = "api-v0_2"


# ## Parse API version information from header

def parse_api_versions(header_content):
    """Parse the header file to extract function names and their API versions."""
    version_map = {}

    # Pattern to match function declarations with version suffixes
    pattern = r'SDFTD3_API_ENTRY\s+\w+\s+SDFTD3_API_CALL\s+(\w+)\s*\([^)]*\)\s*SDFTD3_API_SUFFIX__(\w+);'

    # Single-line matching
    for match in re.finditer(pattern, header_content, re.MULTILINE):
        func_name = match.group(1)
        version_suffix = match.group(2)
        version_map[func_name] = version_suffix

    # Multi-line declarations: join lines and search again
    joined_content = header_content.replace('\n', ' ')
    for match in re.finditer(pattern, joined_content):
        func_name = match.group(1)
        version_suffix = match.group(2)
        version_map[func_name] = version_suffix

    return version_map


# ## Static FFI generation functions

def get_feature_for_version(version_suffix):
    """Convert version suffix (V_0_2) to cargo feature name (api-v0_2)."""
    for v_suffix, feature_name in api_versions:
        if v_suffix == version_suffix:
            return feature_name
    return default_api_version


def add_version_attributes(token, version_map):
    """Add #[cfg(feature = "api-vX_Y")] attributes to extern functions."""

    func_cfg_map = {}
    for func_name, version_suffix in version_map.items():
        feature = get_feature_for_version(version_suffix)
        func_cfg_map[func_name] = feature

    lines = token.split('\n')
    result_lines = []
    i = 0
    processed_funcs = set()

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        if stripped.startswith('#[doc ='):
            j = i + 1
            while j < len(lines) and lines[j].strip().startswith('#[doc ='):
                j += 1

            if j < len(lines):
                func_line = lines[j]
                func_match = re.match(r'\s*pub fn (\w+)\s*\(', func_line)
                if func_match:
                    func_name = func_match.group(1)
                    if func_name in func_cfg_map and func_name not in processed_funcs:
                        feature = func_cfg_map[func_name]
                        indent = len(line) - len(line.lstrip())
                        cfg_line = ' ' * indent + f'#[cfg(feature = "{feature}")]'
                        result_lines.append(cfg_line)
                        processed_funcs.add(func_name)

        result_lines.append(line)
        i += 1

    return '\n'.join(result_lines)


def generate_static_ffi(token, version_map):
    """Generate ffi_static.rs content from bindgen output."""
    token = token.replace("::core::ffi::", "")
    token = token.replace("minor + 100", "minor * 100")
    token = add_version_attributes(token, version_map)

    feature_docs = """//! FFI bindings for simple-dftd3.
//!
//! # API Version Features
//!
//! This crate provides versioned FFI bindings through cargo features:
//!
//! - `api-v0_2`: Base API (default)
//! - `api-v0_3`: Extends api-v0_2
//! - `api-v0_4`: Extends api-v0_3, adds damping parameter functions
//! - `api-v0_5`: Extends api-v0_4, adds optimized power damping and pairwise dispersion
//! - `api-v1_3`: Full API, adds CSO damping and GCP functions
//!
//! Features are cumulative: enabling `api-v0_5` also enables all functions from
//! earlier versions (api-v0_2, api-v0_3, api-v0_4).

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};
"""

    return feature_docs + "\n\n" + token


# ## Dynamic loading generation functions

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


def dyload_main(token):
    """
    Generate dynamic loading files from bindgen output.

    Returns a dict with keys:
    - ffi_base: base types and imports
    - dyload_struct: struct with Option<extern fn> fields
    - dyload_initializer: DyLoadLib::new implementation
    - dyload_compatible: wrapper functions calling through dyload_lib()
    """
    parsed, parsed_ffi = dyload_parse_file(token)

    token_ffi_base = token

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

    for node_extern in parsed_ffi:
        token_ffi_base = dyload_remove_extern(parsed, node_extern)

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


# ## Main execution

def main():
    # Copy necessary headers
    os.makedirs(path_header, exist_ok=True)
    os.makedirs(f"{path_out}/src", exist_ok=True)
    os.makedirs(f"{path_out}/src/ffi_dynamic", exist_ok=True)

    for name in ["s-dftd3.h"]:
        shutil.copy(f"{path_repo}/include/{name}", f"{path_header}")

    # Read header and parse version information
    header_path = f"{path_header}/s-dftd3.h"
    with open(header_path, "r") as f:
        header_content = f.read()

    version_map = parse_api_versions(header_content)

    # Prepare temporary directory
    shutil.rmtree(path_temp, ignore_errors=True)
    shutil.copytree(path_header, path_temp)
    os.chdir(path_temp)

    # Run bindgen
    subprocess.run([
        "bindgen",
        "s-dftd3.h", "-o", "ffi.rs",
        "--allowlist-file", "s-dftd3.h",
        "--no-layout-tests",
        "--use-core",
        "--merge-extern-blocks",
    ])

    # Read bindgen output
    with open("ffi.rs", "r") as f:
        bindgen_output = f.read()

    # Generate static FFI (ffi_static.rs)
    static_ffi = generate_static_ffi(bindgen_output, version_map)
    with open(f"{path_out}/src/ffi_static.rs", "w") as f:
        f.write(static_ffi)

    # Generate dynamic loading files (ffi_dynamic/)
    dyload_files = dyload_main(bindgen_output)

    with open(f"{path_out}/src/ffi_dynamic/ffi_base.rs", "w") as f:
        f.write(dyload_files["ffi_base"])

    with open(f"{path_out}/src/ffi_dynamic/dyload_struct.rs", "w") as f:
        f.write(dyload_files["dyload_struct"])

    with open(f"{path_out}/src/ffi_dynamic/dyload_initializer.rs", "w") as f:
        f.write(dyload_files["dyload_initializer"])

    with open(f"{path_out}/src/ffi_dynamic/dyload_compatible.rs", "w") as f:
        f.write(dyload_files["dyload_compatible"])

    with open(f"{path_out}/src/ffi_dynamic/mod.rs", "w") as f:
        f.write(DYLOAD_MOD_TEMPLATE)

    # Run cargo fmt
    os.chdir(path_out)
    subprocess.run(["cargo", "fmt"])

    print(f"Generated:")
    print(f"  - {path_out}/src/ffi_static.rs")
    print(f"  - {path_out}/src/ffi_dynamic/")


if __name__ == "__main__":
    main()