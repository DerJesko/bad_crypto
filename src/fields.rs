use crate::prime;
use bigdecimal::BigDecimal;
use num_bigint::{RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Field {
    pub order: BigDecimal,
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field {{ order: {} }}", self.order)
    }
}

impl Field {
    fn is_zero(&self) -> bool {
        self.order.is_zero()
    }

    fn zero() -> Self {
        Field {
            order: BigDecimal::zero(),
        }
    }
}

impl Field {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        Field {
            order: BigDecimal::from((prime::random_prime(sec_param, rng).to_bigint().unwrap(), 0)),
        }
    }
}

#[derive(Clone)]
pub struct FiniteFieldElement {
    pub number: BigDecimal,
    field: Rc<Field>,
}

impl fmt::Debug for FiniteFieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement {{ number: {}, fieldorder: {} }}",
            self.number, self.field.order
        )
    }
}

impl fmt::Display for FiniteFieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mod {}", self.number, self.field.order)
    }
}

impl Add for &FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn add(self, b: Self) -> FiniteFieldElement {
        if self.field != b.field {
            if Zero::is_zero(&self.field.order) {
                return FiniteFieldElement {
                    number: &(&self.number + &b.number) % &b.field.order,
                    field: b.field.clone(),
                };
            }
            if Zero::is_zero(&b.field.order) {
                return FiniteFieldElement {
                    number: &(&self.number + &b.number) % &self.field.order,
                    field: self.field.clone(),
                };
            }
            panic!(
                "adding {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        if Zero::is_zero(&self.field.order) {
            return FiniteFieldElement {
                number: &self.number + &b.number,
                field: self.field.clone(),
            };
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
        if self.field != b.field {
            if Zero::is_zero(&self.field.order) {
                return FiniteFieldElement {
                    number: &(&self.number + &b.number) % &b.field.order,
                    field: b.field,
                };
            }
            if Zero::is_zero(&b.field.order) {
                return FiniteFieldElement {
                    number: &(&self.number + &b.number) % &self.field.order,
                    field: self.field,
                };
            }
            panic!(
                "adding {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        if Zero::is_zero(&self.field.order) {
            return FiniteFieldElement {
                number: &self.number + &b.number,
                field: self.field,
            };
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
            number: (&(&self.number - &b.number) + &self.field.order) % &self.field.order,
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
            field: Rc::new(Field::zero()),
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
            field: Rc::new(Field::zero()),
        }
    }
}

impl FiniteFieldElement {
    pub fn rand_new(field: &Rc<Field>, rng: &mut ThreadRng) -> FiniteFieldElement {
        let r = rng.gen_bigint_range(&Zero::zero(), &field.order.to_bigint().unwrap());
        Self::new(BigDecimal::from((r, 0)), &field)
    }

    pub fn new(number: BigDecimal, field: &Rc<Field>) -> FiniteFieldElement {
        FiniteFieldElement {
            number: &number % &field.order,
            field: field.clone(),
        }
    }
}
