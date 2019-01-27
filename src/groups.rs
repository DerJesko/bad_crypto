use crate::prime;
use num_bigint::BigUint;
use rand::prelude::ThreadRng;

pub(crate) trait FiniteGroup {
    fn modulus(&self) -> &BigUint;

    fn mult(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a + b) % self.modulus()
    }

    fn pow(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a * b) % self.modulus()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PrimeGroup {
    pub(crate) modulus: BigUint,
}

impl FiniteGroup for PrimeGroup {
    fn modulus(&self) -> &BigUint {
        &self.modulus
    }
}

impl PrimeGroup {
    pub(crate) fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        PrimeGroup {
            modulus: prime::random_prime(sec_param, rng),
        }
    }
    pub(crate) fn exp_inverse(&self, a: &BigUint) -> BigUint {
        self.modulus() - a
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoublePrimeGroup {
    pub(crate) modulus: BigUint,
}

impl FiniteGroup for DoublePrimeGroup {
    fn modulus(&self) -> &BigUint {
        &self.modulus
    }
}

impl DoublePrimeGroup {
    pub(crate) fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> (Self, BigUint, BigUint) {
        let q = prime::random_prime(sec_param, rng);
        let p = prime::random_prime(sec_param, rng);
        (DoublePrimeGroup { modulus: &p * &q }, p, q)
    }
}
