use crate::prime;
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct FiniteField {
    pub order: BigDecimal,
}

impl FiniteField {
    fn is_zero(&self) -> bool {
        self.order.is_zero()
    }

    fn zero() -> Self {
        FiniteField {
            order: BigDecimal::zero(),
        }
    }
}

impl FiniteField {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        FiniteField {
            order: BigDecimal::from((prime::random_prime(sec_param, rng).to_bigint().unwrap(), 0)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FiniteFieldElement {
    pub number: BigDecimal,
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
        if self.field != b.field && !(b.field.is_zero()) {
            panic!(
                "adding {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        FiniteFieldElement {
            number: &(&self.number + &b.number) % &self.field.order,
            field: self.field,
        }
    }
}

impl Sub for &FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn sub(self, b: Self) -> FiniteFieldElement {
        if self.field != b.field && !(b.field.is_zero()) {
            panic!(
                "substracting {:?} from {:?} did't work they have differnt fields",
                b, self
            );
        }
        FiniteFieldElement {
            number: &(&self.number - &b.number) % &self.field.order,
            field: self.field.clone(),
        }
    }
}

impl Sub for FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn sub(self, b: Self) -> FiniteFieldElement {
        if self.field != b.field && !(b.field.is_zero()) {
            panic!(
                "substracting {:?} from {:?} did't work they have differnt fields",
                b, self
            );
        }
        FiniteFieldElement {
            number: &(&self.number - &b.number) % &self.field.order,
            field: self.field,
        }
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
        if self.field != b.field && !(b.field.is_zero()) {
            panic!(
                "multiplying {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        FiniteFieldElement {
            number: &(&self.number * &b.number) % &self.field.order,
            field: self.field,
        }
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
    pub fn rand_new(field: &Rc<FiniteField>, rng: &mut ThreadRng) -> FiniteFieldElement {
        let r = rng.gen_bigint_range(&Zero::zero(), &field.order.to_bigint().unwrap());
        Self::new(BigDecimal::from((r, 0)), &field)
    }

    pub fn new(number: BigDecimal, field: &Rc<FiniteField>) -> FiniteFieldElement {
        FiniteFieldElement {
            number: &number % &field.order,
            field: field.clone(),
        }
    }
}
