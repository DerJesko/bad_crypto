use crate::prime;
use num_bigint::BigUint;
use num_traits::{Zero,One};
use rand::prelude::ThreadRng;

struct FiniteField {
    order: BigUint,
}

impl FiniteField {
    pub(crate) fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        FiniteField {
            order: prime::random_prime(sec_param, rng),
        }
    }
}

struct FiniteFieldElement {
    number: BigUint,
    field: Rc<FiniteField>
}

impl FiniteFieldElement {
    pub(crate) fn zero(g: &FiniteField) -> Self {
        FiniteFieldElement {
            number: Zero::zero(),
            field: g.clone()
        }
    }

    pub(crate) fn one(g: &FiniteField) -> Self {
        FiniteFieldElement {
            number: One::one(),
            field: g.clone()
        }
    }

    pub(crate) fn add(&self, b: &Self) -> Self {
        if self.field != b.field {
            panic!("adding {:?} and {:?} did't work they have differnt fields", self, b);
        }
        FiniteFieldElement {
            number: &(&self.number + &b.number) % &self.field.order,
            field: self.field.clone()
        }
    }

    pub(crate) fn mult(&self, b: &Self) -> Self {
        if self.field != b.field {
            panic!("multiplying {:?} and {:?} did't work they have differnt fields", self, b);
        }
        FiniteFieldElement {
            number: &(&self.number * &b.number) % &self.field.order,
            field: self.field.clone()
        }
    }

    pub(crate) fn pow(&self, b: &Self) -> Self {
        if self.field != b.field {
            panic!("raising {:?} to the power of {:?} did't work they have differnt fields", self, b);
        }
        FiniteFieldElement {
            number: self.number.modpow(&b.number, &self.field.order),
            field: self.field.clone()
        }
    }
}