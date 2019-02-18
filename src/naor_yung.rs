use crate::dummy_nikz::{DummyCRS, DummyNIZK, DummyProof, DummyWitness, DummyWord};
use crate::elgamal;
use crate::traits::{NIZKProof, PubKEncryption};
use rand::prelude::ThreadRng;

pub struct NaorYung();

#[derive(Debug, Clone)]
struct PublicKey {
    crs: DummyCRS,
    pk1: elgamal::PublicKey,
    pk2: elgamal::PublicKey,
}

#[derive(Debug, Clone)]
struct SecretKey {
    pk: PublicKey,
    sk1: elgamal::SecretKey,
}

#[derive(Debug, Clone)]
struct Message(elgamal::Message);

#[derive(Debug, Clone)]
struct Ciphertext(elgamal::Ciphertext, elgamal::Ciphertext, DummyProof);

impl PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for NaorYung {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let (pk1, sk1) = elgamal::ElGamal::key_generation(sec_param, rng);
        let (pk2, _) = elgamal::ElGamal::key_generation(sec_param, rng);
        let crs = DummyNIZK::crs_generation(sec_param, rng);
        (
            PublicKey {
                crs: crs.clone(),
                pk1: pk1.clone(),
                pk2: pk2.clone(),
            },
            SecretKey {
                pk: PublicKey { crs, pk1, pk2 },
                sk1,
            },
        )
    }
    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        let Message(m) = message;
        Ciphertext(
            elgamal::ElGamal::encrypt(&pub_key.pk1, &m, rng),
            elgamal::ElGamal::encrypt(&pub_key.pk2, &m, rng),
            DummyNIZK::prove(&pub_key.crs, &DummyWord(), &DummyWitness(), rng),
        )
    }
    fn decrypt(
        sec_key: &SecretKey,
        ciphertext: &Ciphertext,
        rng: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2, pi) = ciphertext;
        if !DummyNIZK::verify(&sec_key.pk.crs, &DummyWord(), pi, rng) {
            return None;
        }
        Some(Message(
            elgamal::ElGamal::decrypt(&sec_key.sk1, c1, rng).unwrap(),
        ))
    }
}
