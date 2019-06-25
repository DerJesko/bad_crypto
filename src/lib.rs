#[macro_use]
extern crate ndarray;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;

//mod bgn;
mod dummy_nikz;
pub mod elgamal;
mod groups;
mod matrix;
mod naor_yung;
mod prime;
pub mod regev;
mod ring;
pub mod rsa;
mod small_prime;
mod traits;

#[cfg(test)]
mod tests;

use num_bigint::{BigUint, ToBigUint};

#[allow(non_snake_case)]
pub(crate) fn TWO() -> BigUint {
    { 2 as usize }.to_biguint().unwrap()
}
