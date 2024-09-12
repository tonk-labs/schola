# Shamir's Secret Sharing

## Implementation

### Dependencies

1. Uses `rand` for random number generation to ensure the generated primes are different.
1. Uses [`primes`](https://docs.rs/primes/latest/primes/) dependency to generate prime numbers.
1. Uses `num-bigint` and `num-traits` to work with large integers.

### Learnings

Difference between `&[u64]` (borrowed slice) and `Vec<u64>` (owned vector).

### Limitations

1. I'm working with small primes. To work with larger primes, I'd need to use `BigInt` from
   `num-bigint`.

## Theory

$S = \sum_{j=1}^{t} y_j \prod_{1 \leq m \leq t, m \neq j} \frac{x_m}{x_m - x_j} \mod p$
