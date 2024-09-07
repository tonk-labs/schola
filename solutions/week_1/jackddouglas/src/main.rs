use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

const SMALL_PRIMES: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

fn generate_prime(bits: u64) -> BigUint {
    let mut rng = thread_rng();
    loop {
        let n: BigUint = rng.gen_biguint(bits);
        if n.is_odd() && is_prime(&n) {
            return n;
        }
    }
}

fn miller_rabin_test(n: &BigUint, a: &BigUint) -> bool {
    if *n == BigUint::from(2u32) {
        return true;
    }
    if n.is_even() {
        return false;
    }

    let n_minus_one = n - 1u32;
    let s = n_minus_one.trailing_zeros().unwrap();
    let d = &n_minus_one >> s;

    let mut x = a.modpow(&d, n);
    if x == BigUint::one() || x == n_minus_one {
        return true;
    }

    for _ in 0..s - 1 {
        x = (&x * &x) % n;
        if x == n_minus_one {
            return true;
        }
    }

    false
}

fn is_prime(n: &BigUint) -> bool {
    for &p in &SMALL_PRIMES {
        if *n == p.into() {
            return true;
        }
        if n % p == BigUint::zero() {
            return false;
        }
    }

    if n < &BigUint::from(2047u32) {
        return miller_rabin_test(n, &BigUint::from(2u32));
    }

    let bases: Arc<Vec<BigUint>> = Arc::new(
        if n.bits() < 64 {
            vec![2u32, 3, 5, 7, 11, 13, 17]
        } else if n.bits() < 128 {
            vec![2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
        } else {
            vec![2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]
        }
        .into_iter()
        .map(BigUint::from)
        .collect::<Vec<BigUint>>(),
    );

    bases.par_iter().all(|b| miller_rabin_test(n, b))
}

fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let a = a.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();
    let (mut t, mut newt) = (BigInt::zero(), BigInt::one());
    let (mut r, mut newr) = (m.clone(), a);

    while !newr.is_zero() {
        let quotient = &r / &newr;
        (t, newt) = (newt.clone(), t - &quotient * &newt);
        (r, newr) = (newr.clone(), r - &quotient * &newr);
    }

    if r > BigInt::one() {
        None
    } else {
        while t < BigInt::zero() {
            t += &m;
        }
        Some((t % &m).to_biguint().unwrap())
    }
}

fn generate_keypair() -> (BigUint, BigUint, BigUint) {
    let (p, q) = rayon::join(|| generate_prime(1024), || generate_prime(1024));
    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);
    let e = BigUint::from(65537u32);
    let d = mod_inverse(&e, &phi).unwrap();
    (n, e, d)
}

fn encrypt(m: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
    m.modpow(e, n)
}

fn decrypt(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(d, n)
}

fn main() {
    let (n, e, d) = generate_keypair();
    println!("Public key (n, e): ({}, {})", n, e);
    println!("Private key (d): {}", d);

    let message = BigUint::from(1234567890u64);
    println!("Original message: {}", message);

    let encrypted = encrypt(&message, &e, &n);
    println!("Encrypted message: {}", encrypted);

    let decrypted = decrypt(&encrypted, &d, &n);
    println!("Decrypted message: {}", decrypted);

    assert_eq!(message, decrypted);
}
