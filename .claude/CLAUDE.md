# Agent instruction of DFTD3 Rust FFI and Wrapper

## Notes to human developers

You should also create a file `CLAUDE.local.md` to place local resources:
- `DFTD3_REPO_PATH`: local of original simple-dftd3 repository. The source code can help you understand how dftd3 works.

## DFTD3 original library

General rules
- This repository should live at `DFTD3_REPO_PATH`, which is defined in `CLAUDE.local.md`.
- **This repository should not be modified**, unless you are going to checkout specific tags (versions) of dftd3.

Important files for FFI and wrapper development:
- `include/s-dftd3.h`: the headers. Note that these files are also copied to this project under `dftd3/headers` folder.
- `python/dftd3`: the python wrapper of dftd3. We should at least implement all major features of the certain wrapper:
  - `interface.py`, corresponding to this project `dftd3/src/interface.rs`, also `interface_gcp.rs`.
  - `parameters.py`, corresponding to this project `dftd3/src/parameters.rs`.
  - Make sure the functionalities are tested. We use `dftd3/example/test_interface.rs` corresponding to `test_interface.py` in the original wrapper for testing.
- `assets/parameters.toml`: the parameters file, which should be copied to `dftd3/src/assets/parameters.toml` in this project.

## The additional feature in this crate

- We support toml parsing of DFTD3 parameters. The related code is at `/dftd3/src/parsing.rs`. The related test is at `dftd3/example/test_parsing.rs`.
- We support dynamic loading of the DFTD3 library.
- We use tags such as `api-v0_5` to reflect the API version of DFTD3 we are using.

## Naming convention

- For functions and structs that will be exposed to users, add prefix `dftd3_` for general functions, and `DFTD3` for structs.
- If some function is to be fallible, we can add suffix `_f` (`fn <func>_f -> Result<_, DFTD3Error>`).

## Header handling

We use bindgen (python script at `scripts/generate_bindings.py`) to generate Rust bindings for the C header files. **Not modify the generated files directly**.
