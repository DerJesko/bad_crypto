use crate::ring;
use ndarray::{Array, Array2, Axis, ShapeBuilder};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::fmt;
use std::ops::{Add, Sub};
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
            m: Array::from_shape_fn((columns, rows).f(), |_| rng.gen_range(0, field.order)),
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
        let ring = match ring::Ring::unify(&self.field, &b.field) {
            Some(f) => f,
            None => panic!(
                "Failed to add {:?} and {:?} due to using different fields",
                self, b
            ),
        };
        Matrix {
            m: ((&self.m + ring.order) - &b.m) % ring.order,
            field: ring,
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
                m: result % f.order,
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

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.m)
    }
}

impl Matrix {
    pub fn dot(&self, b: &Matrix) -> Matrix {
        let result = self.m.dot(&b.m);
        match ring::Ring::unify(&self.field, &b.field) {
            Some(f) => Matrix {
                m: result % f.order,
                field: f,
            },
            None => panic!(
                "Matix multiply {:?} and {:?} due to using different fields",
                self, b
            ),
        }
    }

    pub fn to_number(&self) -> Option<usize> {
        if self.m.len() == 1 {
            return Some(self.m[[0, 0]]);
        }
        None
    }

    pub fn to_vec(&self) -> Option<Vec<usize>> {
        if self.m.len_of(Axis(0)) == 1 {
            let mut v = vec![];
            for i in 0..self.m.len_of(Axis(1)) {
                v.push(self.m[[0, i]]);
            }
            return Some(v);
        }
        None
    }
}
