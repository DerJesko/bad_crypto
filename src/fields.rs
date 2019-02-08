use crate::prime;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct FiniteField {
    order: BigUint,
}

impl FiniteField {
    fn is_zero(&self) -> bool {
        self.order.is_zero()
    }

    fn zero() -> Self {
        FiniteField {
            order: BigUint::zero(),
        }
    }
}

impl FiniteField {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        FiniteField {
            order: prime::random_prime(sec_param, rng),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FiniteFieldElement {
    number: BigUint,
    field: Rc<FiniteField>,
}

impl Add for &FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn add(self, b: Self) -> FiniteFieldElement {
        if self.field != b.field && !(b.field.is_zero()) {
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
}

impl Add for FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn add(self, b: Self) -> FiniteFieldElement {
        &self + &b
    }
}

impl Zero for FiniteFieldElement {
    fn is_zero(&self) -> bool {
        self.number.is_zero()
    }

    fn zero() -> FiniteFieldElement {
        FiniteFieldElement {
            number: Zero::zero(),
            field: Rc::new(FiniteField::zero()),
        }
    }
}

impl Mul for &FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn mul(self, b: Self) -> FiniteFieldElement {
        if self.field != b.field && !(b.field.is_zero()) {
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
}

impl Mul for FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn mul(self, b: Self) -> FiniteFieldElement {
        &self + &b
    }
}

impl One for FiniteFieldElement {
    fn is_one(&self) -> bool {
        self.number.is_one()
    }

    fn one() -> FiniteFieldElement {
        FiniteFieldElement {
            number: One::one(),
            field: Rc::new(FiniteField::zero()),
        }
    }
}

impl FiniteFieldElement {
    pub fn pow(&self, b: &BigUint) -> Self {
        FiniteFieldElement {
            number: self.number.modpow(b, &self.field.order),
            field: self.field.clone(),
        }
    }
}
