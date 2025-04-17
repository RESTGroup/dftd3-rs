# Common API documentation for custom damping parameter specification

If your task is to retrive damping parameters of some specific xc-functionals, you may wish to try [`dftd3_load_param`] function.

In this crate, you may have three ways to define customized parameters:

- By `DFTD3***DampingParam` struct. In this way, all parameters (include optional parameters with default value) must be provided. For example of D3-Zero:

    ```rust
  # use dftd3::prelude::*;
  # let atm = true;
    let param = DFTD3ZeroDampingParam {
        s6: 1.0,
        s8: 1.683,
        rs6: 1.139,
        rs8: 1.0,
        alp: 14.0,
        s9: if atm { 1.0 } else { 0.0 },
    };
    let param = param.new_param();
    // this will give param: DFTD3Param
    ```

- By `DFTD3***DampingParamBuilder` struct. In this way, optional parameters can be omitted. For example of D3-Zero:

    ```rust
  # use dftd3::prelude::*;
  # let atm = true;
    let param = DFTD3ZeroDampingParamBuilder::default()
        .s8(1.683)
        .rs6(1.139)
        .s9(if atm { 1.0 } else { 0.0 })
        .init();
    // this will give param: DFTD3Param
    ```

- By `DFTD3Param` utility. In this way, all parameters must be provided. For example of D3-Zero:

    ```rust
  # use dftd3::prelude::*;
  # let atm = true;
    let param = DFTD3Param::new_zero_damping(
        1.0,                         // s6
        1.683,                       // s8
        if atm { 1.0 } else { 0.0 }, // s9
        1.139,                       // rs6
        1.0,                         // rs8
        14.0,                        // alp
    );
    ```

Please note that different DFT-D3 versions may have different parameters, for example modified zero damping have another parameter `bet` for beta, and rational damping (D3-BJ) have parameter name `a1`, `a2` instead of `rs6`, `rs8`.
