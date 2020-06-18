use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::{Distribution, Uniform};
use rand_distr::Binomial;

pub struct BoundedFiniteGauss {
    bound: usize,
    modulus: usize,
}

impl BoundedFiniteGauss {
    pub fn new(bound: usize, modulus: usize) -> Self {
        if bound > modulus {
            panic!();
        }
        BoundedFiniteGauss { bound, modulus }
    }
}

impl Distribution<usize> for BoundedFiniteGauss {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        (Binomial::new((2 * self.bound) as u64, 0.5)
            .unwrap()
            .sample(rng)) as usize
            + self.modulus
            - self.bound
    }
}

pub struct SmallFlat {
    size: usize,
    modulus: usize,
}

impl SmallFlat {
    pub fn new(size: usize, modulus: usize) -> Self {
        if size > modulus {
            panic!();
        }
        SmallFlat { size, modulus }
    }
}

impl Distribution<usize> for SmallFlat {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        Uniform::new(0, 2 * self.size + 1).sample(rng) + self.modulus - self.size
    }
}
