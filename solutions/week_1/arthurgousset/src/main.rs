use primes::{PrimeSet, Sieve};

fn main() {
    let (p, q) = generate_prime_numbers();
    let _n = p * q; // the "modulus"
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
