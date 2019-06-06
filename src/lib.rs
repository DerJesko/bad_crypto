#[macro_use]
extern crate ndarray;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;

//mod bgn;
mod dummy_nikz;
pub mod elgamal;
mod fields;
mod groups;
mod matrix;
mod naor_yung;
mod prime;
mod prime2000;
pub mod regev;
pub mod rsa;
mod traits;

#[cfg(test)]
mod tests;

use num_bigint::{BigUint, ToBigUint};
use prime2000::PRIME2000;

#[allow(non_snake_case)]
pub(crate) fn TWO() -> BigUint {
    { 2 as usize }.to_biguint().unwrap()
}

#[allow(non_snake_case)]
pub(crate) fn MIN_ODD() -> BigUint {
    PRIME2000[1999].to_biguint().unwrap()
}
