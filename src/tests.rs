use crate::elgamal;
use crate::fields::{Field, FiniteFieldElement};
use crate::groups::{PrimeGroup, PrimeGroupElement};
use crate::matrix::dot;
use crate::prime::random_prime;
use crate::regev;
use crate::traits::PubKEncryption;
use bigdecimal::BigDecimal;
use ndarray::arr2;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};
use rand;
use rand::Rng;
use std::rc;

#[test]
fn deci() {
    println!("a / b: {}", BigDecimal::from(20) / BigDecimal::from(3))
}

#[test]
fn field() {
    let mut rng = rand::thread_rng();
    let f = rc::Rc::new(Field::rand_new(2, &mut rng));
    let z: FiniteFieldElement = Zero::zero();
    let o: FiniteFieldElement = One::one();
    let r: FiniteFieldElement = FiniteFieldElement::rand_new(&f, &mut rng);
    println!("{:?}", &z + &o);
    println!("{:?}", &z + &r);
    println!("{:?}", &r + &o);
    println!("{:?}", &r + &r);
}

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

#[test]
fn test_matrix_mul() {
    let mut rng = rand::thread_rng();
    let f = rc::Rc::new(Field::rand_new(2, &mut rng));
    let a = arr2(&[[FiniteFieldElement::rand_new(&f, &mut rng)]]);
    let b = arr2(&[[FiniteFieldElement::rand_new(&f, &mut rng), One::one()]]);
    println!("{:?}", dot(&a, &b));
}

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
