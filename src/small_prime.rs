// This should not be in use
use crate::prime2000::PRIME2000;
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn random_prime(sec_param: usize, rng: &mut ThreadRng) -> usize {
    let n = if sec_param < 14 { 14 } else { sec_param };
    let mut res;
    let min_odd = PRIME2000[1999];
    loop {
        res = rng.gen_range(0, n) * 2 + min_odd;
        if prime_eh(res, n, rng) {
            return res;
        }
    }
}

pub fn prime_eh(n: usize, amount_checks: usize, rng: &mut ThreadRng) -> bool {
    for j in PRIME2000.iter() {
        if Zero::is_zero(&(n % j)) {
            return false;
        }
    }
    let (exponent, factor) = div_by_pow_2(n - 1);
    'witness: for _ in 0..amount_checks {
        let a = rng.gen_range(2, n - 2);
        let mut x = a.pow(factor as u32) % n;
        if !(One::is_one(&x) || x == n - 1) {
            for _ in 1..(exponent - (1 as usize)) {
                x = x.pow(2) % n;
                if x == n - 1 {
                    continue 'witness;
                }
            }
            return false;
        }
    }
    true
}

fn div_by_pow_2(n: usize) -> (usize, usize) {
    let mut n1 = n;
    let mut i = 0;
    while Zero::is_zero(&(n1 % 2)) {
        i += 1;
        n1 /= 2;
    }
    (i, n1)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if Zero::is_zero(&a) {
        b
    } else {
        gcd(b % a, a)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
