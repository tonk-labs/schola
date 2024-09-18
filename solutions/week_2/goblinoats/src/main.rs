/**
 * This code was nearly one-shotted by Chat-GPT o1
 * 
 *     _.-^^---....,,--       
 * _--                  --_  
 * <                        >)
 * |                         | 
 * \._                   _./  
 *    ```--. . , ; .--'''       
 *          | |   |             
 *       .-=||  | |=-.   
 *       `-=#$%&%$#=-'   
 *          | ;  :|     
 *       _.-"O.o"O"-._
 *      /_.-'    '-._ \
 *     /    _     _   \
 *    /|  /         \ |\
 *   | | |_o      o_| | |
 *   | |     \___/     | |
 *   | \    ~~   ~~   / |
 *    \_\    (   )   /_/
 *       \__________/
 *         |_...._|
 *         \______/
 * 
 */
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{Zero, One};
use rand::thread_rng;

/// Generates random polynomial coefficients for the secret sharing.
/// The first coefficient is the secret itself.
///
/// # Arguments
///
/// * `threshold` - The minimum number of shares required to reconstruct the secret.
/// * `secret` - The secret to be shared.
/// * `prime` - A large prime number for modulo operations.
///
/// # Returns
///
/// A vector of `BigInt` coefficients for the polynomial.
fn generate_coefficients(threshold: usize, secret: &BigInt, prime: &BigInt) -> Vec<BigInt> {
    let mut coefficients = Vec::with_capacity(threshold);
    coefficients.push(secret.clone()); // a_0 = secret
    let mut rng = thread_rng();
    for _ in 1..threshold {
        let coeff = rng.gen_bigint_range(&BigInt::zero(), prime);
        coefficients.push(coeff);
    }
    coefficients
}

/// Evaluates the polynomial at a given x value.
///
/// # Arguments
///
/// * `x` - The x-coordinate at which to evaluate the polynomial.
/// * `coefficients` - The coefficients of the polynomial.
/// * `prime` - A large prime number for modulo operations.
///
/// # Returns
///
/// The y-coordinate corresponding to the given x.
fn evaluate_polynomial(x: &BigInt, coefficients: &[BigInt], prime: &BigInt) -> BigInt {
    let mut y = BigInt::zero();      // Initialize the result y to zero
    let mut x_pow = BigInt::one();   // Initialize x_pow to x^0 = 1
    for coeff in coefficients {
        // Compute the current term of the polynomial: coeff * x_pow mod prime
        let term = (coeff * &x_pow) % prime;
        // Add the current term to the result y modulo prime
        y = (y + term) % prime;
        // Update x_pow for the next term: x_pow = x_pow * x mod prime
        x_pow = (x_pow * x) % prime;
    }
    y   // Return the evaluated polynomial value y = f(x) mod prime
}


/// Creates shares for the secret.
///
/// # Arguments
///
/// * `num_shares` - The total number of shares to create.
/// * `threshold` - The minimum number of shares required to reconstruct the secret.
/// * `secret` - The secret to be shared.
/// * `prime` - A large prime number for modulo operations.
///
/// # Returns
///
/// A vector of tuples, each containing an x and y coordinate.
fn create_shares(
    num_shares: usize,
    threshold: usize,
    secret: &BigInt,
    prime: &BigInt,
) -> Vec<(BigInt, BigInt)> {
    let coefficients = generate_coefficients(threshold, secret, prime);
    let mut shares = Vec::with_capacity(num_shares);
    for i in 1..=num_shares {
        let x = i.to_bigint().unwrap();
        let y = evaluate_polynomial(&x, &coefficients, prime);
        shares.push((x, y));
    }
    shares
}

/// Computes the modular inverse of a number.
///
/// # Arguments
///
/// * `a` - The number to find the inverse of.
/// * `m` - The modulus.
///
/// # Returns
///
/// An `Option` containing the modular inverse if it exists.
fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    a.modinv(m)
}

/// Reconstructs the secret from a given set of shares using Lagrange interpolation.
///
/// # Arguments
///
/// * `shares` - A slice of tuples containing x and y coordinates.
/// * `prime` - A large prime number for modulo operations.
///
/// # Returns
///
/// The reconstructed secret as a `BigInt`.
fn reconstruct_secret(shares: &[(BigInt, BigInt)], prime: &BigInt) -> BigInt {
    let mut secret = BigInt::zero(); // Initialize the reconstructed secret to zero
    // Iterate over each share (xj, yj)
    for (j, (xj, yj)) in shares.iter().enumerate() {
        let mut numerator = BigInt::one();    // Initialize numerator for Lagrange basis polynomial
        let mut denominator = BigInt::one();  // Initialize denominator for Lagrange basis polynomial
        // Compute the Lagrange basis polynomial L_j(0)
        for (m, (xm, _)) in shares.iter().enumerate() {
            if m != j {
                // Update numerator: multiply by (-xm) mod prime
                numerator = (numerator * -xm) % prime;
                // Update denominator: multiply by (xj - xm) mod prime
                denominator = (denominator * (xj - xm)) % prime;
            }
        }
        // Compute the modular inverse of the denominator modulo prime
        let inv = modinv(&denominator, prime).unwrap();
        // Compute the term to be added: yj * numerator * inv mod prime
        let term = yj * numerator * inv;
        // Add the term to the secret modulo prime
        secret = (secret + term) % prime;
    }
    (secret + prime) % prime  // Ensure the secret is positive by adding prime if necessary
}

fn main() {
    // Define the secret and prime
    let secret = BigInt::from(123456789);
    // 2048-bit NIST Prime
    let prime = BigInt::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AACAA68FFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();

    use std::time::Instant;

    // Function to run and profile the secret sharing process
    fn run_and_profile(num_shares: usize, threshold: usize, secret: &BigInt, prime: &BigInt) {
        println!("\nRunning with {} shares and threshold {}", num_shares, threshold);
        
        let start = Instant::now();
        let shares = create_shares(num_shares, threshold, secret, prime);
        let create_time = start.elapsed();
        
        println!("Time to create shares: {:?}", create_time);

        let subset_of_shares = shares[..threshold].to_vec();
        let start = Instant::now();
        let recovered_secret = reconstruct_secret(&subset_of_shares, prime);
        let reconstruct_time = start.elapsed();

        println!("Time to reconstruct secret: {:?}", reconstruct_time);
        println!("Recovered secret: {}", recovered_secret);
        assert_eq!(*secret, recovered_secret, "Secret reconstruction failed!");
    }

    // Run scenarios with increasing number of shares
    for &num_shares in &[5, 15, 30, 50, 100, 200, 500] {
        let threshold = num_shares;  // Set threshold to majority of shares
        run_and_profile(num_shares, threshold, &secret, &prime);
    }
}