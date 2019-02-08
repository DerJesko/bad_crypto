use crate::groups::{PrimeGroup, PrimeGroupElement};
use crate::traits;
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
    pub generator_g: PrimeGroupElement,
    generator_h: PrimeGroupElement,
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pk: PublicKey,
    exponent: BigUint,
}

pub struct Function();

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for ElGamal {
    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        let group = &pub_key.generator_g.group;
        let exponent = rng.gen_biguint_range(&One::one(), &group.modulus);
        let Message(m) = message;
        Ciphertext(
            pub_key.generator_g.pow(&exponent),
            &(pub_key.generator_h.pow(&exponent)) * m,
        )
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _rng: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let group = &sec_key.pk.generator_g.group;
        Some(Message(
            &(c1.pow(&group.exp_inverse(&sec_key.exponent))) * &c2,
        ))
    }

    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let group = Rc::new(PrimeGroup::rand_new(sec_param, rng));
        let generator_g = PrimeGroupElement::rand_generator(&group, rng);
        let exponent = rng.gen_biguint_range(&One::one(), &group.modulus);
        let generator_h = generator_g.pow(&exponent);
        let pk = PublicKey {
            generator_g,
            generator_h,
        };
        (pk.clone(), SecretKey { pk, exponent })
    }
}

impl traits::HomomorphEncryption<PublicKey, SecretKey, Message, Ciphertext, Function> for ElGamal {
    // let m1 and and m2 be messages (group elements) this function can evaluate m1*m2
    fn eval(pubk: &PublicKey, _: Function, c: Vec<Ciphertext>, _: &mut ThreadRng) -> Ciphertext {
        let mut r1 = PrimeGroupElement::one(&pubk.generator_g.group);
        let mut r2 = PrimeGroupElement::one(&pubk.generator_g.group);
        for i in c {
            let Ciphertext(c1, c2) = i;
            r1 = &r1 * &c1;
            r2 = &r2 * &c2;
        }
        Ciphertext(r1, r2)
    }
}
