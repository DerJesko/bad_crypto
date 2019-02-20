use crate::prime::{extended_gcd, lcm, random_prime};
use crate::traits;
use num_bigint::{BigUint, ToBigInt};
use rand::prelude::ThreadRng;

pub struct RSA();

#[derive(Debug, PartialEq)]
pub struct Message(pub BigUint);
pub struct Ciphertext(BigUint);
#[derive(Clone, Debug)]
pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}
pub struct SecretKey(pub BigUint, PublicKey);

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for RSA {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let p = random_prime(sec_param, rng);
        let q = random_prime(sec_param, rng);
        let n = &p * &q;
        let lambda_n = lcm(&(p - (1 as u8)), &(q - (1 as u8))).to_bigint().unwrap();
        let e: BigUint = BigUint::from(1009 as u16);
        let ((e_inverse, _), _) = extended_gcd(e.to_bigint().unwrap(), lambda_n.clone());
        let e_inverse_pos = (e_inverse + &lambda_n) % &lambda_n;
        let pk = PublicKey { n, e };
        (
            pk.clone(),
            SecretKey(e_inverse_pos.to_biguint().unwrap(), pk),
        )
    }
    fn encrypt(pub_key: &PublicKey, message: &Message, _rng: &mut ThreadRng) -> Ciphertext {
        let Message(m) = message;
        Ciphertext(m.modpow(&pub_key.e, &pub_key.n))
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _rng: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c) = cipher_text;
        Some(Message(c.modpow(&sec_key.0, &sec_key.1.n)))
    }
}
