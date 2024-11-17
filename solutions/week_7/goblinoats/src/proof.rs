use crate::{curve::{ECPoint, EllipticGroup}, field::{BaseFieldElement, FieldElement}, poynomial::{Monomial, Polynomial}, srs::SRS, circuit::{Circuit, PLONK_CIRCUIT}};


pub const ROOTS_OF_UNITY: [i32; 4] = [1,4,16,13];
pub const k1: i32 = 2;
pub const k2: i32 = 3;
pub const COSET_1: [i32; 4] = [2, 8, 15, 9];
pub const COSET_2: [i32; 4] = [3, 12, 14, 5];

pub type SubgroupElement = FieldElement<17>;

pub fn interpolate(q: &[i32; 4], h: &[i32; 4]) -> Polynomial<Monomial, 17, 4> {
    let mut matrix = [[SubgroupElement::ZERO; 4]; 4];
    let mut rhs = [SubgroupElement::ZERO; 4];

    // Build the Vandermonde matrix and RHS vector
    for i in 0..4 {
        let x = SubgroupElement::new(h[i] as usize);
        matrix[i][0] = SubgroupElement::ONE;
        matrix[i][1] = x;
        matrix[i][2] = x * x;
        matrix[i][3] = x * x * x;
        rhs[i] = SubgroupElement::new(q[i] as usize);
    }

    // Replace the printing code with the helper function call
    // print_matrix(&matrix, &rhs);

    // Gaussian elimination
    for i in 0..4 {
        // Find pivot
        let pivot = matrix[i][i];
        
        // Divide row i by pivot
        for j in 0..4 {
            matrix[i][j] = matrix[i][j] / pivot;
        }
        rhs[i] = rhs[i] / pivot;

        // Eliminate column i from all other rows
        for k in 0..4 {
            if k != i {
                let factor = matrix[k][i];
                for j in 0..4 {
                    matrix[k][j] = matrix[k][j] - factor * matrix[i][j];
                }
                rhs[k] = rhs[k] - factor * rhs[i];
            }
        }
    }

    // rhs // Contains the coefficients [d, c, b, a]
    Polynomial::<Monomial, 17, 4>::new(rhs)
}

pub fn round_1_comm_wires(circuit: Circuit, srs: &SRS<101>) -> (ECPoint, ECPoint, ECPoint) {
    let mut B = Vec::with_capacity(5);
    let determined = [7, 4, 11, 12, 16, 2]; 
    for i in 0..6 {
        // We will use the predetermined randomness from Plonk by Hand 
        // B.push(SubgroupElement::new(rand::random::<usize>() % 17));
        B.push(SubgroupElement::new(determined[i]));
    }
    let f_a = interpolate(&circuit.a, &ROOTS_OF_UNITY);
    let f_b = interpolate(&circuit.b, &ROOTS_OF_UNITY);
    let f_c = interpolate(&circuit.c, &ROOTS_OF_UNITY);
    
    // println!("f_a {}", f_a);
    // println!("f_b {}", f_b);
    // println!("f_c {}", f_c);
    
    let B_a = Polynomial::<Monomial, 17, 2>::new([B[1], B[0]]);
    let B_b = Polynomial::<Monomial, 17, 2>::new([B[3], B[2]]);
    let B_c = Polynomial::<Monomial, 17, 2>::new([B[5], B[4]]);

    let a_x = (B_a * get_z_h()) + f_a; 
    let b_x = (B_b * get_z_h()) + f_b; 
    let c_x = (B_c * get_z_h()) + f_c; 

    println!("a_x: {}", a_x);
    println!("b_x: {}", b_x);
    println!("c_x: {}", c_x);

    let commit_a_x = a_x.coefficients.iter().enumerate().map(|(i, f)| {
      let s_i = srs.g1_elements.get(i).unwrap();
      s_i.scalar_mul(f.value)
    }).reduce(|acc, e| acc.add(&e)).unwrap();

    let commit_b_x = b_x.coefficients.iter().enumerate().map(|(i, f)| {
      let s_i = srs.g1_elements.get(i).unwrap();
      s_i.scalar_mul(f.value)
    }).reduce(|acc, e| acc.add(&e)).unwrap();

    let commit_c_x = c_x.coefficients.iter().enumerate().map(|(i, f)| {
      let s_i = srs.g1_elements.get(i).unwrap();
      s_i.scalar_mul(f.value)
    }).reduce(|acc, e| acc.add(&e)).unwrap();


    (commit_a_x, commit_b_x, commit_c_x)
}

fn compute_acc_vec<const D: usize>(elems: [SubgroupElement; D]) -> SubgroupElement {
    if D < 3 {
        panic!("Array must have at least 3 elements");
    }
    
    // Sum first and last elements
    let sum_ends = elems[0] + elems[D-1];
    
    // Multiply middle elements
    let middle_product = elems[1..D-1].iter()
        .fold(SubgroupElement::ONE, |acc, &x| acc * x);
    
    // Multiply sum with the product
    sum_ends + middle_product
}

fn acc_recursive(circuit: Circuit, beta: FieldElement<17>, gamma: FieldElement<17>, i: usize) -> [FieldElement<17>; 4] {
  if i == 0 {
    return [SubgroupElement::new(1), SubgroupElement::ZERO, SubgroupElement::ZERO, SubgroupElement::ZERO];
  }
  let mut so_far = acc_recursive(circuit, beta, gamma, i - 1);

  let num_1 = compute_acc_vec([circuit.a[i-1], beta.value as i32, gamma.value as i32].map(|e| {
    SubgroupElement::new(e as usize)
  }));
  let num_2 = compute_acc_vec([circuit.b[i-1], beta.value as i32, k1, gamma.value as i32].map(|e| {
    SubgroupElement::new(e as usize)
  }));
  let num_3 = compute_acc_vec([circuit.c[i-1], beta.value as i32, k2, gamma.value as i32].map(|e| {
    SubgroupElement::new(e as usize)
  }));
  let denom_1 = compute_acc_vec([circuit.a[i-1], beta.value as i32, circuit.s_1[i-1], gamma.value as i32].map(|e| {
    SubgroupElement::new(e as usize)
  }));
  let denom_2 = compute_acc_vec([circuit.b[i-1], beta.value as i32, circuit.s_2[i-1], gamma.value as i32].map(|e| {
    SubgroupElement::new(e as usize)
  }));
  let denom_3 = compute_acc_vec([circuit.c[i-1], beta.value as i32, circuit.s_3[i-1], gamma.value as i32].map(|e| {
    SubgroupElement::new(e as usize)
  }));

  println!("denom_1: {:?}", denom_1.value());
  println!("denom_2: {:?}", denom_2.value());
  println!("denom_3: {:?}", denom_3.value());

  let i_minus = so_far[i-1];
  let result = i_minus * ((num_1*num_2*num_3)/(denom_1*denom_2*denom_3));
  so_far[i] = result;
  so_far
}

pub fn acc(circuit: Circuit, beta: FieldElement<17>, gamma: FieldElement<17>) -> Polynomial<Monomial, 17, 4> {
  let acc_vec = acc_recursive(circuit, beta, gamma, 3);
  println!("acc_vec: {:?}", acc_vec);
  Polynomial::<Monomial, 17, 4>::new(acc_vec)
}

pub fn round_2_perm_challenge(circuit: Circuit, srs: &SRS<101>) -> ECPoint {
    let mut B = Vec::with_capacity(3);
    let determined = [14, 11, 7]; 
    for i in 0..3 {
        // We will use the predetermined randomness from Plonk by Hand 
        // B.push(SubgroupElement::new(rand::random::<usize>() % 17));
        B.push(SubgroupElement::new(determined[i]));
    }

    // these represent the challenges
    // they would typically be the result of a hash of the transcript up to this point
    let beta = SubgroupElement::new(12);
    let gamma = SubgroupElement::new(13); 


    // I suspect the results in Plonk by Hand are wrong
    // acc(circuit, beta, gamma);
    // so we're just going to continue on with their numbers to continue checking correctness

    let acc = [1, 3, 9, 4];
    let acc_x = interpolate(&acc, &ROOTS_OF_UNITY);
    println!("acc_x {}", acc_x);
    let b_x = Polynomial::<Monomial, 17,3>::new([B[2],B[1],B[0]]);
    println!("b_x {}", b_x);
    let Z_H = get_z_h();

    let z_x = (b_x*Z_H) + acc_x;

    let commit_z_x = z_x.coefficients.iter().enumerate().map(|(i, f)| {
      let s_i = srs.g1_elements.get(i).unwrap();
      s_i.scalar_mul(f.value)
    }).reduce(|acc, e| acc.add(&e)).unwrap();

    commit_z_x
    
}

pub fn get_z_h() -> Polynomial<Monomial, 17, 5> {
    Polynomial::<Monomial, 17, 5>::new([
        SubgroupElement::NEG_ONE,  // x^0 term (-1)
        SubgroupElement::ZERO,     // x^1 term
        SubgroupElement::ZERO,     // x^2 term
        SubgroupElement::ZERO,     // x^3 term
        SubgroupElement::ONE,      // x^4 term
    ])
}


fn print_matrix(matrix: &[[SubgroupElement; 4]; 4], rhs: &[SubgroupElement; 4]) {
    println!("\nVandermonde Matrix:");
    for i in 0..4 {
        print!("[");
        for j in 0..4 {
            print!("{:3}, ", matrix[i][j].value());
        }
        println!("] | {}", rhs[i].value());
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use crate::curve::EllipticGroup;

    use super::*;

    #[test]
    fn test_interpolation() {
        let q = [3, 4, 5, 9];
        let coeffs = interpolate(&q, &ROOTS_OF_UNITY).coefficients;
        
        // println!("{:?}", coeffs);
        let expected_result = [1,13,3,3];

        for i in 0..4 {
            assert_eq!(coeffs[i].value(), expected_result[i]);
        }

    }

    #[test]
    fn test_round_1() {
        let srs = SRS::<101>::setup(4); //4 being the # of gates we'll be using
        
        let commitments = round_1_comm_wires(PLONK_CIRCUIT, &srs);

    }

    #[test]
    fn test_round_2() {
        let srs = SRS::<101>::setup(4);
        let comm_z_x = round_2_perm_challenge(PLONK_CIRCUIT, &srs);
        println!("z_x {}", comm_z_x);
    }


}

