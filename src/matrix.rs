use crate::fields::FiniteFieldElement;
use ndarray::{Array, Array2, ShapeBuilder};
use num_traits::Zero;

pub fn dot(
    a: &Array2<FiniteFieldElement>,
    b: &Array2<FiniteFieldElement>,
) -> Array2<FiniteFieldElement> {
    Array::from_shape_fn((a.shape()[0], b.shape()[1]).f(), |(i, j)| {
        let mut res = Zero::zero();
        for l in 0..a.shape()[1] {
            res = res + &a[[i, l]] * &b[[l, j]];
        }
        res
    })
}
