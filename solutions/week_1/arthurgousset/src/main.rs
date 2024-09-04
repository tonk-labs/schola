use primes::{PrimeSet, Sieve};

fn main() {
    let mut pset = Sieve::new();
    let primes = pset.iter().skip(1_000_000).take(2).collect::<Vec<_>>();
    let p = primes[0];
    let q = primes[1];

    println!("Prime p: {}", p);
    println!("Prime q: {}", q);
}
