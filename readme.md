# simple-dftd3 FFI bindings

This project contains simple-dftd3 FFI bindings, wrapper and build-from-source.

Current binding of simple-dftd3: [bd59f81](https://github.com/dftd3/simple-dftd3/commit/bd59f81e9f3ab3cf383e4699e1dda03dce5d9845) (after [![v1.2.1](https://img.shields.io/github/v/release/dftd3/simple-dftd3)](https://github.com/dftd3/simple-dftd3/releases/v1.2.1))

Source code of simple-dftd3 is available on [github](https://github.com/dftd3/simple-dftd3).

This crate is not official bindgen project. It is originally intended to potentially serve rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

## Crate `dftd3`

This crate contains simple-dftd3 FFI bindings and wrapper.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/dftd3.svg)](https://crates.io/crates/dftd3) |
| API Document | [![API Documentation](https://docs.rs/dftd3/badge.svg)](https://docs.rs/dftd3) |
| FFI Binding | [bd59f81](https://github.com/dftd3/simple-dftd3/commit/bd59f81e9f3ab3cf383e4699e1dda03dce5d9845) after [![v1.2.1](https://img.shields.io/github/v/release/dftd3/simple-dftd3)](https://github.com/dftd3/simple-dftd3/releases/v1.2.1) |

### Cargo features of `dftd3`

- **`gcp`**: Support of geometric counterpoise correction. Please note that this is not available in latest stable release of simple-dftd3 (at the time writing this readme, is v1.2.1). Unless you build simple-dftd3 from git repository, you may not use this feature (especially installed simple-dftd3 from conda or similar).

### Example: r2SCAN with D3(BJ)

For example, full code for computing r2SCAN dispersion energy with D3(BJ):

```rust
fn main() {
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
}
```

## License

This repository is licensed under LGPL-3.0, the same to simple-dftd3.

Some parts of this project is derivative work of original library [simple-dftd3](https://github.com/dftd3/simple-dftd3), and contains some source code (headers) and AI-translated code (pytest).
