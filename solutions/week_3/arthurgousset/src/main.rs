use ark_bls12_381::{Bls12_381, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::{AffineRepr, CurveGroup, pairing::Pairing};
use ark_ff::{Field, UniformRand, Zero, One};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial, univariate::DenseOrSparsePolynomial};
use ark_std::rand::SeedableRng;
use std::ops::Mul;

struct KZGCommitment {
    trusted_setup_g1: Vec<G1Affine>,
    trusted_setup_g2: Vec<G2Affine>,
}

impl KZGCommitment {
    fn new(degree: usize) -> Self {
        let (trusted_setup_g1, trusted_setup_g2) = Self::trusted_setup(degree);
        Self {
            trusted_setup_g1,
            trusted_setup_g2,
        }
    }

    fn trusted_setup(degree: usize) -> (Vec<G1Affine>, Vec<G2Affine>) {
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(0u64);
        let tau = Fr::rand(&mut rng);
        let mut trusted_setup_g1 = Vec::with_capacity(degree);
        let mut trusted_setup_g2 = Vec::with_capacity(degree);
        
        for i in 0..degree {
            let tau_i = tau.pow([i as u64]);
            trusted_setup_g1.push((G1Affine::generator() * tau_i).into_affine());
            trusted_setup_g2.push((G2Affine::generator() * tau_i).into_affine());
        }

        (trusted_setup_g1, trusted_setup_g2)
    }

    fn lagrange_interpolation(points: &[(Fr, Fr)]) -> DensePolynomial<Fr> {
        let mut result = DensePolynomial::zero();
        for (i, &(x_i, y_i)) in points.iter().enumerate() {
            let mut term = DensePolynomial::from_coefficients_vec(vec![y_i]);
            for (j, &(x_j, _)) in points.iter().enumerate() {
                if i != j {
                    let scalar = (x_i - x_j).inverse().unwrap();
                    let factor = DensePolynomial::from_coefficients_vec(vec![-x_j * scalar, scalar]);
                    term = &term * &factor;
                }
            }
            result += &term;
        }
        result
    }

    fn commit_polynomial(&self, polynomial: &DensePolynomial<Fr>) -> G1Affine {
        let mut result = G1Projective::zero();
        for (i, coeff) in polynomial.coeffs().iter().enumerate() {
            result += self.trusted_setup_g1[i] * coeff;
        }
        result.into_affine()
    }

    fn open(&self, polynomial: &DensePolynomial<Fr>, x: Fr) -> (Fr, G1Affine) {
        let y = polynomial.evaluate(&x);
        let points = vec![(x, y)];
        let point_poly = Self::lagrange_interpolation(&points);
        let numerator = polynomial - &point_poly;
        let denominator = DensePolynomial::from_coefficients_vec(vec![-x, Fr::one()]);
        
        let (quotient, remainder) = DenseOrSparsePolynomial::from(numerator)
            .divide_with_q_and_r(&DenseOrSparsePolynomial::from(denominator))
            .unwrap();
        
        assert!(remainder.is_zero(), "Remainder should be zero");
        
        let proof = self.commit_polynomial(&DensePolynomial::from(quotient));
        (y, proof)
    }

    fn verify(&self, commitment: &G1Affine, x: Fr, y: Fr, proof: &G1Affine) -> bool {
        let g2 = G2Affine::generator();
        let x_g2 = g2.mul(x).into_affine();
        
        let lhs_g1 = commitment.into_group() - G1Affine::generator().mul(y);
        let rhs_g2 = self.trusted_setup_g2[1] - x_g2;
        
        println!("LHS G1: {:?}", lhs_g1);
        println!("Proof: {:?}", proof);
        println!("RHS G2: {:?}", rhs_g2);
        
        let lhs = Bls12_381::pairing(lhs_g1, g2);
        let rhs = Bls12_381::pairing(*proof, rhs_g2);

        println!("LHS pairing: {:?}", lhs);
        println!("RHS pairing: {:?}", rhs);

        lhs == rhs
    }
}

fn main() {
    let degree = 10;
    let kzg = KZGCommitment::new(degree + 1);

    let polynomial = DensePolynomial::from_coefficients_vec(vec![Fr::one(), Fr::one(), Fr::one()]); // x^2 + x + 1
    let commitment = kzg.commit_polynomial(&polynomial);

    let x = Fr::from(2u32);
    let (y, proof) = kzg.open(&polynomial, x);

    println!("Polynomial: x^2 + x + 1");
    println!("Evaluation point: 2");
    println!("Raw evaluation result: {:?}", y);
    println!("Expected result: 7"); // 2^2 + 2 + 1 = 7
    println!("Commitment: {:?}", commitment);
    println!("Proof: {:?}", proof);

    let is_valid = kzg.verify(&commitment, x, y, &proof);
    println!("Proof is valid: {}", is_valid);
}
