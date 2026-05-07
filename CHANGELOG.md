# Changelog

## v0.2.2 -- 2026-05-07

Update original simple-dftd3 version to be binding (v1.4.0).

## v0.2.1 -- 2026-05-07

Behavior change:

- Update dynamic loading logic
    - now check CONDA_PREFIX first, then python in PATH, then python3 in PATH
    - now all python in PATH will try to be parsed, not the first one

## v0.2.0 -- 2026-04-27

Code changes applied from RESTGroup/dftd3-rs#3 and RESTGroup/dftd3-rs#4.

API breaking changes:

- Cargo features added. Now default changes to `dynamic_loading` and `api-v0_5` (previously static loading is a requirement).

Enhancements:

- FFI (to original simple-dftd3) updated to DFTD3 v1.3.
- Added dynamic loading of the DFTD3 library (optional, enabled by default).
- Added cargo features to select version of the DFTD3 API (v0.5 `api-v0_5` is default, latest v1.3 `api-v1_3`).
- Added support for toml/json parsing of DFTD3 parameters (toml is builtin, json is optional).

## v0.1.1 -- 2025-04-17
