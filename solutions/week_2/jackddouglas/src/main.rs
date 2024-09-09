use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

// a large prime number (2^521 - 1)
const PRIME: &str = "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151";

fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base.clone();
    let mut exp = exp.clone();
    while exp > BigUint::zero() {
        if exp.bit(0) {
            result = (result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1;
    }
    result
}

// helper function: modular inverse using extended Euclidian algorithm
fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = m.clone();
    let mut new_r = a.clone();

    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        t -= &quotient * &new_t;
        std::mem::swap(&mut t, &mut new_t);
        r -= &quotient * &new_r;
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > BigInt::one() {
        panic!("a is not invertible");
    }
    if t < BigInt::zero() {
        t += m;
    }
    t
}

// generate shares
fn generate_shares(
    secret: &BigUint,
    num_shares: usize,
    threshold: usize,
) -> Vec<(BigUint, BigUint)> {
    let prime = BigUint::parse_bytes(PRIME.as_bytes(), 10).unwrap();
    let mut rng = thread_rng();
    let mut coefficients = vec![secret.clone()];
    for _ in 1..threshold {
        coefficients.push(rng.gen_biguint_below(&prime));
    }

    (1..=num_shares)
        .map(|x| {
            let x_biguint = BigUint::from(x);
            let mut y = BigUint::zero();
            for (i, coeff) in coefficients.iter().enumerate() {
                y += coeff * mod_pow(&x_biguint, &BigUint::from(i), &prime);
                y %= &prime;
            }
            (x_biguint, y)
        })
        .collect()
}

// reconstruct the secret
fn reconstruct_secret(shares: &[(BigUint, BigUint)], threshold: usize) -> BigUint {
    let prime = BigUint::parse_bytes(PRIME.as_bytes(), 10).unwrap();
    let prime_int = prime.to_bigint().unwrap();
    let mut secret = BigInt::zero();

    for i in 0..threshold {
        let (xi, yi) = &shares[i];
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for j in 0..threshold {
            if i != j {
                let (xj, _) = &shares[j];
                numerator *= xj.to_bigint().unwrap();
                numerator %= &prime_int;
                denominator *=
                    (xj.to_bigint().unwrap() - xi.to_bigint().unwrap() + &prime_int) % &prime_int;
                denominator %= &prime_int;
            }
        }

        let term = yi.to_bigint().unwrap() * numerator * mod_inverse(&denominator, &prime_int);
        secret += term;
        secret %= &prime_int;
    }

    if secret < BigInt::zero() {
        secret += prime_int;
    }
    secret.to_biguint().unwrap()
}

fn main() {
    let secret = BigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
    let num_shares = 5;
    let threshold = 3;

    let shares = generate_shares(&secret, num_shares, threshold);
    println!("Shares:");
    for (i, share) in shares.iter().enumerate() {
        println!("Share {}: ({}, {})", i + 1, share.0, share.1);
    }

    let reconstructed_secret = reconstruct_secret(&shares[0..threshold], threshold);
    println!("Reconstructed secret:      {}", reconstructed_secret);
    println!("Original secret:           {}", secret);
    println!(
        "Reconstruction successful: {}",
        reconstructed_secret == secret
    );
}

