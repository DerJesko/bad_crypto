use crate::fields::{Field, FiniteFieldElement};
use crate::matrix::dot;
use crate::traits;
use bigdecimal::BigDecimal;
use ndarray::{Array, Array2, ShapeBuilder};
use rand::distributions::StandardNormal;
use rand::prelude::*;
use std::rc::Rc;

const M: usize = 5;
const N: usize = 2;
const B: f64 = 2.;

fn chi(rng: &mut ThreadRng) -> BigDecimal {
    loop {
        let r = rng.sample(StandardNormal);
        if r.abs() < B {
            return BigDecimal::from(r);
        }
    }
}

pub struct Regev();

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct PublicKey {
    field: Rc<Field>,
    A: Array2<FiniteFieldElement>,
    b: Array2<FiniteFieldElement>,
}

pub struct Ciphertext(Array2<FiniteFieldElement>, Array2<FiniteFieldElement>);

#[derive(PartialEq, Debug)]
pub struct Message(pub bool);

pub struct SecretKey(Array2<FiniteFieldElement>, PublicKey);

impl traits::PubKEncryption<PublicKey, SecretKey, Message, Ciphertext> for Regev {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PublicKey, SecretKey) {
        let field = Rc::new(Field::rand_new(sec_param, rng));
        #[allow(non_snake_case)]
        let A = Array::from_shape_fn((M, N).f(), |_| FiniteFieldElement::rand_new(&field, rng));
        let s = Array::from_shape_fn((N, 1).f(), |_| FiniteFieldElement::rand_new(&field, rng));
        let e = Array::from_shape_fn((M, 1).f(), |_| FiniteFieldElement::new(chi(rng), &field));
        let b = dot(&A, &s) + e;
        let pk = PublicKey {
            field: field,
            A: A,
            b: b,
        };
        (pk.clone(), SecretKey(s, pk))
    }

    fn encrypt(pub_key: &PublicKey, message: &Message, rng: &mut ThreadRng) -> Ciphertext {
        let x = Array::from_shape_fn((1, M).f(), |_| {
            FiniteFieldElement::new(
                BigDecimal::from({
                    if rng.gen() {
                        1
                    } else {
                        0
                    }
                }),
                &pub_key.field,
            )
        });
        Ciphertext(
            dot(&x, &pub_key.A),
            dot(&x, &pub_key.b)
                + Array::from_shape_fn((1, 1).f(), |_| {
                    FiniteFieldElement::new(
                        &pub_key.field.order / 2 * BigDecimal::from(message.0 as u8),
                        &pub_key.field,
                    )
                }),
        )
    }
    fn decrypt(
        sec_key: &SecretKey,
        cipher_text: &Ciphertext,
        _: &mut ThreadRng,
    ) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let SecretKey(s, pk) = sec_key;
        let z: BigDecimal = &(c2 - &dot(&c1, &s))[[0, 0]].number - &pk.field.order / 2;
        Some(Message(!(z.abs() < &pk.field.order / 4)))
    }
}
