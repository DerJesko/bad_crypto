use crate::matrix::Matrix;
use crate::ring::Ring;
use crate::small_prime::random_prime_in_range;
use crate::{traits, num_bits, chi};
use ndarray::{Array, ShapeBuilder};
use rand::prelude::*;
use std::rc::Rc;

fn G(index: usize,l:usize ,k: usize, ring: Rc<Ring>) -> Matrix {
    let mut a = Array::zeros((l,k));
    for i in 0..k {
        a[[index, i]] = (2usize).pow(i as u32);
    }
    Matrix::new(a, ring)
}

fn g_inverse(y: usize, k: usize, ring: Rc<Ring>) -> Matrix {
    let mut number = y;
    let mut a = Array::zeros((1,k));
    for i in 0..k {
        a[[1,i]] = number % 2;
        number = number / 2;
    }
    Matrix::new(a, ring)
}

pub struct PackedRegev();

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct PublicKey {
    m: usize,
    n: usize,
    A: Matrix,
    b: Matrix,
    l: usize,
    k: usize,
    ring: Rc<Ring>,
}

#[derive(Debug)]
pub struct Ciphertext(Matrix, Matrix);

#[derive(PartialEq, Debug)]
pub struct Message(pub Vec<bool>);

pub struct SecretKey(Matrix, PublicKey);

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for PackedRegev {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let n = sec_param * 50;
        let q = random_prime_in_range(sec_param, n * n, 2 * n * n, rng) * 2;
        let k = num_bits(q);
        let m = 2 * sec_param + (n + 1) * k;
        let ring = Rc::new(Ring::new(q));
        let distribution_limit = 1; // TODO tmp
        let l = sec_param; // TODO
        #[allow(non_snake_case)]
        let A = Matrix::rand_new_of_shape(m, n, ring.clone(), rng);
        let s = Matrix::rand_new_of_shape(n, l, ring.clone(), rng);
        let e = Matrix::new(
            Array::from_shape_fn((l, m).f(), |_| {
                (chi(distribution_limit as u64, rng) + distribution_limit as isize) as usize
            }),
            ring.clone(),
        );
        let b = Matrix::dot(&s, &A) + e;
        let pk = PublicKey { m, n, A, b, l, k, ring };
        (pk.clone(), SecretKey(s, pk))
    }

    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        let Message(m) = message;
        let r = Matrix::new(
            Array::from_shape_fn((pub_key.m, pub_key.k).f(), |_| if rng.gen() { 1 } else { 0 }),
            pub_key.ring.clone(),
        );
        let c1 = Matrix::dot(&pub_key.A, &r);
        let mut c2 = Matrix::dot(&pub_key.b, &r);
        println!("l: {}", pub_key.l);
        for i in 0..pub_key.l {
            if m[i]{
            c2 = c2 + G(i,pub_key.l, pub_key.k, pub_key.ring.clone());
            }
            }
            Ciphertext(c1, c2)
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let SecretKey(s,pk) =sec_key;
        let decryption = c2 - &Matrix::dot(&s, &c1);
        println!("v: {:?}", decryption);
        let v = decryption.to_vec().unwrap().iter().map(|x| ((*x as isize) - pk.ring.order as isize / 2).abs() < pk.ring.order as isize / 4).collect();
        Some(Message(v))
    }
}