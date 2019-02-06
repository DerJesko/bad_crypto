use crate::fields::FiniteFieldElement;
use ndarray::Array2;
use num_bigint::{BigUint, RandBigInt};

pub struct Matrix(pub Array2<FiniteFieldElement>);

impl Matrix {
    pub fn add(&self, b: &Matrix) -> Matrix {
        let Matrix(x) = self;
        let Matrix(y) = b;
        //assert!(x.len_of() == y.len_of());
        //assert!(x.len_of() == y.len_of());
        panic!();
    }

    pub fn mult(&self, b: &Matrix) -> Matrix {
        panic!();
    }
}
