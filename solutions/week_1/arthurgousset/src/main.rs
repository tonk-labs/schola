use primes::{PrimeSet, Sieve};
use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    let (p, q) = generate_prime_numbers();
    let n = p * q; // the "modulus"
    let k = (p - 1) * (q - 1); // the "totient"
    let e = 65537 as u64; // the "public exponent"

    let d = match modular_multiplicative_inverse(e, k) {
        Some(d) => d, // the "private exponent"
        None => {
            println!(
                "No modular multiplicative inverse exists for {} mod {}",
                e, k
            );
            std::process::exit(1); // Exit with error code
        }
    };
    println!(
        "The modular multiplicative inverse of {} mod {} is {}",
        e, k, d
    );

    let public_key = (n, e);
    let private_key = (n, d);

    let message = "Hello, world!";

    let encrypted_message = encrypt_message(message, public_key);
    println!("Encrypted message: {:?}", encrypted_message);

    let decrypted_message = decrypt_message(encrypted_message, private_key);
    println!("Decrypted message: {}", decrypted_message);
}

fn generate_prime_numbers() -> (u64, u64) {
    let mut pset = Sieve::new();
    let primes = pset.iter().skip(1_000_000).take(2).collect::<Vec<_>>();
    let p = primes[0];
    let q = primes[1];

    println!("Prime p: {}", p);
    println!("Prime q: {}", q);

    (p, q)
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y) = extended_gcd(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

fn modular_multiplicative_inverse(e: u64, k: u64) -> Option<u64> {
    let (gcd, x, _) = extended_gcd(e as i64, k as i64);

    // Check if the gcd is 1
    if gcd != 1 {
        // No modular inverse if gcd is not 1
        return None;
    }

    // Ensure x is positive
    let mut d = x % k as i64;
    if d < 0 {
        d += k as i64;
    }

    Some(d as u64)
}

fn encrypt_message(message: &str, public_key: (u64, u64)) -> Vec<u64> {
    let (n, e) = public_key;
    let mut encrypted_message = Vec::new();

    for char in message.chars() {
        let m = char as u64; // Convert the character to its ASCII value
        let c = mod_exp(m, e, n); // Encrypt the ASCII value
        encrypted_message.push(c);
    }

    encrypted_message
}

fn decrypt_message(ciphertext: Vec<u64>, private_key: (u64, u64)) -> String {
  let (n, d) = private_key;
  let mut decrypted_message = String::new();

  for &c in ciphertext.iter() {
      let m = mod_exp(c, d, n); // Decrypt each encrypted number
      let char = std::char::from_u32(m as u32).unwrap(); // Convert the number back to a character
      decrypted_message.push(char);
  }

  decrypted_message
}

// Helper function for modular exponentiation
fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
  let mut result = BigUint::one();
  let mut base = BigUint::from(base);
  let modulus = BigUint::from(modulus);
  let mut exp = BigUint::from(exp);

  while !exp.is_zero() {
      if &exp % 2u8 == BigUint::one() {
          result = (result * &base) % &modulus;
      }
      exp >>= 1;
      base = (&base * &base) % &modulus;
  }

  result.to_u64_digits()[0] // Convert BigUint back to u64
}
