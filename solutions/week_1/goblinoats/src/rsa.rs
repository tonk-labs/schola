use num_bigint::{BigInt, BigUint, ToBigInt};
use rand::Rng;

const LARGE_RANDOM: u64 = 7686958497779733907;

fn decompose(candidate: &BigUint) -> (u32, BigUint) {
    let mut shift = 0;
    let mut odd_factor = candidate.clone();
    while odd_factor.bit(0) == false {
        odd_factor >>= 1;
        shift += 1;
    }
    (shift, odd_factor)
}

fn pow_mod(a: &BigUint, x: &BigUint, n: &BigUint) -> BigUint {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let mut r = one.clone();
    let mut a = a.clone();
    let mut x = x.clone();
    while x != zero {
        if x.bit(0) {
            r = (&r * &a) % n;
        }
        x >>= 1;
        a = (&a * &a) % n;
    }
    r
}

fn fermat_test(n: &BigUint) -> bool {
    let one = BigUint::from(1u32);
    if *n <= one {
        return false;
    }
    let large_random = BigUint::from(LARGE_RANDOM);
    large_random.modpow(&(n - &one), n) == one
    // pow_mod(&large_random, &(n - &one), n) == one
}

fn miller_rabin_test(n: &BigUint, k: u32) -> bool {
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);
    if *n == two || *n == three {
        return true;
    }
    if *n <= one || n.bit(0) == false {
        return false;
    }
    let (s, d) = decompose(&(n - &one));
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a = rng.gen_range(two.clone()..n - &two);
        // let mut witness = pow_mod(&a, &d, n);
        let mut witness = a.modpow(&d, n);
        if witness != one && witness != (n - &one) {
            let mut e = 0;
            loop {
                // we could improve our pow_mod by using montgomery reduction
                // the built-in pow_mod does this.
                // witness = pow_mod(&witness, &two, n);
                witness = witness.modpow(&two, n);
                if witness == (n - &one) {
                    break;
                } else if witness == one || e == (s - 1) {
                    return false;
                }
                e += 1;
            }
        }
    }
    true
}

fn is_prime(n: &BigUint) -> bool {
    fermat_test(n) && miller_rabin_test(n, 10)
}

/**
 * This is the limiting function. Generating large primes accounts for 99% of the time.
 * 
 * 
 * Standard method for generating large prime numbers:
 * 
 * 1. Select a random number of the desired length (check)
 * 2. Apply a Fermat test (optimized with base 2 for speed) (check, could be optimized a bit)
 * 3. Apply multiple Miller-Rabin tests (number depends on length and desired error rate, e.g., 2^-100) (check)
 * 
 * Preselection methods:
 * - Test divisions by small primes (up to a few hundred) (TODO)
 * - Sieve out primes up to 10,000 - 1,000,000, considering candidates of form b + 2i (TODO)
 *   (where b is large, i up to a few thousand)
 * 
 * Preselection function:
 * 
 * Note: The deterministic AKS primality test is generally not used due to:
 * - Slower performance
 * - Higher likelihood of hardware-induced calculation errors compared to probabilistic methods
 * 
 * Hardware acceleration:
 * Many smart cards include coprocessors for modular arithmetic (1024 to several thousand bits).
 * Manufacturers often provide libraries for RSA and key generation utilizing these coprocessors.
 */
fn generate_prime(min: &BigUint, max: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let mut n: BigUint = rng.gen_range(min.clone()..=max.clone());
        // this bit flag means we spend less time generating non-prime numbers
        n |= (BigUint::from(1u32) << (n.bits() - 1)) | BigUint::from(1u32);

        // Apply Miller-Rabin test with fewer rounds for initial screening
        if !fermat_test(&n) {
            continue;
        }

        if !miller_rabin_test(&n, 3) {
            continue;
        }


        if miller_rabin_test(&n, 10) {
            return n;
        }

    }
}

fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let big_int_zero = BigInt::from(0);
    let big_int_one = BigInt::from(1);
    if a == &big_int_zero {
        (b.clone(), big_int_zero.clone(), big_int_one.clone())
    } else {
        let (g, x, y) = egcd(&(b % a), a);
        let q = b / a;
        (g, y - (&q * &x), x)
    }
}

pub fn modinverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let big_int_one = BigInt::from(1);
    let a_bigint = a.to_bigint().unwrap();
    let m_bigint = m.to_bigint().unwrap();
    let (g, x, _) = egcd(&a_bigint, &m_bigint);
    if g != big_int_one {
        None
    } else {
        Some(((x % &m_bigint + &m_bigint) % &m_bigint).to_biguint().unwrap())
    }
}

pub struct RsaPublicKey {
    /// Modulus: product of prime numbers `p` and `q`
    n: BigUint,
    /// Public exponent: power to which a plaintext message is raised in
    /// order to encrypt it.
    ///
    /// Typically 0x10001 (65537)
    e: BigUint,
}

impl std::fmt::Debug for RsaPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RsaPublicKey")
            .field("n", &self.n.to_string())
            .field("e", &self.e.to_string())
            .finish()
    }
}

pub struct RsaPrivateKey {
    pubkey_components: RsaPublicKey,

    /// Private exponent
    d: BigUint,
}

impl std::fmt::Debug for RsaPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RsaPrivateKey")
            .field("pubkey_components", &self.pubkey_components)
            .field("d", &self.d.to_string())
            .finish()
    }
}

impl RsaPublicKey {
    // to encrypt a message m, compute c = m^e mod n
    pub fn encrypt(&self, message: &BigUint) -> BigUint {
        message.modpow(&self.e, &self.n)
    }
}

impl RsaPrivateKey {
    pub fn new(key_size: u64) -> (RsaPublicKey, Self) {
        let one = BigUint::from(1u32);
        // choose some random prime numbers p and q
        let min = BigUint::from(1u64) << (key_size - 1);
        let max = BigUint::from(1u64) << key_size;
        let p = generate_prime(&min, &max);
        let q = generate_prime(&min, &max);
        // compute n = p*q
        let n = &p * &q;
        // compute phi(n) = (p-1)*(q-1)
        let phi = (&p - &one) * (&q - &one);
        // choose some random number e, such that gcd(e, phi(n)) = 1
        // it's considered efficient to choose a known prime
        let e = BigUint::from(65537u64);
        // compute d, such that d*e = 1 mod phi(n)
        let d = modinverse(&e, &phi).unwrap();

        // (e, n) is the public key
        // (d, n) is the private key
        (
            RsaPublicKey {
                n: n.clone(),
                e: e.clone(),
            },
            Self {
                pubkey_components: RsaPublicKey { n, e },
                d,
            },
        )
    }

    pub fn decrypt(&self, ciphertext: &BigUint) -> BigUint {
        // to decrypt a message c, compute m = c^d mod n
        ciphertext.modpow(&self.d, &self.pubkey_components.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primality() {
        let p1 = BigUint::from(2791u64);
        let p2 = BigUint::from(87178291199u64);
        let p3 = BigUint::from(2u64);
        let np = BigUint::from(4u64);
        let np2 = BigUint::from(3000000002u64);
        assert!(is_prime(&p1));
        assert!(is_prime(&p2));
        assert!(is_prime(&p3));
        assert!(!is_prime(&np));
        assert!(!is_prime(&np2));
    }

    #[test]
    fn test_modinverse() {
        let a = BigUint::from(3u64);
        let m = BigUint::from(11u64);
        let result = modinverse(&a, &m);
        assert_eq!(result, Some(BigUint::from(4u64)));

        // Test when inverse doesn't exist
        let a = BigUint::from(2u64);
        let m = BigUint::from(4u64);
        let result = modinverse(&a, &m);
        assert_eq!(result, None);

        // Test with larger numbers
        let a = BigUint::from(17u64);
        let m = BigUint::from(3120u64);
        let result = modinverse(&a, &m);
        assert_eq!(result, Some(BigUint::from(2753u64)));
    }

    #[test]
    fn rsa() {
        let (pubkey, privkey) = RsaPrivateKey::new(2048);
        let message = BigUint::from(42u64);
        let encrypted = pubkey.encrypt(&message);
        let decrypted = privkey.decrypt(&encrypted);
        println!("Message: {}", message);
        println!("Encrypted: {}", encrypted);
        println!("Decrypted: {}", decrypted);
        assert_ne!(message, encrypted);
        assert_eq!(message, decrypted);
    }
}
