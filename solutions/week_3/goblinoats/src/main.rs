use ark_bls12_381::{Bls12_381, Fr as ScalarField};
use ark_ff::UniformRand; 
use ark_poly::{univariate::DensePolynomial, Polynomial, DenseUVPolynomial};
use goblinoats::kzg10::{KZG10, KZGParams};

fn main() {
    println!("Starting KZG10 polynomial commitment demonstration");

    // Maximum degree of polynomials
    let max_degree = 5;
    println!("Setting up KZG10 with max degree: {}", max_degree);

    let mut rng = ark_std::rand::thread_rng();
    // Setup
    let params: KZGParams<Bls12_381> = KZG10::<Bls12_381, DensePolynomial<ScalarField>>::setup(&mut rng, max_degree);
    println!("KZG10 setup completed");

    // Define a polynomial f(x)
    let coeffs = vec![
        ScalarField::rand(&mut rng),
        ScalarField::rand(&mut rng),
        ScalarField::rand(&mut rng),
    ]; // degree 2 polynomial
    println!("Generated random polynomial of degree 2");
    println!("Coefficients: {:?}", coeffs);

    let poly = DensePolynomial::from_coefficients_slice(&coeffs);

    // Commit to the polynomial
    let commitment = KZG10::commit(&params, &poly);
    println!("Created commitment to the polynomial");
    println!("Commitment: {:?}", commitment);

    // Choose a random point z
    let z = ScalarField::rand(&mut rng);
    println!("Chose random point z for evaluation: {:?}", z);

    // Evaluate f(z)
    let y = poly.evaluate(&z);
    println!("Evaluated polynomial at z: f(z) = y");
    println!("y = {:?}", y);

    // Open the commitment at point z
    let proof = KZG10::open(&params, &poly, z);
    println!("Generated proof for f(z) = y");
    println!("Proof: {:?}", proof);

    // Verify the proof
    let is_valid = KZG10::<Bls12_381, DensePolynomial<ScalarField>>::verify(&params, commitment, z, y, proof);
    println!("Verifying the proof...");
    println!("Verification result: {}", is_valid);

    let fake_y = ScalarField::rand(&mut rng);
    let is_not_valid = KZG10::<Bls12_381, DensePolynomial<ScalarField>>::verify(&params, commitment, z, fake_y, proof);
    println!("Testing the proof with fake y...");
    println!("Verification result: {}", is_not_valid);
    
    if is_valid && !is_not_valid {
        println!("KZG10 polynomial commitment verification succeeded!");
    } else {
        println!("KZG10 polynomial commitment verification failed!");
    }

}

