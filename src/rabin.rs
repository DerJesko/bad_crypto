use crate::prime::{extended_gcd, lcm, random_prime};
use crate::traits;
use crate::TWO;
use num_bigint::{BigUint, ToBigInt};
use rand::prelude::ThreadRng;

pub struct Rabin();

#[derive(Debug, PartialEq)]
pub struct Message(pub BigUint);
pub struct Ciphertext(BigUint);
#[derive(Clone, Debug)]
pub struct PublicKey(pub BigUint);
pub struct SecretKey(pub BigUint, pub BigUint, PublicKey);

//WRONG

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for Rabin {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let p = random_prime(sec_param, rng);
        let q = random_prime(sec_param, rng);
        let n = &p * &q;
        let lambda_n = lcm(&(&p - (1 as u8)), &(&q - (1 as u8)))
            .to_bigint()
            .unwrap();
        let e: BigUint = BigUint::from(1009 as u16);
        let ((e_inverse, _), _) = extended_gcd(e.to_bigint().unwrap(), lambda_n.clone());
        let e_inverse_pos = (e_inverse + &lambda_n) % &lambda_n;
        let pk = PublicKey(n);
        (pk.clone(), SecretKey(p, q, pk))
    }
    fn encrypt(pub_key: &PublicKey, message: &Message, _rng: &mut ThreadRng) -> Ciphertext {
        let Message(m) = message;
        Ciphertext(m.modpow(&TWO(), &pub_key.0))
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _rng: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c) = cipher_text;
        //Some(Message(c.modpow(&sec_key.0, &sec_key.1.n)))
        None
    }
}
