extern crate dftd3_src;
use dftd3::prelude::*;

#[test]
fn test_linked() {
    let ver = dftd3_get_api_version();
    println!("DFTD3 version: {}", ver);
}
