use crate::prime::{lcm, random_prime};
use crate::traits;
use num_bigint::BigUint;
use rand::prelude::ThreadRng;

pub struct RSA();

struct Message();
struct Ciphertext();
struct PublicKey();
struct SecretKey();

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for RSA {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let p = random_prime(sec_param, rng);
        let q = random_prime(sec_param, rng);
        let n = &p * &q;
        let lambda_n = lcm(&(p - (1 as u8)), &(q - (1 as u8)));
        let e: BigUint = BigUint::from(1009 as u16);
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
