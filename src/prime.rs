use crate::prime2000::PRIME2000;
use crate::{MIN_ODD, TWO};
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::prelude::ThreadRng;

pub fn random_prime(sec_param: usize, rng: &mut ThreadRng) -> BigUint {
    let n = if sec_param < 14 { 14 } else { sec_param };
    let mut res;
    let two = TWO();
    let min_number = MIN_ODD();
    loop {
        res = rng.gen_biguint(n) * &two + &min_number; // uniformly choose a odd number which is >17391
        if prime_eh(&res, n, rng) {
            return res;
        }
    }
}

pub fn prime_eh(n: &BigUint, amount_checks: usize, rng: &mut ThreadRng) -> bool {
    let mut prime_candidate = true;
    for j in PRIME2000.iter() {
        if Zero::is_zero(&(n % j)) {
            prime_candidate = false;
            break;
        }
    }
    if !prime_candidate {
        return false;
    }

    let one: BigUint = One::one();
    let two = TWO();
    let n_minus_one: BigUint = n - one;
    let (exponent, factor) = div_by_pow_2(n_minus_one.clone());

    'witness: for _ in 1..amount_checks {
        let a = rng.gen_biguint_range(&(two), &(n - &two));
        let mut x = a.modpow(&factor, n);
        if One::is_one(&x) || x == n_minus_one {
            continue;
        }
        for _ in 1..(exponent - (1 as usize)) {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                continue 'witness;
            }
        }
        return false;
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
