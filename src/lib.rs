#[allow(unused_imports)] // they are used in the tests
#[macro_use]
extern crate ndarray;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;

pub mod elgamal;
mod groups;
mod matrix;
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

pub(crate) fn num_bits(x: usize) -> usize {
    (std::mem::size_of::<usize>() * 8) - (x.leading_zeros() as usize)
}
