use num_traits::{One, Zero};
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn random_prime_in_range(
    checks: usize,
    lower_bound: usize,
    upper_bound: usize,
    rng: &mut ThreadRng,
) -> usize {
    let mut res: usize;
    let l = lower_bound / 2;
    let u = upper_bound / 2;
    loop {
        res = (rng.gen_range(l, u) * 2) + 1;
        if prime_eh(res, checks, rng) {
            return res;
        }
    }
}

pub fn prime_eh(n: usize, amount_checks: usize, rng: &mut ThreadRng) -> bool {
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
