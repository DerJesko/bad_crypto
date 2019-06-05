use crate::prime;
use num_traits::{ToPrimitive, Zero};
use rand::prelude::ThreadRng;
use std::fmt;

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
    fn is_zero(&self) -> bool {
        self.order.is_zero()
    }

    fn zero() -> Self {
        Field { order: 0. }
    }
}

impl Field {
    pub fn rand_new(sec_param: usize, rng: &mut ThreadRng) -> Self {
        Field {
            order: prime::random_prime(sec_param, rng)
                .to_f64()
                .expect("the security parameter is too big"),
        }
    }
}
