use crate::matrix::Matrix;
use crate::ring::Ring;
use crate::small_prime::random_prime_in_range;
use crate::{traits, num_bits, chi};
use ndarray::{Array, ShapeBuilder};
use rand::prelude::*;
use std::rc::Rc;

pub struct Regev();

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct PublicKey {
    m: usize,
    n: usize,
    A: Matrix,
    b: Matrix,
    field: Rc<Ring>,
}

#[derive(Debug)]
pub struct Ciphertext(Matrix, Matrix);

#[derive(PartialEq, Debug)]
pub struct Message(pub bool);

pub struct SecretKey(Matrix, PublicKey);

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for Regev {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let n = sec_param * 50;
        let q = random_prime_in_range(sec_param, n * n, 2 * n * n, rng);
        let m = 2 * sec_param + (n + 1) * num_bits(q);
        let field = Rc::new(Ring::new(q));
        let distribution_limit = (q / (4 * m)) - 1;
        #[allow(non_snake_case)]
        let A = Matrix::rand_new_of_shape(n, m, field.clone(), rng);
        let s = Matrix::rand_new_of_shape(1, n, field.clone(), rng);
        let e = Matrix::new(
            Array::from_shape_fn((m, 1).f(), |_| {
                (chi(distribution_limit as u64, rng) + distribution_limit as isize) as usize
            }),
            field.clone(),
        );
        let b = Matrix::dot(&A, &s) + e;
        let pk = PublicKey { m, n, A, b, field };
        (pk.clone(), SecretKey(s, pk))
    }

    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        let Message(mu) = message;
        let x = Matrix::new(
            Array::from_shape_fn((1, pub_key.m).f(), |_| if rng.gen() { 1 } else { 0 }),
            pub_key.field.clone(),
        );
        let enc_mu = Matrix::new(
            Array::from_shape_fn((1, 1).f(), |_| &pub_key.field.order / 2 * *mu as usize),
            pub_key.field.clone(),
        );
        Ciphertext(
            Matrix::dot(&x, &pub_key.A),
            Matrix::dot(&x, &pub_key.b) + enc_mu,
        )
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let SecretKey(s, pk) = sec_key;
        let z = (c2 - &Matrix::dot(&c1, &s)).to_number().unwrap() as isize
            - pk.field.order as isize / 2;
        Some(Message(z.abs() < pk.field.order as isize / 4))
    }
}
