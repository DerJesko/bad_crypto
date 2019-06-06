use crate::prime;
use num_traits::{ToPrimitive, Zero};
use rand::prelude::ThreadRng;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Field {
    pub order: f64,
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field {{ order: {} }}", self.order)
    }
}

impl Field {
    pub fn rand_new(rng: &mut ThreadRng) -> Self {
        Field {
            order: prime::random_prime(20, rng)
                .to_f64()
                .expect("the security parameter is too big"),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.order.is_zero()
    }

    pub fn zero() -> Self {
        Field { order: 0. }
    }

    pub fn unify(f1: &Rc<Field>, f2: &Rc<Field>) -> Option<Rc<Field>> {
        if f1 != f2 {
            if f1.is_zero() {
                return Some(f2.clone());
            }
            if f2.is_zero() {
                return Some(f1.clone());
            }
            return None;
        }
        return Some(f1.clone());
    }
}
