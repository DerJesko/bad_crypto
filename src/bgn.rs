use crate::groups::DoublePrimeGroup;
use crate::traits::HomomorphEncryption;
use num_bigint::{BigUint, RandBigInt};
use rand::prelude::ThreadRng;

pub struct BGN();

pub struct SecretKey(BigUint);
pub struct PublicKey {
    group: DoublePrimeGroup,
    generator: BigUint,
    h: BigUint,
}
pub struct Message(bool);
pub struct Ciphertext(BigUint);
pub struct Function {
    alpha: Vec<Vec<BigUint>>,
    beta: Vec<BigUint>,
    gamma: BigUint,
}

impl HomomorphEncryption<PublicKey, SecretKey, Message, Ciphertext, Function> for BGN {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        panic!();
    }
    fn encrypt(pubkey: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        panic!();
    }
    fn eval(
        pub_key: &PublicKey,
        function: Function,
        ciphertexts: Vec<Ciphertext>,
        rng: &mut ThreadRng,
    ) -> Ciphertext {
        panic!();
    }

    fn decrypt(
        seckey: &SecretKey,
        cipher_text: &Ciphertext,
        rng: &mut ThreadRng,
    ) -> Option<Message> {
        panic!();
    }
}
