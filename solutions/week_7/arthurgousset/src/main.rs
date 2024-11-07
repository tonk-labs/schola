use num_bigint::BigInt;
use week_7_arthurgousset::{mod_prime, srs::setup_srs, mod_inverse};

fn compute_interpolation_matrix(domain: &[BigInt], modulus: &BigInt) -> Vec<Vec<BigInt>> {
    let n = domain.len();
    let mut matrix = vec![vec![BigInt::from(0); n]; n];

    // Build Vandermonde matrix
    // Each row i represents: [1, x_i, x_i^2, x_i^3]
    for i in 0..n {
        let x = &domain[i];
        matrix[i][0] = BigInt::from(1);  // x^0
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
        for j in 0..2*n {
            augmented[i][j] = mod_prime(&(&augmented[i][j] * &pivot_inv), modulus);
        }

        // Eliminate column i
        for k in 0..n {
            if k != i {
                let factor = augmented[k][i].clone();
                for j in 0..2*n {
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
        BigInt::from(0),
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

    println!("\nK1H: {:?}", K1H);
    println!("K2H: {:?}", K2H);

    // Interpolating Using the Roots of Unity
    // The interpolated polynomial will be a polynomial of degree 3
    // will have the form f_a(x) = d + cx + bx^2 + ax^3
    // We want f_a(1) = 3, f_a(4) = 4, f_a(16) = 5, f_a(13) = 9
    // Compute interpolation matrix

    let interpolation_matrix = compute_interpolation_matrix(&H, &modulus);
    
    println!("\nInterpolation Matrix:");
    for row in &interpolation_matrix {
        println!("{:?}", row);
    }

    // Example: interpolate polynomial for vector a
    let mut coefficients_a = vec![BigInt::from(0); 4];
    for i in 0..4 {
        for j in 0..4 {
            let term = mod_prime(&(&interpolation_matrix[i][j] * &a[j]), &modulus);
            coefficients_a[i] = mod_prime(&(&coefficients_a[i] + &term), &modulus);
        }
    }

    println!("\nCoefficients for polynomial a:");
    println!("{:?}", coefficients_a);
}
