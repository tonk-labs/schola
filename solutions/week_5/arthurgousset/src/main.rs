use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

fn main() {
    println!("--- Oblivious Transfer using ElGamal Cryptosystem ---\n");

    // Common parameters (public)
    let p = generate_large_prime(); // Large prime modulus
    let g = 2.to_bigint().unwrap(); // Generator (for simplicity, we use 2)

    println!("Public parameters:");
    println!("  Prime modulus (p): {}", p);
    println!("  Generator (g): {}\n", g);

    // Sender's messages
    let m0 = 42.to_bigint().unwrap(); // First message
    let m1 = 99.to_bigint().unwrap(); // Second message

    println!("Sender's messages:");
    println!("  Message 0 (m0): {}", m0);
    println!("  Message 1 (m1): {}\n", m1);

    // Receiver chooses which message to receive (b = 0 or 1)
    let b = 1; // Receiver wants to receive message m1

    println!("Receiver wants to obtain message m{}.\n", b);

    // Receiver's key generation
    let mut rng = thread_rng();
    let x = rng.gen_bigint_range(&One::one(), &(p.clone() - 1)); // Receiver's private key
    let h = g.modpow(&x, &p); // Receiver's public key

    println!("Receiver's key generation:");
    println!("  Private key (x): {}", x);
    println!("  Public key (h): {}\n", h);

    // Sender encrypts messages
    // Sender picks random k0 and k1
    let k0 = rng.gen_bigint_range(&One::one(), &(p.clone() - 1));
    let k1 = rng.gen_bigint_range(&One::one(), &(p.clone() - 1));

    // Compute c0 and c1
    let c0 = h.modpow(&k0, &p);
    let c1 = h.modpow(&k1, &p);

    // Compute d0 and d1
    let s0 = g.modpow(&k0, &p);
    let s1 = g.modpow(&k1, &p);

    let d0 = (m0 * &s0) % &p;
    let d1 = (m1 * &s1) % &p;

    println!("Sender's encryption:");
    println!("  Random k0: {}", k0);
    println!("  Random k1: {}", k1);
    println!("  Ciphertext c0: {}", c0);
    println!("  Ciphertext c1: {}", c1);
    println!("  Encrypted message d0: {}", d0);
    println!("  Encrypted message d1: {}\n", d1);

    // Sender sends (c0, d0) and (c1, d1) to Receiver

    // Receiver computes s_b = c_b^{x} mod p
    let c_b = if b == 0 { c0 } else { c1 };
    let d_b = if b == 0 { d0 } else { d1 };

    let s_b = c_b.modpow(&x, &p);

    // Receiver recovers message m_b = d_b * s_b^{-1} mod p
    let s_b_inv = modinv(&s_b, &p).unwrap(); // Compute multiplicative inverse
    let m_b = (d_b.clone() * s_b_inv.clone()) % &p;

    println!("Receiver's decryption:");
    println!("  Received c_b: {}", c_b);
    println!("  Received d_b: {}", d_b);
    println!("  Computed s_b: {}", s_b);
    println!("  Inverse of s_b: {}", s_b_inv);
    println!("  Recovered message m_b: {}\n", m_b);

    println!("Receiver successfully obtained message m{}: {}", b, m_b);
}

// Function to generate a large prime number (for demonstration purposes, we use a fixed prime)
fn generate_large_prime() -> BigInt {
    // For simplicity, we use a known prime number
    // In practice, you should generate a large random prime
    // Use a known large prime p
    // This is a 2048-bit MODP Group from RFC 3526
    let p = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6BF12FFA06D98A0864D87602733EC86A64521F2B18177B200CBBE117577A615D6C770988C0BAD946E208E24FA074E5AB3143DB5BFCE0FD108E4B82D120A93AD2CAFFFFFFFFFFFFFFFF", 16).unwrap();
    p
}

// Function to compute the modular inverse
fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a.clone(), m.clone());
    if g != One::one() {
        None
    } else {
        Some((x % m + m) % m)
    }
}

// Extended Euclidean Algorithm
fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a == Zero::zero() {
        (b, Zero::zero(), One::one())
    } else {
        let (g, y, x) = extended_gcd(&b % &a, a.clone());
        (g, x - (b / a) * y.clone(), y)
    }
}
