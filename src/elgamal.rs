use crate::groups::{PrimeGroup, PrimeGroupElement};
use crate::{traits, TWO};
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::prelude::ThreadRng;
use std::rc::Rc;

pub struct ElGamal();

#[derive(Debug, Clone, PartialEq)]
pub struct Message(PrimeGroupElement);

impl Message {
    pub fn new(m: PrimeGroupElement) -> Self {
        Message(m)
    }
}

#[derive(Debug, Clone)]
pub struct Ciphertext(PrimeGroupElement, PrimeGroupElement);

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub group: PrimeGroup,
    generator_g: PrimeGroupElement,
    generator_h: PrimeGroupElement,
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
            pub_key.generator_g.pow(&exponent),
            (pub_key.generator_h.pow(&exponent)).mult(m),
        )
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _rng: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let group = &sec_key.pk.group;
        Some(Message(
            (c1.pow(&group.exp_inverse(&sec_key.exponent))).mult(&c2),
        ))
    }

    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let group = PrimeGroup::rand_new(sec_param, rng);
        let generator_g = PrimeGroupElement::rand_generator(Rc::new(group.clone()), rng);
        let exponent = rng.gen_biguint_range(&One::one(), &group.modulus);
        let generator_h = generator_g.pow(&exponent);
        let pk = PublicKey {
            group,
            generator_g,
            generator_h,
        };
        (pk.clone(), SecretKey { pk, exponent })
    }
}
