use ark_ec::pairing::Pairing;
use ark_ff::{One, UniformRand, Zero};
use ark_poly::DenseUVPolynomial;
use ark_std::{ops::Div, ops::Mul, vec};

use ark_std::rand::RngCore;
use std::marker::PhantomData;

/// [kzg10]: http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
pub struct KZG10<E: Pairing, P: DenseUVPolynomial<E::ScalarField>> { 
    _engine: PhantomData<E>,
    _poly: PhantomData<P>,
}

pub struct KZGParams<E: Pairing> {
    /// Group elements of the form `{ \beta^i G }`, where `i` ranges from 0 to `degree`.
    pub powers_of_g: Vec<E::G1>,
    pub g1: E::G1,
    /// The generator of G2.
    pub g2: E::G2,
    /// \beta times the above generator of G2.
    pub beta_g2: E::G2,
}

impl<E, P> KZG10<E, P>
where
    E: Pairing,
    P: DenseUVPolynomial<E::ScalarField, Point = E::ScalarField>,
    for<'a, 'b> &'a P: Div<&'b P, Output = P>,
{
    pub fn setup<R: RngCore>(rng: &mut R, max_degree: usize) -> KZGParams<E> {
        // Sample secret s
        let s = E::ScalarField::rand(rng);

        // G1 and G2 generators
        let g1: E::G1 = E::G1::rand(rng);
        let g2: E::G2 = E::G2::rand(rng);

        // Compute powers of s in G1: [s^i]G1
        let mut powers_of_g = Vec::with_capacity(max_degree + 1);
        let mut current_power = E::ScalarField::one();

        for _ in 0..=max_degree {
            powers_of_g.push(g1.mul(current_power));
            current_power *= &s;
        }

        // Compute beta_g2 = sG2
        let beta_g2 = g2.mul(s);

        KZGParams {
            g1: g1,
            g2: g2,
            powers_of_g,
            beta_g2,
        }
    }

    pub fn commit(params: &KZGParams<E>, poly: &P) -> E::G1 {
        let coeffs = poly.coeffs();
        let commitment = params
            .powers_of_g
            .iter()
            .zip(coeffs)
            .fold(E::G1::zero(), |acc, (power, coeff)| acc + power.mul(coeff));
        commitment
    }

    pub fn open(params: &KZGParams<E>, poly: &P, z: E::ScalarField) -> E::G1 {
        // Compute f(x) - f(z)
        let y = poly.evaluate(&z);
        let mut f_minus_y = poly.clone();
        f_minus_y -= &P::from_coefficients_vec(vec![y]);

        // Compute denominator x - z
        let denom = P::from_coefficients_vec(vec![-z, E::ScalarField::one()]);

        // Compute quotient q(x)
        let q = &f_minus_y / &denom;

        // Now, compute the commitment to q(x)
        Self::commit(params, &q)
    }

    pub fn verify(
        params: &KZGParams<E>,
        commitment: E::G1,
        z: E::ScalarField,
        y: E::ScalarField,
        pi: E::G1,
    ) -> bool {
        // Compute C - [y]G1
        let g1 = params.powers_of_g[0];
        let y_g1 = g1.mul(&y);
        let c_minus_y_g = commitment - y_g1;

        // Compute [s - z]G2 = sG2 - zG2
        let z_g2 = params.g2.mul(&z);
        let s_minus_z_g2 = params.beta_g2 - z_g2;

        // Compute pairings
        let lhs = E::pairing(c_minus_y_g, params.g2);
        let rhs = E::pairing(pi, s_minus_z_g2);

        lhs == rhs 
    }
}
