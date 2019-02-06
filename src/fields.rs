use crate::prime;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct FiniteField {
    order: BigUint,
}

impl FiniteField {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        FiniteField {
            order: prime::random_prime(sec_param, rng),
        }
    }
}

#[derive(Debug)]
pub struct FiniteFieldElement {
    number: BigUint,
    field: Rc<FiniteField>,
}

impl FiniteFieldElement {
    pub fn zero(g: &Rc<FiniteField>) -> Self {
        FiniteFieldElement {
            number: Zero::zero(),
            field: g.clone(),
        }
    }

    pub fn one(g: &Rc<FiniteField>) -> Self {
        FiniteFieldElement {
            number: One::one(),
            field: g.clone(),
        }
    }

    pub fn add(&self, b: &Self) -> Self {
        if self.field != b.field {
            panic!(
                "adding {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        FiniteFieldElement {
            number: &(&self.number + &b.number) % &self.field.order,
            field: self.field.clone(),
        }
    }

    pub fn mult(&self, b: &Self) -> Self {
        if self.field != b.field {
            panic!(
                "multiplying {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        FiniteFieldElement {
            number: &(&self.number * &b.number) % &self.field.order,
            field: self.field.clone(),
        }
    }

    pub fn pow(&self, b: &Self) -> Self {
        if self.field != b.field {
            panic!(
                "raising {:?} to the power of {:?} did't work they have differnt fields",
                self, b
            );
        }
        FiniteFieldElement {
            number: self.number.modpow(&b.number, &self.field.order),
            field: self.field.clone(),
        }
    }
}
