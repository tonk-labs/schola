use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

/// Returns an integer equivalent in the finite field F_101 (i.e., between 0 and 100)
fn mod_101(x: BigInt) -> BigInt {
    let prime: BigInt = 101.to_bigint().unwrap();
    let result = x % &prime;
    
    // Ensure the result is positive
    if result < BigInt::zero() {
        result + prime
    } else {
        result
    }
}

/// Calculates the modular inverse of `a` mod `p` using Fermat's Little Theorem
fn mod_inverse(a: &BigInt, p: &BigInt) -> BigInt {
    // a^(p-2) mod p
    a.modpow(&(p - 2.to_bigint().unwrap()), p)
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    /// Creates a new point from x and y coordinates
    fn new(x: BigInt, y: BigInt) -> Self {
        Point { x, y }
    }

    /// Doubles a point on the elliptic curve
    fn double(&self) -> Self {
        let two = 2.to_bigint().unwrap();
        let three = 3.to_bigint().unwrap();
        let prime = 101.to_bigint().unwrap();

        // Compute the slope `m` as (3 * x^2) / (2 * y) in F_101
        let numerator = three.clone() * &self.x * &self.x;
        let denominator = two.clone() * &self.y;
        let denominator_inv = mod_inverse(&denominator, &prime);

        let m = mod_101(numerator * denominator_inv);
        println!("m: {:?}", m);

        // Calculate the new x and y coordinates using modular arithmetic
        let new_x = mod_101(&m * &m - two * &self.x);
        let new_y = mod_101(&m * (three * &self.x - &m * &m) - &self.y);

        Point { x: new_x, y: new_y }
    }

    /// Inverts a point on the elliptic curve
    fn invert(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: mod_101(-&self.y),
        }
    }
}

fn main() {
    let prime = 101.to_bigint().unwrap();
    let g1 = Point::new(1.to_bigint().unwrap(), 2.to_bigint().unwrap());

    println!("G1: {:?}", g1);

    let doubled = g1.double();
    println!("Doubled point: {:?}", doubled);

    let inverted = g1.invert();
    println!("Inverted point: {:?}", inverted);

    // Test the mod_101 function with a negative number
    let test = mod_101(-10.to_bigint().unwrap());
    println!("Modular equivalent of -10 in F_101: {}", test);
}
