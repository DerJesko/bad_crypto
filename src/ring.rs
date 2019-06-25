use num_traits::Zero;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Ring {
    pub order: usize,
}

impl fmt::Debug for Ring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field {{ order: {} }}", self.order)
    }
}

impl Ring {
    pub fn new(order: usize) -> Self {
        Ring { order }
    }

    pub fn is_zero(&self) -> bool {
        self.order.is_zero()
    }

    pub fn zero() -> Self {
        Ring { order: 0 }
    }

    pub fn unify(f1: &Rc<Ring>, f2: &Rc<Ring>) -> Option<Rc<Ring>> {
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
