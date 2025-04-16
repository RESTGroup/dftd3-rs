# # Bindgen of simple-dftd3

# This python file can also be opened by Jupyter notebook with jupytext extension.

# User must change `path_repo` to the local path of simple-dftd3 repository.

import subprocess
import os
import shutil

path_cwd = os.path.abspath(os.getcwd())

# ## Bindgen configuration

# Users may change the following fields for their needs.

# Source code of Netlib Lapack
path_repo = f"{os.getenv('HOME')}/Git-Others/simple-dftd3"

# Path for storing useful header files
path_header = f"{path_cwd}/../header"

# Path for temporary files
path_temp = f"{path_cwd}/tmp"

# Path for bindgen crate root
path_out = f"{path_cwd}/.."

# ## Copy necessary headers

os.makedirs(path_header, exist_ok=True)
os.makedirs(f"{path_out}/src", exist_ok=True)

# +
# # copy to header directory

for name in ["s-dftd3.h"]:
    shutil.copy(f"{path_repo}/include/{name}", f"{path_header}")

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

with open("ffi.rs", "r") as f:
    token = f.read()

token = token.replace("::core::ffi::", "")
token = token.replace("minor + 100", "minor * 100")

token = """
//! FFI bindings for simple-dftd3.

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};
""" + "\n\n" + token

with open("ffi.rs", "w") as f:
    f.write(token)

# ## Move FFI binding files to output

for name in ["ffi.rs"]:
    shutil.copy(f"{path_temp}/{name}", f"{path_out}/src/{name}")

# ## Cargo fmt

# +
os.chdir(path_out)

subprocess.run(["cargo", "fmt"])
