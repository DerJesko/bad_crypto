use crate::prime::{extended_gcd, random_prime};
use crate::traits;
use crate::TWO;
use num_bigint::ToBigUint;
use num_bigint::{BigUint, ToBigInt};
use rand::prelude::ThreadRng;

pub struct Rabin();

#[derive(Debug, PartialEq)]
pub struct Message(pub BigUint);
pub struct Ciphertext(BigUint);
#[derive(Clone, Debug)]
pub struct PublicKey(pub BigUint);
pub struct SecretKey(
    pub BigUint,
    pub BigUint,
    pub BigUint,
    pub BigUint,
    PublicKey,
);

fn big(x: usize) -> BigUint {
    x.to_biguint().unwrap()
}

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for Rabin {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let mut p;
        loop {
            p = random_prime(sec_param, rng);
            if &p % big(4) == big(3) {
                break;
            }
        }
        let mut q;
        loop {
            q = random_prime(sec_param, rng);
            if &q % big(4) == big(3) {
                break;
            }
        }
        let n = &p * &q;
        let ((bezout_p, bezout_q), _) =
            extended_gcd(p.to_bigint().unwrap(), q.to_bigint().unwrap());
        let pk = PublicKey(n);
        (
            pk.clone(),
            SecretKey(
                p,
                q,
                bezout_p.to_biguint().unwrap(),
                bezout_q.to_biguint().unwrap(),
                pk,
            ),
        )
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
        let SecretKey(p, q, bezout_p, bezout_q, PublicKey(n)) = sec_key;
        let m_p = c.modpow(&((p + big(1)) / big(4)), p);
        let m_q = c.modpow(&((q + big(1)) / big(4)), q);
        let r1 = (p * bezout_p * m_p + q * bezout_q * m_q) % n;
        let r2 = n - r1;
        let r3 = (p * bezout_p * m_p - q * bezout_q * m_q) % n;
        let r4 = n - r1;
        let println!("{:?},{:?},{:?},{:?}", r1, r2, r3, r4);
        //Some(Message(c.modpow(&sec_key.0, &sec_key.1.n)))
        None
    }
}
