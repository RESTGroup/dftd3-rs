---
description: Update dftd3-rs bindings to support a new simple-dftd3 version
argument-hint: <version> (e.g., v1.4.0)
---

You are performing a version update of the dftd3-rs bindings to support simple-dftd3 **$ARGUMENTS**.

Read the CLAUDE.local.md file to get `DFTD3_REPO_PATH`, then follow the procedure below. Use `DFTD3_REPO_PATH` as the path to the upstream simple-dftd3 repository throughout.

## Step 0: Gather upstream changes

Determine the previous version by checking the last entry in the `api_versions` list in `dftd3/scripts/generate_ffi.py`. Then diff the upstream repo:

```bash
cd $DFTD3_REPO_PATH
git fetch --tags
git checkout $ARGUMENTS
git diff v<PREVIOUS>..$ARGUMENTS -- include/s-dftd3.h          # header changes (CRITICAL)
git diff v<PREVIOUS>..$ARGUMENTS -- python/dftd3/interface.py   # Python wrapper changes
git diff v<PREVIOUS>..$ARGUMENTS -- assets/parameters.toml      # parameter DB changes
git diff v<PREVIOUS>..$ARGUMENTS -- src/ --stat                  # Fortran source changes
```

From the header diff, identify:
- New `SDFTD3_API_SUFFIX__V_X_Y` macro (defines the version tag)
- New function declarations (tagged with version suffix)
- New opaque types (e.g., `dftd3_gcp`)
- Changed function signatures (rare)

**Report your findings to the user before proceeding.** Summarize what's new/changed so they can confirm scope.

## Step 1: Update generate_ffi.py

File: `dftd3/scripts/generate_ffi.py`

1. Add the new version to `api_versions` list (follow existing pattern):
   ```python
   ("V_X_Y", "api-vX_Y"),  # new entry
   ```
2. Update the docstring inside `generate_static_ffi()` to list the new version.

## Step 2: Regenerate FFI bindings

```bash
cd dftd3/scripts && python generate_ffi.py
```

This auto-copies the header, runs bindgen, generates `ffi_static.rs` and all `ffi_dynamic/` files.

**Verify** the new function appears:
- In `ffi_static.rs` with `#[cfg(feature = "api-vX_Y")]`
- In `ffi_dynamic/dyload_struct.rs`, `dyload_initializer.rs`, `dyload_compatible.rs`

## Step 3: Update Cargo.toml features

File: `dftd3/Cargo.toml`

Add the new feature extending the previous one:
```toml
api-vX_Y = ["api-v<PREVIOUS>"]
```

Rules:
- Must extend the **previous** feature (cumulative chain)
- If the version introduces a new capability category (like `gcp` for v1_3), create a separate feature and depend on it
- Do **not** update the `default` feature — leave it for manual editing

## Step 4: Update interface.rs (safe wrappers)

This is the most manual step. For each new C function, add a safe Rust wrapper following these patterns:

### Pattern A: New method on existing struct

Add both infallible and fallible (`_f`) versions, gated by the new feature:
```rust
#[cfg(feature = "api-vX_Y")]
pub fn new_method(&self, /* args */) -> ReturnType {
    self.new_method_f(/* args */).unwrap()
}

#[cfg(feature = "api-vX_Y")]
pub fn new_method_f(&self, /* args */) -> Result<ReturnType, DFTD3Error> {
    let mut error = DFTD3Error::new();
    unsafe { ffi::dftd3_new_method(error.get_c_ptr(), /* args */) };
    match error.check() {
        true => Err(error),
        false => Ok(/* result */),
    }
}
```

### Pattern B: New damping type

1. Add a new `DFTD3XyzDampingParam` struct with `#[derive(Builder, Debug, Clone, Deserialize, Serialize)]`
2. Add `new_xyz_damping_f`/`new_xyz_damping` and `load_xyz_damping_f`/`load_xyz_damping` on `DFTD3Param`
3. Implement `DFTD3ParamAPI` for the new struct
4. Add `impl_load_param_api!` and `impl_damping_param_builder!` macro invocations
5. Update `dftd3_load_param`/`dftd3_load_param_f` match arms with `#[cfg]` gating

### Pattern C: New opaque type

1. Create a new `interface_xyz.rs` file
2. Add module in `lib.rs` gated by appropriate feature
3. Update `prelude`
4. Follow the `interface_gcp.rs` pattern

## Step 5: Update parameters module (if parameters changed)

If `assets/parameters.toml` changed upstream:
1. Copy: `cp $DFTD3_REPO_PATH/assets/parameters.toml dftd3/src/parameters.toml`
2. Update `parameters.rs`:
   - Add variant to `D3MethodParams` and `D3DefaultParams`
   - Add to `DFTD3DampingParamEnum`
   - Update `convert_to_damping_param`, `get_variant_entry`, `get_variant_entry_for_defaults`
   - Add `#[cfg(feature = "...")]` gates

## Step 6: Update parsing module (if new damping variants)

File: `dftd3/src/parsing.rs`

1. Update `valid_fields_for_version()` with new variant's valid fields
2. Add `#[cfg(feature = "...")]` gating
3. Add `#[cfg(not(feature = "..."))]` error arm

## Step 7: Update lib.rs (if new modules)

If you added new interface files:
```rust
#[cfg(feature = "api-vX_Y")]
pub mod interface_xyz;
```
And update `prelude`.

## Step 8: Update examples

Check upstream Python examples for new functionality. Add corresponding Rust examples.

## Step 9: Update dftd3-src (if build system changed)

```bash
git diff v<PREVIOUS>..$ARGUMENTS -- CMakeLists.txt meson.build
```
Update `dftd3-src/external_deps/` and `build.rs` if needed.

## Step 10: Update CI

Update `.github/workflows/test-dftd3.yml` to use the new feature version:
- Replace `api-v<PREVIOUS>` with `api-vX_Y` in all `cargo test` commands
- This applies to both the `test-static-linking` and `test-dynamic-loading` jobs

Also check if other CI files need updates: newer conda-forge package version, updated test matrix, new test cases.

## Step 11: Test

```bash
cargo build --features api-vX_Y
cargo test --all-features
cargo test --features api-vX_Y
cargo doc --all-features --no-deps
```

## Step 12: Update documentation

- Update `CHANGELOG.md`
- Update `readme.md` if API surface changed
- Update feature docstring in `generate_ffi.py`

---

**Important invariants** (from project rules — always follow these):
- Never manually edit `ffi_static.rs` or `ffi_dynamic/` files
- Naming: `dftd3_` prefix for functions, `DFTD3` prefix for structs
- Fallible variants use `_f` suffix returning `Result<_, DFTD3Error>`
- Feature flags are cumulative: each version extends the previous
