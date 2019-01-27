use crate::{traits, TWO};
use crate::groups::{FiniteGroup, PrimeGroup};
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::prelude::ThreadRng;

pub struct ElGamal();

#[derive(Debug, Clone, PartialEq)]
pub struct Message(pub BigUint);

#[derive(Debug, Clone)]
pub struct Ciphertext(BigUint, BigUint);

#[derive(Debug, Clone)]
pub struct PublicKey {
    group: PrimeGroup,
    generator_g: BigUint,
    generator_h: BigUint,
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pk: PublicKey,
    exponent: BigUint,
}

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for ElGamal {
    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        let group = &pub_key.group;
        let exponent = rng.gen_biguint_range(&One::one(), &group.modulus);
        let Message(m) = message;
        Ciphertext(
            group.pow(&pub_key.generator_g, &exponent),
            group.mult(&group.pow(&pub_key.generator_h, &exponent), m),
        )
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _rng: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let group = &sec_key.pk.group;
        Some(Message(group.mult(
            &group.pow(&c1, &group.exp_inverse(&sec_key.exponent)),
            &c2,
        )))
    }

    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let group = PrimeGroup::rand_new(sec_param, rng);
        let generator_g = rng.gen_biguint_range(&TWO(), &group.modulus);
        let exponent = rng.gen_biguint_range(&One::one(), &group.modulus);
        let generator_h = group.pow(&generator_g, &exponent);
        let pk = PublicKey {
            group,
            generator_g,
            generator_h,
        };
        (pk.clone(), SecretKey { pk, exponent })
    }
}
