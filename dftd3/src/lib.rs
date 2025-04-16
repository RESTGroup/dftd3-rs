/*!

# DFTD3 API specification document entrance

For API users, the most important part of this crate is the [`interface`]
module. The commonly used functions and structs can be
- [`DFTD3Model`](interface::DFTD3Model)
- [`dftd3_load_param`](interface::dftd3_load_param)

*/

pub mod ffi;
pub mod interface;

#[cfg(feature = "gcp")]
pub mod interface_gcp;

pub mod prelude {
    pub use crate::interface::*;
}
