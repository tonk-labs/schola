use ark_bls12_381::{Bls12_381, Fr, G1Projective, G2Affine, G2Projective};
use ark_ec::pairing::Pairing;
use ark_ec::{AffineRepr, CurveGroup, Group, VariableBaseMSM};
use ark_ff::{One, UniformRand, Zero};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use ark_std::rand::Rng;
use rand::thread_rng;

/// KZG Public Parameters.
///
/// This struct contains the public parameters needed for the KZG commitment scheme.
/// It includes precomputed powers of the group generators raised to secret exponents.
struct KZGParams {
    // powers of g1: [g1^{s^0}, g1^{s^1}, ..., g1^{s^n}]
    g1_powers: Vec<G1Projective>,
    // g2^{s}
    g2_s: G2Projective,
}

/// KZG Commitment.
///
/// Represents a commitment to a polynomial in the KZG scheme.
struct KZGCommitment(G1Projective);

/// KZG Proof.
///
/// Represents a proof that a committed polynomial evaluates to a certain value at a point.
struct KZGProof(G1Projective);

/// Generates the KZG public parameters.
///
/// This function performs a trusted setup by generating the secret `s` and computing
/// the necessary public parameters for the commitment scheme.
///
/// # Arguments
///
/// * `max_degree` - The maximum degree of the polynomials that can be committed.
/// * `rng` - A random number generator.
///
/// # Returns
///
/// A `KZGParams` struct containing the public parameters.
fn setup<R: Rng>(max_degree: usize, rng: &mut R) -> KZGParams {
    // sample a random secret `s`
    let s = Fr::rand(rng);

    // precompute powers of `s`
    let mut s_powers = vec![Fr::one()]; // s^0
    for i in 1..=max_degree {
        s_powers.push(s_powers[i - 1] * s);
    }

    // compute g1^{}
    let g1 = G1Projective::generator();
    let g1_powers: Vec<G1Projective> = s_powers.iter().map(|s_i| g1 * s_i).collect();

    // compute g2^{s}
    let g2 = G2Projective::generator();
    let g2_s = g2 * s;

    KZGParams { g1_powers, g2_s }
}

/// Commits to a polynomial using the KZG commitment scheme.
///
/// The commitment is computed as a linear combination of the public parameters
/// and the coefficients of the polynomial.
///
/// # Arguments
///
/// * `poly` - The polynomial to commit to.
/// * `params` - The KZG public parameters.
///
/// # Returns
///
/// A `KZGCommitment` representing the commitment to the polynomial.
fn commit(poly: &DensePolynomial<Fr>, params: &KZGParams) -> KZGCommitment {
    let coeffs = &poly.coeffs;
    let g1_powers = &params.g1_powers[..coeffs.len()];

    // compute the commitment as a multi-scalar multiplication
    let commitment = G1Projective::msm_unchecked(
        &g1_powers
            .iter()
            .map(|p| p.into_affine())
            .collect::<Vec<_>>(),
        &coeffs,
    );

    KZGCommitment(commitment)
}

/// Divides a polynomial by a linear term `(x - z)` using synthetic division.
///
/// This function performs efficient division of a polynomial by a linear divisor,
/// returning the quotient polynomial and the remainder.
///
/// # Arguments
///
/// * `numerator` - The numerator polynomial.
/// * `z` - The evaluation point `z`.
///
/// # Returns
///
/// A tuple containing:
/// - The quotient polynomial.
/// - The remainder of the division.
fn divide_by_linear(numerator: &DensePolynomial<Fr>, z: Fr) -> (DensePolynomial<Fr>, Fr) {
    let mut quotient_coeffs = Vec::with_capacity(numerator.degree());
    let mut coeff = Fr::zero();

    for &c in numerator.coeffs.iter().rev() {
        coeff = c + z * coeff;
        quotient_coeffs.push(coeff);
    }

    // The last value of coeff is the remainder
    let remainder = quotient_coeffs.pop().unwrap();

    // Reverse to get the correct order
    quotient_coeffs.reverse();

    (
        DensePolynomial::from_coefficients_vec(quotient_coeffs),
        remainder,
    )
}

/// Generates a proof of evaluation for a polynomial at a point `z`.
///
/// This function computes a witness polynomial and generates a proof that the
/// committed polynomial evaluates to a certain value at the point `z`.
///
/// # Arguments
///
/// * `poly` - The polynomial `p(x)`.
/// * `z` - The point at which the polynomial is evaluated.
/// * `params` - The KZG public parameters.
///
/// # Returns
///
/// A `KZGProof` representing the proof of evaluation.
fn open(poly: &DensePolynomial<Fr>, z: Fr, params: &KZGParams) -> KZGProof {
    // Compute p(z)
    let p_z = poly.evaluate(&z);

    // Compute numerator polynomial: p(x) - p(z)
    let numerator = poly - &DensePolynomial::from_coefficients_vec(vec![p_z]);

    // Perform division by (x - z)
    let (witness_poly, remainder) = divide_by_linear(&numerator, z);

    // Ensure that the remainder is zero
    assert!(
        remainder.is_zero(),
        "Polynomial division had a non-zero remainder"
    );

    // Commit to the witness polynomial
    let proof = commit(&witness_poly, params);

    KZGProof(proof.0)
}

/// Verifies a proof that a committed polynomial evaluates to a given value at point `z`.
///
/// This function checks the pairing equation to verify that the proof corresponds
/// to the claimed evaluation of the committed polynomial.
///
/// # Arguments
///
/// * `commitment` - The commitment to the polynomial.
/// * `z` - The evaluation point.
/// * `value` - The claimed value `p(z)`.
/// * `proof` - The proof of evaluation.
/// * `params` - The KZG public parameters.
///
/// # Returns
///
/// `true` if the proof is valid, `false` otherwise.
fn verify(
    commitment: &KZGCommitment,
    z: Fr,
    value: Fr,
    proof: &KZGProof,
    params: &KZGParams,
) -> bool {
    // compute commitment: g1^{value}
    let g1_value = G1Projective::generator() * value;
    let commitment_minus_value = commitment.0 - g1_value;

    // compute g2^{s - z}
    let g2_z = G2Projective::generator() * z;
    let g2_s_minus_z = params.g2_s - g2_z;

    let pairing_check =
        Bls12_381::pairing(commitment_minus_value.into_affine(), G2Affine::generator())
            == Bls12_381::pairing(proof.0.into_affine(), g2_s_minus_z.into_affine());

    pairing_check
}

/// Example usage of the KZG commitment scheme.
///
/// This function demonstrates how to set up the parameters, commit to a polynomial,
/// generate a proof of evaluation, and verify the proof.
fn main() {
    let rng = &mut thread_rng();
    let max_degree = 5;
    let params = setup(max_degree, rng);

    // Define polynomial p(x) = 6x^2 + 4x + 2
    let poly = DensePolynomial::from_coefficients_vec(vec![
        Fr::from(2u64),
        Fr::from(4u64),
        Fr::from(6u64),
    ]);

    // Commit to the polynomial
    let commitment = commit(&poly, &params);

    // Choose a random evaluation point z
    let z = Fr::rand(rng);
    let value = poly.evaluate(&z);

    // Generate a proof of evaluation
    let proof = open(&poly, z, &params);

    // Verify the proof
    let is_valid = verify(&commitment, z, value, &proof, &params);

    assert!(is_valid, "Proof verification failed");
    println!("Proof verified successfully!");
}
