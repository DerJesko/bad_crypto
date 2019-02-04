use crate::elgamal;
use crate::groups::PrimeGroupElement;
use crate::prime::random_prime;
use crate::traits::PubKEncryption;
use num_bigint::ToBigUint;
use rand;
use std::rc::Rc;

#[test]
fn gen_prime() {
    let mut rng = rand::thread_rng();
    println!("A prime: {}", random_prime(13, &mut rng));
}
#[test]
fn test_elgamal() {
    let mut rng = rand::thread_rng();
    let (pk, sk) = elgamal::ElGamal::key_generation(24, &mut rng);
    let a = elgamal::Message::new(PrimeGroupElement::new(
        (24 as usize).to_biguint().unwrap(),
        Rc::new(pk.group.clone()),
    ));
    let c = elgamal::ElGamal::encrypt(&pk, &a, &mut rng);
    let m = elgamal::ElGamal::decrypt(&sk, &c, &mut rng);
    assert_eq!(a, m.unwrap());
}
