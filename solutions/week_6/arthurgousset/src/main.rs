use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::rngs::OsRng;

/// Entry point demonstrating RSA blind signature
fn main() {
    // Generate RSA keys
    let bits = 512; // Bit length of RSA modulus n
    let (n, e, d) = generate_rsa_keys(bits);

    // Message to be signed
    let message = BigUint::from_bytes_be(b"Hello, world!");

    // Blinding factor and blinded message
    let (blinded_message, r) = blind_message(&message, &e, &n);

    // Signer signs the blinded message
    let blinded_signature = rsa_sign(&blinded_message, &d, &n);

    // Unblind the signature to get the valid signature on the original message
    let signature = unblind_signature(&blinded_signature, &r, &n);

    // Verify the signature
    let is_valid = rsa_verify(&message, &signature, &e, &n);

    println!("Signature valid: {}", is_valid);
}

/// Generate RSA keys (n, e, d) with specified bit length
fn generate_rsa_keys(bits: usize) -> (BigUint, BigUint, BigUint) {
    let mut rng = OsRng;

    // Generate two distinct large primes p and q
    let p = generate_large_prime(bits / 2, &mut rng);
    let mut q = generate_large_prime(bits / 2, &mut rng);
    while q == p {
        q = generate_large_prime(bits / 2, &mut rng);
    }

    // Compute n = p * q
    let n = &p * &q;

    // Compute φ(n) = (p - 1)(q - 1)
    let phi = (&p - BigUint::one()) * (&q - BigUint::one());

    // Choose e such that 1 < e < φ(n) and gcd(e, φ(n)) = 1
    let e = BigUint::from(65537u32); // Common choice for e

    // Ensure that e and φ(n) are coprime
    assert_eq!(e.gcd(&phi), BigUint::one());

    // Compute d ≡ e^{-1} mod φ(n)
    let d = modinv(&e, &phi).expect("Modular inverse does not exist");

    (n, e, d)
}

/// Generate a large prime number of specified bit length
fn generate_large_prime(bits: usize, rng: &mut OsRng) -> BigUint {
    loop {
        // Generate random odd number of specified bit length
        let mut candidate = rng.gen_biguint(bits.try_into().unwrap());
        candidate.set_bit(0, true); // Ensure it's odd

        if is_prime(&candidate) {
            return candidate;
        }
    }
}

/// Probabilistic Miller-Rabin primality test
fn is_prime(candidate: &BigUint) -> bool {
    if candidate <= &BigUint::from(3u32) {
        return candidate == &BigUint::from(2u32) || candidate == &BigUint::from(3u32);
    }

    if candidate.is_even() {
        return false;
    }

    // Write candidate - 1 as 2^s * d
    let mut d = candidate - BigUint::one();
    let mut s = 0u32;

    while d.is_even() {
        d /= 2u32;
        s += 1;
    }

    let mut rng = OsRng;
    let rounds = 5; // Number of test rounds

    for _ in 0..rounds {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), &(candidate - BigUint::from(2u32)));
        let mut x = a.modpow(&d, candidate);

        if x == BigUint::one() || x == candidate - BigUint::one() {
            continue;
        }

        let mut is_composite = true;
        for _ in 0..(s - 1) {
            x = x.modpow(&BigUint::from(2u32), candidate);
            if x == candidate - BigUint::one() {
                is_composite = false;
                break;
            }
        }

        if is_composite {
            return false;
        }
    }

    true
}

/// Compute modular inverse of a mod m
fn modinv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (g, x, _) = extended_gcd(&a.to_bigint().unwrap(), &m.to_bigint().unwrap());
    if g != BigInt::one() {
        None
    } else {
        Some((x % m.to_bigint().unwrap() + m.to_bigint().unwrap()) % m.to_bigint().unwrap())
            .map(|x| x.to_biguint().unwrap())
    }
}

/// Extended Euclidean Algorithm
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x, y) = extended_gcd(b, &(a % b));
        (g, y.clone(), x - (a / b) * y)
    }
}

/// Blind the message using a random blinding factor
fn blind_message(message: &BigUint, e: &BigUint, n: &BigUint) -> (BigUint, BigUint) {
    let mut rng = OsRng;

    loop {
        // Choose random r where 1 < r < n and gcd(r, n) = 1
        let r = rng.gen_biguint_range(&BigUint::from(2u32), n);

        if r.gcd(n) == BigUint::one() {
            // Compute blinded_message = (message * r^e) mod n
            let blinded_message = (message * r.modpow(e, n)) % n;
            return (blinded_message, r);
        }
    }
}

/// Sign the message using private exponent d
fn rsa_sign(message: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    message.modpow(d, n)
}

/// Unblind the signature to obtain signature on the original message
fn unblind_signature(blinded_signature: &BigUint, r: &BigUint, n: &BigUint) -> BigUint {
    // Compute r^{-1} mod n
    let r_inv = modinv(r, n).expect("No modular inverse of r mod n");

    // Compute signature = (blinded_signature * r^{-1}) mod n
    (blinded_signature * r_inv) % n
}

/// Verify the signature using public exponent e
fn rsa_verify(message: &BigUint, signature: &BigUint, e: &BigUint, n: &BigUint) -> bool {
    // Compute expected_message = signature^e mod n
    let expected_message = signature.modpow(e, n);

    &expected_message == message
}
