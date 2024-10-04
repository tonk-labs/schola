use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use sha2::{Sha256, Digest};
use rand::Rng;
use rand::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, KeyInit};
use aes_gcm::aead::Payload;

fn get_p() -> BigUint {
    // This is a 2048-bit MODP Group (Group 14) from RFC 3526
    BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AACAA68FFFFFFFFFFFFFFFF", 16).unwrap()
}

fn get_g() -> BigUint {
    // The generator for this group is 2
    BigUint::from(2u32)
}

fn main() {
    let p = get_p();
    let g = get_g();
    let (v, beta) = sender_round_1();
    let mut rng = OsRng;

    // Receiver random value
    let alpha = rng.gen_biguint_range(&BigUint::one(), &(&p - 1u32)); // a ∈ Z_p
    // Compute c = h^alpha mod p

    // The index i (e.g., i = 2)
    let i = 2u32;

    let v_i = v.modpow(&BigUint::from(i), &p);
    let v_inv = modinv(&v_i, &p).unwrap();
    let u = (mod_exp(&g, &alpha, &p) * v_inv) % &p;

    let (v, c_j) = sender_round_2(v, beta, u);


    // Compute w = v^alpha mod p
    let w = mod_exp(&v, &alpha, &p);
    let k = compute_hash(&v, &w);
    println!("Computed key k: {:?}", k);

    let m = decrypt_aes(&k, &c_j[(i-1) as usize]).unwrap();

    println!("Decrypted message: {:?}", m);
    // Convert m from Vec<u8> to ASCII string
    let m_ascii = String::from_utf8_lossy(&m).to_string();
    println!("Decrypted message as ASCII: {}", m_ascii);
}

fn sender_round_1() -> (BigUint, BigUint) {
    let p = get_p();
    let g = get_g();
    let mut rng = OsRng;
    // Sender random value
    let beta = rng.gen_biguint_range(&BigUint::one(), &(&p - 1u32)); // a ∈ Z_p
    // Compute v = g^beta mod p
    let v = mod_exp(&g, &beta, &p);
    (v, beta)
}

fn sender_round_2(v: BigUint, beta: BigUint, u: BigUint) -> (BigUint, Vec<Vec<u8>>) {
    let p = get_p();
    let g = get_g();
    // Parameters
    // In practice, use large safe primes for p and a primitive root for g

    // Sender's messages m0, m1, m2, and m3
    let m0 = b"Baz".to_vec();
    let m1 = b"Goblin".to_vec();
    let m2 = b"Arthur".to_vec();
    let m3 = b"Jack".to_vec();

    // Sender's private and public keys
    let mut rng = OsRng;
    let mut public_keys = Vec::with_capacity(4);
    for _ in 0..4 {
        let s = rng.gen_biguint_range(&BigUint::one(), &(&p - 1u32)); // s ∈ Z_p
        let h = mod_exp(&g, &s, &p); // h = g^s mod p
        public_keys.push(h);
    }

    // Implement the encryption process for multiple messages
    let n = 4; // Number of messages
    let mut c_j = Vec::with_capacity(n);

    for j in 1..=n {
        // Compute u_j = u * v^j mod p
        let v_j = mod_exp(&v, &BigUint::from(j), &p);
        let u_j = (&u * &v_j) % &p;

        // Compute w_j = u_j^beta mod p
        let w_j = mod_exp(&u_j, &beta, &p);

        // Compute k_j = Hash(v || w_j)
        let k_j = compute_hash(&v, &w_j);
        // Print k_j
        println!("k_{} = {:?}", j, k_j);

        // Compute c_j = AES_Encrypt(k_j, m_j)
        let m_j = match j {
            1 => &m0,
            2 => &m1,
            3 => &m2,
            4 => &m3,
            _ => unreachable!(),
        };

        c_j.push(encrypt_aes(&k_j, m_j));
    }

    // Return v and the list of encrypted messages
    (v, c_j)
}

// Function to encrypt m_j using k_j as the key
// Function to encrypt m_j using k_j as the key
fn encrypt_aes(k_j: &BigUint, m_j: &Vec<u8>) -> Vec<u8> {
    let mut key_bytes = k_j.to_bytes_be();

    // Ensure the key is 32 bytes for AES-256
    if key_bytes.len() != 32 {
        if key_bytes.len() < 32 {
            // Pad the key with zeros if it's shorter than 32 bytes
            let mut key_padded = vec![0u8; 32];
            key_padded[32 - key_bytes.len()..].copy_from_slice(&key_bytes);
            key_bytes = key_padded;
        } else {
            // Truncate the key if it's longer than 32 bytes
            key_bytes = key_bytes[key_bytes.len() - 32..].to_vec();
        }
    }

    // Create the AES cipher instance
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes
    let ciphertext = cipher
        .encrypt(&nonce, m_j.as_ref())
        .expect("Couldn't encrypt the data");

    // Combine nonce and ciphertext
    [nonce.as_slice(), &ciphertext].concat()
}

// Function to decrypt the ciphertext using k_j as the key
fn decrypt_aes(k_j: &BigUint, encrypted_data: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let mut key_bytes = k_j.to_bytes_be();

    // Ensure the key is 32 bytes for AES-256
    if key_bytes.len() != 32 {
        if key_bytes.len() < 32 {
            // Pad the key with zeros if it's shorter than 32 bytes
            let mut key_padded = vec![0u8; 32];
            key_padded[32 - key_bytes.len()..].copy_from_slice(&key_bytes);
            key_bytes = key_padded;
        } else {
            // Truncate the key if it's longer than 32 bytes
            key_bytes = key_bytes[key_bytes.len() - 32..].to_vec();
        }
    }

    // Create the AES cipher instance
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    // Split the encrypted_data into nonce and ciphertext
    let nonce = &encrypted_data[..12]; // AES-GCM uses a 12-byte nonce
    let ciphertext = &encrypted_data[12..];

    // Decrypt the data
    cipher.decrypt(nonce.into(), ciphertext.as_ref())
}

// Add this function to compute the hash
fn compute_hash(v: &BigUint, w_j: &BigUint) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(v.to_bytes_be());
    hasher.update(w_j.to_bytes_be());
    let result = hasher.finalize();
    BigUint::from_bytes_be(&result)
}



// Modular inverse using Extended Euclidean Algorithm
fn modinv(a: &BigUint, modulus: &BigUint) -> Option<BigUint> {
    let a_int = a.to_bigint().unwrap();
    let m_int = modulus.to_bigint().unwrap();
    let (gcd, x, _) = extended_gcd(a_int.clone(), m_int.clone());
    if gcd.is_one() || gcd == BigInt::from(-1) {
        let x_mod = ((x % &m_int) + &m_int) % &m_int;
        Some(x_mod.to_biguint().unwrap())
    } else {
        None
    }
}

fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b, BigInt::zero(), BigInt::one())
    } else {
        let (gcd, x1, y1) = extended_gcd(&b % &a, a.clone());
        let x = y1 - (&b / &a) * &x1;
        let y = x1;
        (gcd, x, y)
    }
}

// Add this function for modular exponentiation
fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base.clone();
    let mut exp = exponent.clone();

    while !exp.is_zero() {
        if exp.bit(0) {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1;
    }

    result
}