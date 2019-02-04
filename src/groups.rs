use crate::prime;
use crate::TWO;
use num_bigint::{BigUint, RandBigInt};
use num_traits::Zero;
use rand::prelude::ThreadRng;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimeGroup {
    pub modulus: BigUint,
}

impl PrimeGroup {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        PrimeGroup {
            modulus: prime::random_prime(sec_param, rng),
        }
    }

    pub fn exp_inverse(&self, x: &BigUint) -> BigUint {
        &self.modulus - &(x % &self.modulus)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimeGroupElement {
    number: BigUint,
    pub group: Rc<PrimeGroup>,
}

impl PrimeGroupElement {
    pub fn rand_generator(group: Rc<PrimeGroup>, rng: &mut ThreadRng) -> PrimeGroupElement {
        PrimeGroupElement {
            number: rng.gen_biguint_range(&TWO(), &group.modulus),
            group: group.clone(),
        }
    }
    pub fn new(number: BigUint, group: Rc<PrimeGroup>) -> PrimeGroupElement {
        PrimeGroupElement {
            number: number,
            group: group.clone(),
        }
    }
    pub fn one(g: &Rc<PrimeGroup>) -> Self {
        PrimeGroupElement {
            number: Zero::zero(),
            group: g.clone(),
        }
    }

    pub fn mult(&self, b: &Self) -> Self {
        if self.group != b.group {
            panic!(
                "multiplying {:?} and {:?} did't work they have differnt groups",
                self, b
            );
        }
        PrimeGroupElement {
            number: &(&self.number + &b.number) % &self.group.modulus,
            group: self.group.clone(),
        }
    }

    pub fn pow(&self, b: &BigUint) -> Self {
        PrimeGroupElement {
            number: &(&self.number * b) % &self.group.modulus,
            group: self.group.clone(),
        }
    }
}
/*
#[derive(Debug, Clone)]
pub(crate) struct DoublePrimeGroup {
    pub(crate) modulus: BigUint,
}

impl DoublePrimeGroup {
    pub(crate) fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> (Self, BigUint, BigUint) {
        let q = prime::random_prime(sec_param, rng);
        let p = prime::random_prime(sec_param, rng);
        (DoublePrimeGroup { modulus: &p * &q }, p, q)
    }
}
*/
