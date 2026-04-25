# # Bindgen of simple-dftd3

# This python file can also be opened by Jupyter notebook with jupytext extension.

# User must change `path_repo` to the local path of simple-dftd3 repository.

import subprocess
import os
import shutil
import re
from collections import defaultdict

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
    # Functions can be multi-line in the header:
    # SDFTD3_API_ENTRY <type> SDFTD3_API_CALL
    # <func_name>(<args>) SDFTD3_API_SUFFIX__V_X_Y;
    # We need to match across potential line breaks
    pattern = r'SDFTD3_API_ENTRY\s+\w+\s+SDFTD3_API_CALL\s+(\w+)\s*\([^)]*\)\s*SDFTD3_API_SUFFIX__(\w+);'

    # First, try single-line pattern
    for match in re.finditer(pattern, header_content, re.MULTILINE):
        func_name = match.group(1)
        version_suffix = match.group(2)
        version_map[func_name] = version_suffix

    # For multi-line declarations, we need to handle them differently
    # Join lines and search again
    joined_content = header_content.replace('\n', ' ')
    for match in re.finditer(pattern, joined_content):
        func_name = match.group(1)
        version_suffix = match.group(2)
        version_map[func_name] = version_suffix

    return version_map

# ## Copy necessary headers

os.makedirs(path_header, exist_ok=True)
os.makedirs(f"{path_out}/src", exist_ok=True)

# +
# # copy to header directory

for name in ["s-dftd3.h"]:
    shutil.copy(f"{path_repo}/include/{name}", f"{path_header}")

# Read header and parse version information (after copying)
header_path = f"{path_header}/s-dftd3.h"
with open(header_path, "r") as f:
    header_content = f.read()

version_map = parse_api_versions(header_content)

# +
# # copy to temporary directory

shutil.rmtree(path_temp, ignore_errors=True)
shutil.copytree(path_header, path_temp)

# +
# From now on, we will always work in temporary directory

os.chdir(path_temp)
# -

# ## Perform bindgen

# ### Bindgen

subprocess.run([
    "bindgen",
    "s-dftd3.h", "-o", "ffi.rs",
    "--allowlist-file", "s-dftd3.h",
    # "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ### Post-process

def get_feature_for_version(version_suffix):
    """Convert version suffix (V_0_2) to cargo feature name (api-v0_2)."""
    for v_suffix, feature_name in api_versions:
        if v_suffix == version_suffix:
            return feature_name
    return default_api_version

def add_version_attributes(token, version_map):
    """Add #[cfg(feature = "api-vX_Y")] attributes to extern functions."""

    # Build a mapping of function names to their cfg attributes
    func_cfg_map = {}
    for func_name, version_suffix in version_map.items():
        feature = get_feature_for_version(version_suffix)
        func_cfg_map[func_name] = feature

    lines = token.split('\n')
    result_lines = []
    i = 0
    processed_funcs = set()  # Track functions we've already added cfg to

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check if this is a doc comment line
        if stripped.startswith('#[doc ='):
            # Look ahead to find the function declaration
            j = i + 1
            # Skip additional doc comments if any
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
                        # Insert cfg attribute before the doc comment
                        cfg_line = ' ' * indent + f'#[cfg(feature = "{feature}")]'
                        result_lines.append(cfg_line)
                        processed_funcs.add(func_name)

        result_lines.append(line)
        i += 1

    return '\n'.join(result_lines)

with open("ffi.rs", "r") as f:
    token = f.read()

token = token.replace("::core::ffi::", "")
token = token.replace("minor + 100", "minor * 100")

# Add version attributes to functions
token = add_version_attributes(token, version_map)

# Generate feature documentation
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

token = feature_docs + "\n\n" + token

with open("ffi.rs", "w") as f:
    f.write(token)

# ## Move FFI binding files to output

for name in ["ffi.rs"]:
    shutil.copy(f"{path_temp}/{name}", f"{path_out}/src/{name}")

# ## Cargo fmt

# +
os.chdir(path_out)

subprocess.run(["cargo", "fmt"])
