use primes::{PrimeSet, Sieve};
use rand::prelude::*;

fn main() {
    let secret = 42; // A random integer I chose manually for simplicity
    let shares = 3;

    let prime = generate_prime_number_greater_than(secret);
    let polynomial_coefficients = generate_polynomial(secret, shares, prime);

    let x: u64 = 59; // A random integer I chose manually for simplicity
    println!(
        "Evaluate polynomial, at x: {}, y: {}",
        x,
        evaluate_polynomial(&polynomial_coefficients, x, prime)
    )
}

fn generate_prime_number_greater_than(lower_bound: u64) -> u64 {
    let mut pset = Sieve::new();
    let seed = rand::thread_rng().gen_range(lower_bound..lower_bound + 1000000);
    let primes = pset
        .iter()
        .skip(seed.try_into().unwrap())
        .take(2)
        .collect::<Vec<_>>();
    let prime = primes[0];
    println!("Random prime number is: {}", prime);
    prime
}

fn generate_polynomial(secret: u64, shares: u64, prime: u64) -> Vec<u64> {
    // TODO: prime should be BigInt
    let mut coefficients = Vec::with_capacity(shares as usize);

    // The first coefficient of the polynomial is the secret
    coefficients.push(secret);

    // Generate random coefficients for the remaining terms
    let mut rng = rand::thread_rng();
    for _ in 1..shares {
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
