use crate::prime;
use crate::TWO;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::One;
use rand::prelude::ThreadRng;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimeGroup {
    pub modulus: BigUint,
    pub big_prime: BigUint,
    pub generator: BigUint,
}

impl PrimeGroup {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        loop {
            let p = prime::random_prime(sec_param, rng);
            let one: BigUint = One::one();
            if prime::prime_eh(&(&(&p * &(2 as u8)) + &one), sec_param, rng) {
                return PrimeGroup {
                    modulus: &p * &(2 as u8) + one,
                    big_prime: p,
                    generator: (3 as u8).to_biguint().unwrap(),
                };
            }
        }
    }

    pub fn exp_inverse(&self, x: &BigUint) -> BigUint {
        &self.big_prime - &(x % &self.big_prime)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimeGroupElement {
    number: BigUint,
    pub group: Rc<PrimeGroup>,
}

impl PrimeGroupElement {
    pub fn rand_generator(group: &Rc<PrimeGroup>, rng: &mut ThreadRng) -> PrimeGroupElement {
        let r = rng.gen_biguint_range(&TWO(), &group.big_prime);
        Self::new(r, &group)
    }

    pub fn new(number: BigUint, group: &Rc<PrimeGroup>) -> PrimeGroupElement {
        PrimeGroupElement {
            number: group.generator.modpow(&number, &group.modulus),
            group: group.clone(),
        }
    }
    pub fn one(g: &Rc<PrimeGroup>) -> PrimeGroupElement {
        let one: BigUint = One::one();
        PrimeGroupElement {
            number: one,
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
            number: &(&self.number * &b.number) % &self.group.modulus,
            group: self.group.clone(),
        }
    }

    pub fn pow(&self, b: &BigUint) -> Self {
        PrimeGroupElement {
            number: self.number.modpow(b, &self.group.modulus),
            group: self.group.clone(),
        }
    }
}
