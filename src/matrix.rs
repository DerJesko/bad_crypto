use crate::ring;
use ndarray::{Array, Array2, ShapeBuilder};
use num_traits::Zero;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(Clone)]
pub struct Matrix {
    m: Array2<usize>,
    field: Rc<ring::Ring>,
}

impl Matrix {
    pub fn new(matrix: Array2<usize>, field: Rc<ring::Ring>) -> Self {
        if field.is_zero() {
            Matrix { m: matrix, field }
        } else {
            Matrix {
                m: matrix % field.order,
                field,
            }
        }
    }

    pub fn rand_new_of_shape(
        rows: usize,
        columns: usize,
        field: Rc<ring::Ring>,
        rng: &mut ThreadRng,
    ) -> Self {
        Matrix {
            m: Array::from_shape_fn((columns, rows).f(), |_| rng.gen_range(0, &field.order)),
            field: field.clone(),
        }
    }
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

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, b: Self) -> Matrix {
        let result = &self.m - &b.m; // TODO still wrong
        match ring::Ring::unify(&self.field, &b.field) {
            Some(f) => Matrix {
                m: result,
                field: f,
            },
            None => panic!(
                "Failed to add {:?} and {:?} due to using different fields",
                self, b
            ),
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, b: Self) -> Matrix {
        &self - &b
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, b: Self) -> Matrix {
        let result = &self.m + &b.m;
        match ring::Ring::unify(&self.field, &b.field) {
            Some(f) => Matrix {
                m: result,
                field: f,
            },
            None => panic!(
                "Failed to add {:?} and {:?} due to using different fields",
                self, b
            ),
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, b: Self) -> Matrix {
        &self + &b
    }
}

impl Matrix {
    pub fn dot(&self, b: &Matrix) -> Matrix {
        let result = self.m.dot(&b.m);
        match ring::Ring::unify(&self.field, &b.field) {
            Some(f) => Matrix {
                m: result,
                field: f,
            },
            None => panic!(
                "Matix multiply {:?} and {:?} due to using different fields",
                self, b
            ),
        }
    }

    pub fn to_number(&self) -> Option<usize> {
        if self.m.len() == 0 {
            return Some(self.m[[0, 0]]);
        }
        None
    }
}
