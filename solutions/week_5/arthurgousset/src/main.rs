// main.rs

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
    let m0 = 55.to_bigint().unwrap(); // First message
    let m1 = 42.to_bigint().unwrap(); // Second message

    println!("Sender's messages:");
    println!("  Message 0 (m0): {}", m0);
    println!("  Message 1 (m1): {}\n", m1);

    // Receiver's choice (b = 0 or 1)
    let b = 1; // Receiver wants to receive message m1

    println!("Receiver wants to obtain message m{}.\n", b);

    // Receiver generates a key pair
    let mut rng = thread_rng();
    let x = rng.gen_bigint_range(&One::one(), &(p.clone() - 1)); // Receiver's private key
    let h = g.modpow(&x, &p); // Receiver's public key

    println!("Receiver's key generation:");
    println!("  Private key (x): {}", x);
    println!("  Public key (h): {}\n", h);

    // Receiver sends h_b to the sender
    // For b=0, h0 = h, h1 = random element
    // For b=1, h0 = random element, h1 = h
    let h0;
    let h1;
    let r = rng.gen_bigint_range(&One::one(), &(p.clone() - 1)); // Random element

    if b == 0 {
        h0 = h.clone();
        h1 = g.modpow(&r, &p);
    } else {
        h0 = g.modpow(&r, &p);
        h1 = h.clone();
    }

    println!("Receiver sends h0 and h1 to the sender:");
    println!("  h0: {}", h0);
    println!("  h1: {}\n", h1);

    // Sender encrypts messages
    let k = rng.gen_bigint_range(&One::one(), &(p.clone() - 1)); // Sender's random value

    // Encrypt messages
    let c0 = encrypt(&m0, &h0, &k, &g, &p);
    let c1 = encrypt(&m1, &h1, &k, &g, &p);

    println!("Sender's encryption:");
    println!("  Random k: {}", k);
    println!("  Ciphertext c0: {:?}", c0);
    println!("  Ciphertext c1: {:?}\n", c1);

    // Sender sends (c0, c1) to Receiver

    // Receiver decrypts the chosen message
    let c_b = if b == 0 { c0 } else { c1 };

    let m_b = decrypt(&c_b, &x, &p);

    println!("Receiver's decryption:");
    println!("  Received ciphertext: {:?}", c_b);
    println!("  Decrypted message m_b: {}\n", m_b);

    println!("Receiver successfully obtained message m{}: {}", b, m_b);
}

// Function to generate a large prime number (for demonstration purposes, we use a fixed prime)
fn generate_large_prime() -> BigInt {
    // For simplicity, we use a known prime number (2048-bit MODP Group)
    BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
                          29024E088A67CC74020BBEA63B139B22514A08798E3404DDE\
                          F9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E4\
                          85B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE3\
                          86BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC200\
                          7CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655\
                          D23DCA3AD961C62F356208552BB9ED529077096966D670C35\
                          4E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772\
                          C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6\
                          955817183995497CEA956AE515D2261898FA051015728E5A8\
                          AAAC42DAD33170D04507A33A85521ABDF1CBA64ECFB850458\
                          DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7ABF5AE8CDB0\
                          933D71E8C94E0FFEE0E6BA0D2F383B74D9C2D221D81E7A8DC\
                          1666E8A060CD64EA95BA53A68B71E3C0E04A85401373E03C4\
                          2A6C899FA6E5EB20AC2FA3D5C000000000000000000000001", 16).unwrap()
}

// Function to encrypt a message using ElGamal encryption
fn encrypt(m: &BigInt, h: &BigInt, k: &BigInt, g: &BigInt, p: &BigInt) -> (BigInt, BigInt) {
    let c1 = g.modpow(k, p);
    let s = h.modpow(k, p);
    let c2 = (m * s) % p;
    (c1, c2)
}

// Function to decrypt a message using ElGamal decryption
fn decrypt(ciphertext: &(BigInt, BigInt), x: &BigInt, p: &BigInt) -> BigInt {
    let (c1, c2) = ciphertext;
    let s = c1.modpow(x, p);
    let s_inv = modinv(&s, p).unwrap();
    (c2 * s_inv) % p
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