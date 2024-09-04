use primes::{PrimeSet, Sieve};

fn generate_prime_numbers() -> (u64, u64) {
    let mut pset = Sieve::new();
    let primes = pset.iter().skip(1_000_000).take(2).collect::<Vec<_>>();
    let p = primes[0];
    let q = primes[1];

    println!("Prime p: {}", p);
    println!("Prime q: {}", q);

    (p, q)
}

fn main() {
    let (p, q) = generate_prime_numbers();
    println!("Successfully deconstructed values {} and {}", p, q);
}
