use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Prime modulus
pub const PRIME: i32 = 101;

// Re-export all the types and functions we want to make public
pub mod field;
pub mod points;
pub mod srs;

// Move utility functions here
pub fn mod_prime(x: &BigInt, modulus: &BigInt) -> BigInt {
    let result = x % modulus;
    if result < BigInt::zero() {
        result + modulus
    } else {
        result
    }
}

pub fn mod_inverse(a: &BigInt, p: &BigInt) -> BigInt {
    let exp = p - 2;
    a.modpow(&exp, p)
} 