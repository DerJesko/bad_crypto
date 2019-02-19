use crate::prime::{extended_gcd, lcm, random_prime};
use crate::traits;
use num_bigint::BigUint;
use rand::prelude::ThreadRng;

pub struct RSA();

struct Message();
struct Ciphertext();
#[derive(Clone)]
struct PublicKey {
    n: BigUint,
    e: BigUint,
}
struct SecretKey(BigUint, PublicKey);

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for RSA {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let p = random_prime(sec_param, rng);
        let q = random_prime(sec_param, rng);
        let n = &p * &q;
        let lambda_n = lcm(&(p - (1 as u8)), &(q - (1 as u8)));
        let e: BigUint = BigUint::from(1009 as u16);
        let ((d, _), _) = extended_gcd(e.clone(), lambda_n);
        let pk = PublicKey { n, e };

        (pk.clone(), SecretKey(d, pk))
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
