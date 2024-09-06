use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_traits::{One, Signed, Zero};
use rand::thread_rng;

fn generate_prime(bits: u64) -> BigUint {
    let mut rng = thread_rng();
    loop {
        let n: BigUint = rng.gen_biguint(bits);
        if is_prime(&n, 20) {
            return n;
        }
    }
}

fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base.clone();
    let mut exp = exp.clone();
    let zero = BigUint::zero();
    let two = BigUint::from(2u32);

    while exp > zero {
        if &exp % &two == BigUint::one() {
            result = (&result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }
    result
}

fn miller_rabin_test(n: &BigUint, a: &BigUint) -> bool {
    if *n == BigUint::from(2u32) {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_one = n - &one;

    let mut r = 0u32;
    let mut s = n_minus_one.clone();
    while &s % &two == BigUint::zero() {
        r += 1;
        s >>= 1;
    }

    let mut x = mod_pow(a, &s, n);
    if x == one || x == n_minus_one {
        return true;
    }

    for _ in 0..r - 1 {
        x = mod_pow(&x, &two, n);
        if x == n_minus_one {
            return true;
        }
    }

    false
}

fn is_prime(n: &BigUint, k: u32) -> bool {
    if *n < BigUint::from(2u32) {
        return false;
    }

    let mut rng = thread_rng();

    for _ in 0..k {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), &(n - 1u32));
        if !miller_rabin_test(n, &a) {
            return false;
        }
    }

    true
}

fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let a = a.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();
    let mut t = BigInt::zero();
    let mut newt = BigInt::one();
    let mut r = m.clone();
    let mut newr = a;

    while !newr.is_zero() {
        let quotient = &r / &newr;
        let temp_t = t - &quotient * &newt;
        t = std::mem::replace(&mut newt, temp_t);

        let temp_r = &r - &quotient * &newr;
        r = std::mem::replace(&mut newr, temp_r);
    }

    if r > BigInt::one() {
        None
    } else {
        let mut result = if t.is_negative() { t + &m } else { t };
        while result < BigInt::zero() {
            result += &m;
        }
        while result >= m {
            result -= &m;
        }
        Some(result.to_biguint().unwrap())
    }
}

fn generate_keypair() -> (BigUint, BigUint, BigUint) {
    let p = generate_prime(1024);
    let q = generate_prime(1024);
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
