use crate::TWO;
use num_bigint::{BigInt, BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;

pub fn random_prime(sec_param: usize, rng: &mut ThreadRng) -> BigUint {
    let mut res: BigUint;
    let two = TWO();
    let one: BigUint = One::one();
    loop {
        res = (rng.gen_biguint(sec_param as u64) * &two) + &one;
        if prime_eh(&res, sec_param, rng) {
            return res;
        }
    }
}

pub fn prime_eh(n: &BigUint, amount_checks: usize, rng: &mut ThreadRng) -> bool {
    let one: BigUint = One::one();
    let two = TWO();
    let n_minus_one: BigUint = n - one;
    let (exponent, factor) = div_by_pow_2(n_minus_one.clone());

    'witness: for _ in 0..amount_checks {
        let a = rng.gen_biguint_range(&(two), &(n - &two));
        let mut x = a.modpow(&factor, n);
        if !(One::is_one(&x) || x == n_minus_one) {
            for _ in 1..(exponent - (1 as usize)) {
                x = x.modpow(&two, n);
                if x == n_minus_one {
                    continue 'witness;
                }
            }
            return false;
        }
    }
    true
}

fn div_by_pow_2(n: BigUint) -> (usize, BigUint) {
    let mut n1 = n;
    let mut i = 0;
    let t = TWO();
    while Zero::is_zero(&(&n1 % &t)) {
        i += 1;
        n1 /= &t;
    }
    (i, n1)
}

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if Zero::is_zero(a) {
        b.clone()
    } else {
        gcd(&(b % a), a)
    }
}

pub fn extended_gcd(a: BigInt, b: BigInt) -> ((BigInt, BigInt), BigInt) {
    let mut s: BigInt = Zero::zero();
    let mut r = b.clone();
    let mut old_s: BigInt = One::one();
    let mut old_r = a.clone();
    let mut quotient;
    while !r.is_zero() {
        quotient = &old_r / &r;
        let r1 = old_r - &quotient * &r;
        old_r = r;
        r = r1;
        let s1 = &old_s - &quotient * &s;
        old_s = s;
        s = s1;
    }
    let bezout_t = if b.is_zero() {
        BigInt::from(0 as u8)
    } else {
        (&old_r - &old_s * a) / b
    };
    ((old_s, bezout_t), old_r)
}

pub fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    (a * b) / gcd(a, b)
}
