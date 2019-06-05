use crate::fields;
use ndarray::{Array, Array2, ShapeBuilder};
use num_traits::Zero;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(Clone)]
pub struct Matrix {
    m: Array2<f64>,
    field: Rc<fields::Field>,
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix {{ matrix: {}, fieldorder: {} }}",
            self.m, self.field.order
        )
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, b: Self) -> Matrix {
        if self.field != b.field {
            if Zero::is_zero(&self.field.order) {
                return Matrix {
                    m: &(&self.m + &b.m) % b.field.order,
                    field: b.field.clone(),
                };
            }
            if Zero::is_zero(&b.field.order) {
                return Matrix {
                    m: &(&self.m + &b.m) % self.field.order,
                    field: self.field.clone(),
                };
            }
            panic!(
                "adding {:?} and {:?} did't work they have differnt fields",
                self, b
            );
        }
        if Zero::is_zero(&self.field.order) {
            return Matrix {
                m: &self.m + &b.m,
                field: self.field.clone(),
            };
        }
        Matrix {
            m: &(&self.m + &b.m) % self.field.order,
            field: self.field.clone(),
        }
    }
}
