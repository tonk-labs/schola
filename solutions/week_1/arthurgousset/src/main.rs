use primes::{PrimeSet, Sieve};

fn main() {
    let mut pset = Sieve::new();

    for (ix, n) in pset.iter().enumerate().take(10) {
        println!("Prime {}: {}", ix, n);
    }
}
