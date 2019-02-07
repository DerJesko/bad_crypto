use crate::fields::FiniteFieldElement;
use crate::traits;
use ndarray::Array2;
use num_bigint::{BigUint, RandBigInt};
use rand::distributions::StandardNormal;
use rand::prelude::*;
use rand::SeedableRng;

const M: usize = 25;
const N: usize = 5;
const B: f64 = 2.;

fn chi(rng: &mut ThreadRng) -> f64 {
    loop {
        let r = rng.sample(StandardNormal);
        if r.abs() < B {
            return r;
        }
    }
}

pub struct Regev();

pub struct PublicKey {
    A: ndarray::Array2<FiniteFieldElement>,
    b: ndarray::Array2<FiniteFieldElement>,
}

pub struct Ciphertext(
    ndarray::Array2<FiniteFieldElement>,
    ndarray::Array2<FiniteFieldElement>,
);

pub struct Message(bool);

pub struct SecretKey(ndarray::Array2<FiniteFieldElement>);

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for Regev {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        panic!();
    }
    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        panic!();
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        rng: &mut ThreadRng,
    ) -> Option<Message> {
        panic!();
    }
}
