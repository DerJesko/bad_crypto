use crate::prime;
use num_bigint::BigUint;
use num_traits::Zero;
use rand::prelude::ThreadRng;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PrimeGroup {
    pub(crate) modulus: BigUint,
}

impl PrimeGroup {
    pub(crate) fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        PrimeGroup {
            modulus: prime::random_prime(sec_param, rng),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PrimeGroupElement {
    number: BigUint,
    group: Rc<PrimeGroup>
}

impl PrimeGroupElement {
    pub(crate) fn one(g: &PrimeGroup) -> Self {
        PrimeGroupElement {
            number: Zero::zero(),
            group: g.clone()
        }
    }

    pub(crate) fn mult(&self, b: &Self) -> Self {
        if self.group != b.group {
            panic!("multiplying {:?} and {:?} did't work they have differnt groups", self, b);
        }
        PrimeGroupElement {
            number: &(&self.number + &b.number) % &self.group.modulus,
            group: self.group.clone()
        }
    }

    pub(crate) fn pow(&self, b: &Self) -> Self {
        if self.group != b.group {
            panic!("raising {:?} to the power of {:?} did't work they have differnt groups", self, b);
        }
        PrimeGroupElement {
            number: &(&self.number * &b.number) % &self.group.modulus,
            group: self.group.clone()
        }
    }
}

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
