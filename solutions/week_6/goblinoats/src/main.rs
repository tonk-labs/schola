use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use num_bigint_dig::{BigUint, RandBigInt, ModInverse};
use num_integer::Integer;
use num_traits::{One, Zero};
use std::error::Error;
use rsa::traits::{PublicKeyParts,PrivateKeyParts};
use sha2::{Sha256, Digest};

fn main() -> Result<(), Box<dyn Error>> {
    // Generate RSA keys
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);
    let n = public_key.n(); // Modulus
    let e = public_key.e(); // Public exponent
    let d = private_key.d(); // Private exponent

    // User's message m (should be less than n)
    let m = rng.gen_biguint_below(n);

    // Hash the message
    let m_hash = Sha256::digest(m.to_bytes_be());
    let m_hash = BigUint::from_bytes_be(&m_hash);

    // User generates blinding factor r, such that gcd(r, n) = 1
    let mut r;
    loop {
        r = rng.gen_biguint_below(n);
        if r.gcd(n).is_one() {
            break;
        }
    }

    // Compute blinded message m' = H(m) * r^e mod n
    let re = r.modpow(e, n);
    let m_blinded = (&m_hash * &re) % n;

    // Signer signs the blinded message to get s' = (m')^d mod n
    let s_blinded = m_blinded.modpow(d, n);

    // User computes the unblinded signature s = s' * r^{-1} mod n
    let r_inv = r.mod_inverse(n).ok_or("Failed to compute modular inverse")?;
    let s = ((&s_blinded * &r_inv).to_biguint().unwrap() % n);

    // Verify the signature: check that s^e mod n == H(m) mod n
    let m_recovered = s.modpow(e, n);

    if m_recovered == m_hash {
        println!("Signature verified successfully!");
    } else {
        println!("Signature verification failed.");
    }

    Ok(())
}