/*!

# DFTD3 API specification document entrance

For API users, the most important part of this crate is the [`interface`]
module. The commonly used functions and structs can be

- [`DFTD3Model`](interface::DFTD3Model): serve as main driver struct for DFTD3.
- [`dftd3_load_param`](interface::dftd3_load_param): load parameters with xc-functional and DFT-D3 version specified.

To specify custom DFT-D3 parameters, some structs you may interest.

- [`DFTD3RationalDampingParam`](interface::DFTD3RationalDampingParam) for rational damping;
- [`DFTD3ZeroDampingParam`](interface::DFTD3ZeroDampingParam) for zero damping;
- [`DFTD3ModifiedRationalDampingParam`](interface::DFTD3ModifiedRationalDampingParam) for modified rational damping;
- [`DFTD3ModifiedZeroDampingParam`](interface::DFTD3ModifiedZeroDampingParam) for modified zero damping;
- [`DFTD3OptimizedPowerDampingParam`](interface::DFTD3OptimizedPowerDampingParam) for optimized power damping.

You may also check [`DFTD3Param`](interface::DFTD3Param), but note that this struct is somehow low-level API, so use it with more care.

*/
#![doc = include_str!("../readme.md")]

pub mod ffi;
pub mod interface;

#[cfg(feature = "gcp")]
pub mod interface_gcp;

pub mod prelude {
    //! Use `dftd3::prelude::*` to import all the commonly used structs and
    //! functions.
    pub use crate::interface::*;

    #[cfg(feature = "gcp")]
    pub use crate::interface_gcp::*;
}
