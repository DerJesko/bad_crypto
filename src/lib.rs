#[allow(unused_imports)] // they are used in the tests
#[macro_use]
extern crate ndarray;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;

pub mod batched_regev;
mod distributions;
pub mod elgamal;
mod groups;
mod matrix;
mod prime;
mod rabin;
mod ring;
pub mod rsa;
mod small_prime;
mod traits;

use num_bigint::{BigUint, ToBigUint};
use rand::prelude::*;
use rand_distr::Binomial;

#[allow(non_snake_case)]
pub(crate) fn TWO() -> BigUint {
    { 2 as usize }.to_biguint().unwrap()
}

pub(crate) fn num_bits(x: usize) -> usize {
    (std::mem::size_of::<usize>() * 8) - (x.leading_zeros() as usize)
}

pub(crate) fn chi(b: u64, rng: &mut ThreadRng) -> isize {
    let distribution = Binomial::new(b * 2 - 1, 0.5).unwrap();
    let r = rng.sample(distribution);
    (r as isize) - (b as isize) + 1
}
