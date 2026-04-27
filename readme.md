# simple-dftd3 FFI bindings

This project contains simple-dftd3 FFI bindings, wrapper and build-from-source.

Current binding of simple-dftd3: [![v1.3.0](https://img.shields.io/github/v/release/dftd3/simple-dftd3)](https://github.com/dftd3/simple-dftd3/releases/v1.3.0)

Source code of simple-dftd3 is available on [github](https://github.com/dftd3/simple-dftd3).

This crate is not official bindgen project. It is originally intended to potentially serve rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

## Crate `dftd3`

This crate contains simple-dftd3 FFI bindings and wrapper.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/dftd3.svg)](https://crates.io/crates/dftd3) |
| API Document | [![API Documentation](https://docs.rs/dftd3/badge.svg)](https://docs.rs/dftd3) |
| FFI Binding | [![v1.3.0](https://img.shields.io/github/v/release/dftd3/simple-dftd3)](https://github.com/dftd3/simple-dftd3/releases/v1.3.0) |

### Example: r2SCAN with D3(BJ)

For example, full code for computing r2SCAN dispersion energy with D3(BJ):

```rust
use dftd3::prelude::*;

// atom indices
let numbers = vec![6, 6, 6, 6, 6, 6, 53, 1, 1, 1, 1, 1, 16, 1, 6, 1, 1, 1];
// geometry in angstrom
#[rustfmt::skip]
let positions = vec![
    -0.755422531,  -0.796459123,  -1.023590391,
     0.634274834,  -0.880017014,  -1.075233285,
     1.406955202,   0.199695367,  -0.653144334,
     0.798863737,   1.361204515,  -0.180597909,
    -0.593166787,   1.434312023,  -0.133597923,
    -1.376239198,   0.359205222,  -0.553258516,
    -1.514344238,   3.173268101,   0.573601106,
     1.110906949,  -1.778801728,  -1.440619836,
     1.399172302,   2.197767355,   0.147412751,
     2.486417780,   0.142466525,  -0.689380574,
    -2.454252250,   0.422581120,  -0.512807958,
    -1.362353593,  -1.630564523,  -1.348743149,
    -3.112683203,   6.289227834,   1.226984439,
    -4.328789697,   5.797771251,   0.973373089,
    -2.689135032,   6.703163830,  -0.489062886,
    -1.684433029,   7.115457372,  -0.460265708,
    -2.683867206,   5.816530502,  -1.115183775,
    -3.365330613,   7.451201412,  -0.890098894,
];
// convert angstrom to bohr
let positions = positions.iter().map(|&x| x / 0.52917721067).collect::<Vec<f64>>();
// generate DFTD3 model
let model = DFTD3Model::new(&numbers, &positions, None, None);
// retrive the DFTD3 parameters
let param = dftd3_load_param("d3bj", "r2SCAN", false);
// obtain the dispersion energy and gradient
let (energy, gradient, _) = model.get_dispersion(&param, true).into();
let gradient = gradient.unwrap();

println!("Dispersion energy: {}", energy);
let energy_ref = -0.00578401192369041;
assert!((energy - energy_ref).abs() < 1e-9);
println!("Dispersion gradient:");
gradient.chunks(3).for_each(|chunk| println!("{:16.9?}", chunk));
```

### Example: Custom parameters by toml

```rust
use dftd3::prelude::*;
// Use custom parameters by toml string
let input = r#"{version = "d3bj", a1 = 0.3981, s8 = 1.9889, a2 = 4.4211, atm = false}"#;
// You can also use the following input to specify B3LYP-D3(BJ) parameters
// let input = r#"{version = "d3bj", method = "b3lyp"}"#;
// toml parameter type
let damping_param = dftd3_parse_damping_param_from_toml(input);
// FFI parameter type
let dftd3_param = damping_param.new_param();

let atom_charges = vec![8, 1, 1];
// coordinates in bohr
#[rustfmt::skip]
let coordinates = vec![
    0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 1.807355,
    1.807355, 0.000000, -0.452500,
];
let model = DFTD3Model::new(&atom_charges, &coordinates, None, None);
let res = model.get_dispersion(&dftd3_param, false);
let eng = res.energy;
println!("Dispersion energy: {eng}");
```

### Cargo features of `dftd3`

Default cargo features of `dftd3` are:
- **`api-v0_5`**: Corresponding to the original simple-dftd3 [v0.5](https://github.com/dftd3/simple-dftd3/releases/tag/v0.5.1). This will enable versions `bj`, `zero`, `mbj`, `mzero`, `op`. Note `cso` and cargo feature `gcp` are not included in `api-v0_5`.
- **`dynamic_loading`**: This will enable dynamic loading of `s-dftd3` library, which can be more flexible for users who do not want to perform static linking. Please place `libs-dftd3.so` in `LD_LIBRARY_PATH` (for macos, place `libs-dftd3.dylib` in `DYLD_LIBRARY_PATH`), and function symbols will be loaded at runtime.

Other cargo features of `dftd3` are:
- **`gcp`**: Support of geometric counterpoise correction. Please note that this is not available in latest stable release of simple-dftd3 (at the time writing this readme, is v1.3.0). Unless you build simple-dftd3 from git repository, you may not use this feature (especially installed simple-dftd3 from conda or similar).
- **`api-v1_3`**: Corresponding to the original simple-dftd3 [v1.3](https://github.com/dftd3/simple-dftd3/releases/tag/v1.3.1). This will additionally enable versions `cso` and cargo feature `gcp`.
- **`json`**: This will enable JSON parsing for DFTD3 parameters. Note that toml parsing is builtin, and json is an optional feature.

## Installation guide and Crate `dftd3-src`

**This crate is only useful for static loading**: If you enabled cargo feature `dynamic_loading` (which is enabled by default), you just need to place `libs-dftd3.so` in `LD_LIBRARY_PATH`, or make dftd3 available in your python environment. The library will be loaded at runtime, so you do not need to perform any static loading.

If you need to link `s-dftd3` library by static loading (either static linking by `libs-dftd3.a` or dynamic linking by `libs-dftd3.so`), proceed to the following instructions.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/dftd3-src.svg)](https://crates.io/crates/dftd3-src) |

To use crate `dftd3` in rust, you may need to perform some configuration to properly link `libs-dftd3.so` into your own program.

### Install `s-dftd3`

Please refer to original [github](https://github.com/dftd3/simple-dftd3) repository for more instructions.

The easiest way is install from conda/mamba, and you can retrive the shared/static library therein.

### Manually link `s-dftd3` into your project

Similar to other projects, after library search path properly defined
```rust,ignore
println!("cargo:rustc-link-search=native={}", path);
```
you may link `s-dftd3` and `mctc-lib` by cargo instructions **in your own project**:
```rust,ignore
// following code is for static linking
println!("cargo:rustc-link-lib=static=s-dftd3");
println!("cargo:rustc-link-lib=static=mctc-lib");
// following code is for dynamic linking
println!("cargo:rustc-link-lib=s-dftd3");
println!("cargo:rustc-link-lib=mctc-lib");
```
It should be noted that, for static linking, you may also need to dynamic link Fortran and OpenMP libraries (for the library installed by conda or mamba, it is usually `gfortran` and `gomp`).

### Link `s-dftd3` by crate `dftd3-src`

You can also link `s-dftd3` by crate `dftd3-src`.

First, **in your own project**'s `lib.rs` or `main.rs`, you need to add a line for explicitly importing this library:
```rust,ignore
extern crate dftd3_src;
```

If you have compiled `s-dftd3` library, make sure path of it (together with `mctc-lib`) is either in
- `DFTD3_DIR`
- `REST_EXT_DIR`
- `LD_LIBRARY_PATH`
- or in other common system library paths.

If you have not compiled `s-dftd3` library, you may try out cargo feature `build_from_source`, but that can also cause trobule when distributing your program binary, so use with caution.

### Cargo features of `dftd3-src`

- **`build_from_source`**: This will use CMake and meson, and pull code from github to first perform build for simple-dftd3. Though this option can be developer-friendly (you do not need to perform any other configurations to make program compile and run by cargo), `build_from_source` does not provide customized compilation.

    CMake configurable variables (can be defined as environment variables):
    - `DFTD3_SRC`: git repository source directory or URL;
    - `DFTD3_VER`: version of DFT-D3 (default v1.3.0);

- **`static`**: This will link static libary instead of dynamic one. Please note that 1. static linking may require additional Fortran and OpenMP linking, which is not provided in this crate; 2. staticly linking LGPL-3.0 license may require your project to be GPL-3.0.

## License

This repository is licensed under LGPL-3.0, the same to simple-dftd3.

Some parts of this project is derivative work of original library [simple-dftd3](https://github.com/dftd3/simple-dftd3), and contains some source code (headers, toml parameters) and AI-translated/generated code.
