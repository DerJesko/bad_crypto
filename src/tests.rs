use crate::elgamal;
use crate::groups::{PrimeGroup, PrimeGroupElement};
use crate::prime::random_prime;
use crate::traits::PubKEncryption;
use crate::TWO;
use ndarray::arr2;
use num_bigint::ToBigUint;
use rand;
use std::rc;

#[test]
fn gen_prime() {
    let mut rng = rand::thread_rng();
    println!("A prime: {}", random_prime(13, &mut rng));
}

#[test]
fn gen_group() {
    let mut rng = rand::thread_rng();
    let group = rc::Rc::new(PrimeGroup::rand_new(2, &mut rng));
    let generator = PrimeGroupElement::rand_generator(&group, &mut rng);
    println!("generator: {:?}", generator);

    let x = (25 as usize).to_biguint().unwrap();
    let gx = generator.pow(&x);
    println!("g^x: {:?}", gx);

    let minus_x = generator.group.exp_inverse(&x);
    let gminusx = generator.pow(&minus_x);
    println!("-x: {:?}", minus_x);

    assert_eq!(PrimeGroupElement::one(&generator.group), &gx * &gminusx);
}
#[test]
fn test_elgamal() {
    let mut rng = rand::thread_rng();
    for _ in 1..5 {
        let (pk, sk) = elgamal::ElGamal::key_generation(24, &mut rng);
        let a = elgamal::Message::new(PrimeGroupElement::new(
            (13338 as usize).to_biguint().unwrap(),
            &pk.generator_g.group,
        ));
        let c = elgamal::ElGamal::encrypt(&pk, &a, &mut rng);
        let m = elgamal::ElGamal::decrypt(&sk, &c, &mut rng);
        assert_eq!(a, m.unwrap());
    }
}
