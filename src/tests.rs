use crate::fields::Field;
use crate::groups::{PrimeGroup, PrimeGroupElement};
use crate::matrix::Matrix;
use crate::prime::{extended_gcd, random_prime};
use crate::traits::PubKEncryption;
use crate::{elgamal, regev, rsa};
use ndarray::arr2;
use num_bigint::{ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use rand;
use rand::Rng;
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
    for _ in 0..5 {
        let (pk, sk) = elgamal::ElGamal::key_generation(24, &mut rng);
        let a = elgamal::Message::new(PrimeGroupElement::new(
            (13338 as usize).to_biguint().unwrap(),
            &pk.generator_g.group,
        ));
        let c = elgamal::ElGamal::encrypt(&pk, &a, &mut rng);
        let m = elgamal::ElGamal::decrypt(&sk, &c, &mut rng).unwrap();
        assert_eq!(a, m);
    }
}

/*
#[test]
fn test_regev() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let (pk, sk) = regev::Regev::key_generation(2, &mut rng);
        let a = regev::Message(rng.gen());
        let c = regev::Regev::encrypt(&pk, &a, &mut rng);
        let m = regev::Regev::decrypt(&sk, &c, &mut rng).unwrap();
        println!("{:?}", m);
        assert_eq!(a, m);
    }
}
*/
#[test]
fn test_egcd() {
    let a = (240 as u8).to_bigint().unwrap();
    let b = (46 as u8).to_bigint().unwrap();
    let ((x, y), _z) = extended_gcd(a.clone(), b.clone());
    println!("x: {}, y: {}", x, y);
    println!("{}", a * x + b * y);
}

#[test]
fn test_rsa() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let (pk, sk) = rsa::RSA::key_generation(2, &mut rng);
        let a = rsa::Message((13338 as usize).to_biguint().unwrap());
        let c = rsa::RSA::encrypt(&pk, &a, &mut rng);
        let m = rsa::RSA::decrypt(&sk, &c, &mut rng).unwrap();
        println!("{:?}", m);
        assert_eq!(a, m);
    }
}
