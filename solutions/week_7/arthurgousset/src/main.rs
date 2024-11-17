use num_bigint::BigInt;
use week_7_arthurgousset::{mod_inverse, mod_prime, points::Point, srs::setup_srs, srs::SrsPoint};

fn compute_interpolation_matrix(domain: &[BigInt], modulus: &BigInt) -> Vec<Vec<BigInt>> {
    let n = domain.len();
    let mut matrix = vec![vec![BigInt::from(0); n]; n];

    // Build Vandermonde matrix
    // Each row i represents: [1, x_i, x_i^2, x_i^3]
    for i in 0..n {
        let x = &domain[i];
        matrix[i][0] = BigInt::from(1); // x^0
        matrix[i][1] = mod_prime(&x, modulus); // x^1
        matrix[i][2] = mod_prime(&(&matrix[i][1] * x), modulus); // x^2
        matrix[i][3] = mod_prime(&(&matrix[i][2] * x), modulus); // x^3
    }

    // Compute inverse modulo 17
    invert_matrix(&matrix, modulus)
}

fn invert_matrix(matrix: &Vec<Vec<BigInt>>, modulus: &BigInt) -> Vec<Vec<BigInt>> {
    let n = matrix.len();
    let mut augmented = vec![vec![BigInt::from(0); 2 * n]; n];

    // Create augmented matrix [A|I]
    for i in 0..n {
        for j in 0..n {
            augmented[i][j] = matrix[i][j].clone();
        }
        augmented[i][i + n] = BigInt::from(1);
    }

    // Gaussian elimination
    for i in 0..n {
        // Find pivot
        let pivot = &augmented[i][i];
        let pivot_inv = mod_inverse(pivot, modulus);

        // Scale row i
        for j in 0..2 * n {
            augmented[i][j] = mod_prime(&(&augmented[i][j] * &pivot_inv), modulus);
        }

        // Eliminate column i
        for k in 0..n {
            if k != i {
                let factor = augmented[k][i].clone();
                for j in 0..2 * n {
                    let temp = mod_prime(&(&factor * &augmented[i][j]), modulus);
                    augmented[k][j] = mod_prime(&(&augmented[k][j] - &temp), modulus);
                }
            }
        }
    }

    // Extract right half (the inverse matrix)
    let mut inverse = vec![vec![BigInt::from(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            inverse[i][j] = augmented[i][j + n].clone();
        }
    }

    inverse
}

fn compute_polynomial_coefficients(
    values: &[BigInt],
    domain: &[BigInt],
    modulus: &BigInt,
) -> Vec<BigInt> {
    let interpolation_matrix = compute_interpolation_matrix(domain, modulus);
    let mut coefficients = vec![BigInt::from(0); 4];

    for i in 0..4 {
        for j in 0..4 {
            let term = mod_prime(&(&interpolation_matrix[i][j] * &values[j]), modulus);
            coefficients[i] = mod_prime(&(&coefficients[i] + &term), modulus);
        }
    }

    coefficients
}

/// Copy constraints are specific to each circuit, so we need to manually compute them
/// based on the circuit structure. It's totally fine that the assignments appear arbitrary.
/// This is specific to the pythagorean circuit.
///
///       a^2     +      b^2      =    c^2
/// 
///  a1        b1  a2         b2  
///   \        /     \        /     
///    \      /       \      /       
///     c1 = a4        c2 = b4       a3    b3
///        \           /               \   /
///         \         /                 \ /
///              c4               =      c3  
fn compute_copy_constraints(
    H: &[BigInt],
    K1H: &[BigInt],
    K2H: &[BigInt],
    modulus: &BigInt,
) -> (Vec<BigInt>, Vec<BigInt>, Vec<BigInt>) {
    // Initialize sigma vectors
    let mut sigma1 = Vec::with_capacity(4);
    let mut sigma2 = Vec::with_capacity(4);
    let mut sigma3 = Vec::with_capacity(4);

    let a = H;     // a is mapped by H
    let b = K1H;   // b is mapped by K1H
    let c = K2H;   // c is mapped by K2H

    // For gates 1-3: connect a[i] to b[i]
    for i in 0..3 {
        sigma1.push(mod_prime(&b[i], modulus));  // a[i+1] = b[i+1]
        sigma2.push(mod_prime(&a[i], modulus));  // b[i+1] = a[i+1]
    }

    // For gate 4: connect outputs to inputs
    sigma1.push(mod_prime(&c[0], modulus));  // a[4] = c[1]
    sigma2.push(mod_prime(&c[1], modulus));  // b[4] = c[2]

    // Connect gate outputs
    sigma3.push(mod_prime(&a[3], modulus));  // c[1] = a[4]
    sigma3.push(mod_prime(&b[3], modulus));  // c[2] = b[4]
    sigma3.push(mod_prime(&c[3], modulus));  // c[3] = c[4]
    sigma3.push(mod_prime(&c[2], modulus));  // c[4] = c[3]

    (sigma1, sigma2, sigma3)
}

fn compute_zh_coefficients(modulus: &BigInt) -> Vec<BigInt> {
    // ZH(x) = x^4 - 1
    // Coefficients are [constant term, x^1 term, x^2 term, x^3 term, x^4 term]
    vec![
        mod_prime(&BigInt::from(-1), modulus), // -1
        BigInt::from(0),                       // 0x
        BigInt::from(0),                       // 0x^2
        BigInt::from(0),                       // 0x^3
        BigInt::from(1),                       // 1x^4
    ]
}

fn compute_round1_polynomials(
    coeffs_a: &[BigInt],
    coeffs_b: &[BigInt],
    coeffs_c: &[BigInt],
    zh_coeffs: &[BigInt],
    modulus: &BigInt,
) -> (Vec<BigInt>, Vec<BigInt>, Vec<BigInt>) {
    // Define specific constants used in the construction
    let beta1 = BigInt::from(7); // b₁
    let beta2 = BigInt::from(4); // b₂
    let beta3 = BigInt::from(11); // b₃
    let alpha1 = BigInt::from(12); // b₄
    let alpha2 = BigInt::from(16); // b₅
    let alpha3 = BigInt::from(2); // b₆

    // Helper function to multiply Z_H(x) by (b_i * x + b_j)
    fn apply_zh_poly(
        zh_coeffs: &[BigInt],
        x_coeff: &BigInt,
        constant: &BigInt,
        modulus: &BigInt,
    ) -> Vec<BigInt> {
        let mut result = vec![BigInt::from(0); zh_coeffs.len() + 1];
        
        // Multiply each term by the constant
        for (i, zh_coeff) in zh_coeffs.iter().enumerate() {
            let term = mod_prime(&(zh_coeff * constant), modulus);
            result[i] = mod_prime(&(result[i].clone() + term), modulus);
        }
        
        // Multiply each term by (x_coeff * x)
        for (i, zh_coeff) in zh_coeffs.iter().enumerate() {
            let term = mod_prime(&(zh_coeff * x_coeff), modulus);
            result[i + 1] = mod_prime(&(result[i + 1].clone() + term), modulus);
        }
        
        result
    }

    // Compute a(x) = (b₁ * x + b₂) * Z_H(x) + f_a(x)
    let mut a_x = apply_zh_poly(zh_coeffs, &beta1, &beta2, modulus);
    for (i, coeff) in coeffs_a.iter().enumerate() {
        a_x[i] = mod_prime(&(a_x[i].clone() + coeff), modulus);
    }

    // Compute b(x) = (b₃ * x + b₄) * Z_H(x) + f_b(x)
    let mut b_x = apply_zh_poly(zh_coeffs, &beta3, &alpha1, modulus);
    for (i, coeff) in coeffs_b.iter().enumerate() {
        b_x[i] = mod_prime(&(b_x[i].clone() + coeff), modulus);
    }

    // Compute c(x) = (b₅ * x + b₆) * Z_H(x) + f_c(x)
    let mut c_x = apply_zh_poly(zh_coeffs, &alpha2, &alpha3, modulus);
    for (i, coeff) in coeffs_c.iter().enumerate() {
        c_x[i] = mod_prime(&(c_x[i].clone() + coeff), modulus);
    }

    (a_x, b_x, c_x)
}

fn evaluate_polynomial_with_srs(coeffs: &[BigInt], srs: &[SrsPoint]) -> Point {
    let mut result = None;

    // Each coefficient i should be multiplied by [s^i] from the SRS
    for (i, coeff) in coeffs.iter().enumerate() {
        if let SrsPoint::G1(point) = &srs[i] {
            let scaled_point = point.scale(coeff);
            result = match result {
                None => Some(scaled_point),
                Some(prev) => Some(prev.add(&scaled_point)),
            };
        }
    }

    result.expect("Failed to evaluate polynomial with SRS")
}

fn main() {
    // SRS
    let srs = setup_srs();
    println!("\nStructured Reference String (SRS):");
    for (i, element) in srs.iter().enumerate() {
        println!("[{}]: {:?}", i, element);
    }

    // Pythagorean circuit
    let qL = vec![
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(1),
    ];
    let qR = vec![
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(1),
    ];
    let qO = vec![
        BigInt::from(-1),
        BigInt::from(-1),
        BigInt::from(-1),
        BigInt::from(-1),
    ];
    let qM = vec![
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(0),
    ];
    let qC = vec![
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(0),
    ];
    let a = vec![
        BigInt::from(3),
        BigInt::from(4),
        BigInt::from(5),
        BigInt::from(9),
    ];
    let b = vec![
        BigInt::from(3),
        BigInt::from(4),
        BigInt::from(5),
        BigInt::from(16),
    ];
    let c = vec![
        BigInt::from(9),
        BigInt::from(16),
        BigInt::from(25),
        BigInt::from(25),
    ];

    println!("\na: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("qL: {:?}", qL);
    println!("qR: {:?}", qR);
    println!("qO: {:?}", qO);
    println!("qM: {:?}", qM);
    println!("qC: {:?}", qC);

    // Roots of Unity
    // We happen to know that the 4th roots of unity in F_{17} are 1, 4, 13, 16.
    let H = vec![
        BigInt::from(1),
        BigInt::from(4),
        BigInt::from(16),
        BigInt::from(13),
    ];

    // Define cosets using K_1 = 2, and K_2 = 3
    let k1 = BigInt::from(2);
    let k2 = BigInt::from(3);
    let modulus = BigInt::from(17);

    let K1H = H
        .iter()
        .map(|h| mod_prime(&(h * &k1), &modulus))
        .collect::<Vec<BigInt>>();
    let K2H = H
        .iter()
        .map(|h| mod_prime(&(h * &k2), &modulus))
        .collect::<Vec<BigInt>>();

    println!("\nH: {:?}", H);
    println!("K1H: {:?}", K1H);
    println!("K2H: {:?}", K2H);

    // Interpolating Using the Roots of Unity
    // The interpolated polynomial will be a polynomial of degree 3
    // will have the form f_a(x) = d + cx + bx^2 + ax^3
    // We want f_a(1) = 3, f_a(4) = 4, f_a(16) = 5, f_a(13) = 9
    // Compute interpolation matrix

    // Compute polynomial coefficients for each vector
    let coeffs_a = compute_polynomial_coefficients(&a, &H, &modulus);
    let coeffs_b = compute_polynomial_coefficients(&b, &H, &modulus);
    let coeffs_c = compute_polynomial_coefficients(&c, &H, &modulus);
    let coeffs_qL = compute_polynomial_coefficients(&qL, &H, &modulus);
    let coeffs_qR = compute_polynomial_coefficients(&qR, &H, &modulus);
    let coeffs_qO = compute_polynomial_coefficients(&qO, &H, &modulus);
    let coeffs_qM = compute_polynomial_coefficients(&qM, &H, &modulus);
    let coeffs_qC = compute_polynomial_coefficients(&qC, &H, &modulus);

    // Print results
    println!("\nPolynomial Coefficients [d, c, b, a] where f(x) = ax³ + bx² + cx + d:");
    println!("a: {:?}", coeffs_a);
    println!("b: {:?}", coeffs_b);
    println!("c: {:?}", coeffs_c);
    println!("qL: {:?}", coeffs_qL);
    println!("qR: {:?}", coeffs_qR);
    println!("qO: {:?}", coeffs_qO);
    println!("qM: {:?}", coeffs_qM);
    println!("qC: {:?}", coeffs_qC);

    // Compute copy constraints
    let (sigma1, sigma2, sigma3) = compute_copy_constraints(&H, &K1H, &K2H, &modulus);

    println!("\nCopy Constraint Vectors:");
    println!("σ₁: {:?}", sigma1);
    println!("σ₂: {:?}", sigma2);
    println!("σ₃: {:?}", sigma3);

    // Interpolate the sigma vectors into polynomials
    let coeffs_sigma1 = compute_polynomial_coefficients(&sigma1, &H, &modulus);
    let coeffs_sigma2 = compute_polynomial_coefficients(&sigma2, &H, &modulus);
    let coeffs_sigma3 = compute_polynomial_coefficients(&sigma3, &H, &modulus);

    println!("\nCopy Constraint Polynomials [d, c, b, a] where f(x) = ax³ + bx² + cx + d:");
    println!("Sσ₁: {:?}", coeffs_sigma1);
    println!("Sσ₂: {:?}", coeffs_sigma2);
    println!("Sσ₃: {:?}", coeffs_sigma3);

    // Round 1 of the Prover Protocol
    println!("\nRound 1 of the Prover Protocol:");

    // Compute ZH polynomial coefficients
    let zh_coeffs = compute_zh_coefficients(&modulus);
    println!("ZH coefficients: {:?}", zh_coeffs);

    // Compute the round 1 polynomials
    let (a_x, b_x, c_x) =
        compute_round1_polynomials(&coeffs_a, &coeffs_b, &coeffs_c, &zh_coeffs, &modulus);

    println!("a(x) coefficients: {:?}", a_x);
    println!("b(x) coefficients: {:?}", b_x);
    println!("c(x) coefficients: {:?}", c_x);

    // Compute [a(s)], [b(s)], [c(s)] using the SRS
    let a_s = evaluate_polynomial_with_srs(&a_x, &srs);
    let b_s = evaluate_polynomial_with_srs(&b_x, &srs);
    let c_s = evaluate_polynomial_with_srs(&c_x, &srs);

    println!("\nRound 1 Commitments:");
    println!("[a(s)]: {:?}", a_s);
    println!("[b(s)]: {:?}", b_s);
    println!("[c(s)]: {:?}", c_s);
}
