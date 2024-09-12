use num_bigint::BigUint;
use num_traits::ToPrimitive;
use primes::{PrimeSet, Sieve};
use rand::prelude::*;

fn main() {
    let secret = 1559; // A random integer I chose manually for simplicity
    println!("Secret is: {}", secret);

    let shares = 6;
    println!("Number of shares are: {}", shares);

    let prime = generate_prime_number_greater_than(secret);
    let secret_shares = generate_secret_shares(secret, shares, prime);
    println!("Secret shares: {:?}", secret_shares);

    let reconstructed_secret = lagrange_interpolation(&secret_shares, prime);
    println!("Reconstructed secret: {}", reconstructed_secret);
}

fn generate_prime_number_greater_than(lower_bound: u64) -> u64 {
    let mut pset = Sieve::new();
    let seed = rand::thread_rng().gen_range(lower_bound..lower_bound + 100);
    let primes = pset
        .iter()
        .skip(seed.try_into().unwrap())
        .take(2)
        .collect::<Vec<_>>();
    let prime = primes[0];
    println!("Random prime number is: {}", prime);
    prime
}

fn generate_polynomial(secret: u64, nr_shares: u64, prime: u64) -> Vec<u64> {
    // TODO: prime should be BigInt
    let mut coefficients = Vec::with_capacity(nr_shares as usize);

    // The first coefficient of the polynomial is the secret
    coefficients.push(secret);

    // Generate random coefficients for the remaining terms
    let mut rng = rand::thread_rng();
    for _ in 1..nr_shares {
        // Generate a random number in the range [0, prime)
        let random_coefficient = rng.gen_range(0..prime);
        coefficients.push(random_coefficient);
    }

    println!("Polynomial coefficients are: {:?}", coefficients);

    coefficients
}

fn evaluate_polynomial(coefficients: &[u64], x: u64, prime: u64) -> u64 {
    let mut y = 0;
    for (i, &coefficient) in coefficients.iter().enumerate() {
        y += coefficient * x.pow(i.try_into().unwrap());
        y %= prime; // Calculate modulo prime to ensure values stay in the finite field
    }
    y
}

fn generate_secret_shares(secret: u64, nr_shares: u64, prime: u64) -> Vec<(u64, u64)> {
    let polynomial_coefficients = generate_polynomial(secret, nr_shares, prime);
    let mut secret_shares: Vec<(u64, u64)> = Vec::with_capacity(nr_shares as usize);

    for i in 1..nr_shares+1 {
        let x = i;
        let y = evaluate_polynomial(&polynomial_coefficients, x, prime);
        secret_shares.push((x, y));
    }
    secret_shares
}

fn lagrange_interpolation(shares: &[(u64, u64)], prime: u64) -> u64 {
    let mut secret = 0;

    for (i, &(x_i, y_i)) in shares.iter().enumerate() {
        let mut numerator = 1;
        let mut denominator = 1;

        for (j, &(x_j, _)) in shares.iter().enumerate() {
            if i != j {
                let prime_big = BigUint::from(prime);
                let x_i_big = BigUint::from(x_i);
                let x_j_big = BigUint::from(x_j);

                let numerator_big = BigUint::from(numerator);
                let denominator_big = BigUint::from(denominator);

                let new_numerator = (numerator_big * &x_j_big) % &prime_big;
                let diff = if x_j_big >= x_i_big {
                    x_j_big - x_i_big
                } else {
                    &prime_big - (x_i_big - x_j_big) % &prime_big
                };
                let new_denominator = (denominator_big * diff) % &prime_big;

                numerator = new_numerator.to_u64().unwrap();
                denominator = new_denominator.to_u64().unwrap();
            }
        }

        // Calculate modular inverse of denominator modulo prime
        let inv_denominator = match modular_multiplicative_inverse(denominator, prime) {
            Some(d) => d, // the "private exponent"
            None => {
                println!(
                    "No modular multiplicative inverse exists for {} mod {}",
                    denominator, prime
                );
                std::process::exit(1); // Exit with error code
            }
        };

        // Add current term to the secret
        let term = (y_i * numerator * inv_denominator) % prime;
        secret = (secret + term) % prime;
    }

    // Ensure secret is positive
    (secret + prime) % prime
}

/// Helper function to calculate modular multiplicative inverse.
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

/// Helper function to calculate GCD.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y) = extended_gcd(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}
