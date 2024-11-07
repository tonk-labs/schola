use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

// Define the finite field F_p where p = 101
const P: i64 = 101;

// Elliptic curve parameters for y^2 = x^3 + ax + b over F_p
const A: i64 = 0;
const B: i64 = 3;

// Generator point G1 = (1, 2)
const G1_X: i64 = 1;
const G1_Y: i64 = 2;

// Extension field F_p^2 where u^2 = -2
const U_SQUARE: i64 = -2;

// Define a point on the elliptic curve
#[derive(Debug, Clone)]
struct ECPoint {
    x: FieldElement,
    y: FieldElement,
}

impl ECPoint {
    // Point doubling
    fn double(&self) -> Self {
        let two = FieldElement::new(2);
        let three = FieldElement::new(3);

        // Slope = (3x^2 + a) / (2y)
        let numerator = three * &self.x.square();
        let denominator = two * &self.y;
        let slope = numerator / denominator;

        // x_r = slope^2 - 2x
        let x_r = &slope.square() - &(two * &self.x);

        // y_r = slope * (x - x_r) - y
        let y_r = &slope * (&self.x - &x_r) - &self.y;

        ECPoint { x: x_r, y: y_r }
    }

    // Point addition
    fn add(&self, other: &Self) -> Self {
        if self.x == other.x && self.y == other.y {
            return self.double();
        }

        let slope = (&other.y - &self.y) / (&other.x - &self.x);
        let x_r = &slope.square() - &self.x - &other.x;
        let y_r = &slope * (&self.x - &x_r) - &self.y;

        ECPoint { x: x_r, y: y_r }
    }

    // Scalar multiplication
    fn mul(&self, scalar: i64) -> Self {
        let mut result = ECPoint::infinity();
        let mut addend = self.clone();
        let mut k = scalar;

        while k > 0 {
            if k % 2 == 1 {
                result = result.add(&addend);
            }
            addend = addend.double();
            k /= 2;
        }

        result
    }

    // Point at infinity (identity element)
    fn infinity() -> Self {
        ECPoint {
            x: FieldElement::zero(),
            y: FieldElement::zero(),
        }
    }
}

// Define field element over F_p
#[derive(Debug, Clone, PartialEq)]
struct FieldElement {
    value: i64,
}

impl FieldElement {
    fn new(value: i64) -> Self {
        FieldElement {
            value: value.rem_euclid(P),
        }
    }

    fn zero() -> Self {
        FieldElement::new(0)
    }

    fn one() -> Self {
        FieldElement::new(1)
    }

    fn square(&self) -> Self {
        self * self
    }

    fn inverse(&self) -> Self {
        // Using Fermat's Little Theorem for inversion
        self.pow(P - 2)
    }

    fn pow(&self, exponent: i64) -> Self {
        let mut result = FieldElement::one();
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            exp /= 2;
        }

        result
    }
}

// Implement arithmetic operations for FieldElement
use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: &FieldElement) -> FieldElement {
        FieldElement::new(self.value + other.value)
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &FieldElement) -> FieldElement {
        FieldElement::new(self.value - other.value)
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &FieldElement) -> FieldElement {
        FieldElement::new(self.value * other.value)
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, other: &FieldElement) -> FieldElement {
        self * &other.inverse()
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> FieldElement {
        FieldElement::new(-self.value)
    }
}

// Implement multiplication with scalar
impl Mul<i64> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, scalar: i64) -> FieldElement {
        FieldElement::new(self.value * scalar)
    }
}

// Implement multiplication with FieldElement and scalar
impl Mul<&FieldElement> for i64 {
    type Output = FieldElement;

    fn mul(self, fe: &FieldElement) -> FieldElement {
        FieldElement::new(self * fe.value)
    }
}

// Trusted setup for PLONK
fn trusted_setup(s: i64, n: usize, g1: &ECPoint) -> Vec<ECPoint> {
    let mut srs = Vec::new();
    for i in 0..n {
        let exponent = s.pow(i as u32);
        let point = g1.mul(exponent);
        srs.push(point);
    }
    srs
}

fn main() {
    // Define the generator point G1
    let g1 = ECPoint {
        x: FieldElement::new(G1_X),
        y: FieldElement::new(G1_Y),
    };

    // Secret value s (randomly chosen)
    let s = 7; // For example purposes

    // Number of gates (n) + 5 as per PLONK requirements
    let n = 4 + 5;

    // Run the trusted setup to generate the SRS
    let srs = trusted_setup(s, n, &g1);

    // Print the SRS points
    println!("Structured Reference String (SRS):");
    for (i, point) in srs.iter().enumerate() {
        println!("s^{} * G1 = ({}, {})", i, point.x.value, point.y.value);
    }

    // Now, we can define the circuit constraints
    // For the Pythagorean triple (3, 4, 5)
    let a_values = vec![3, 4, 5, 9];
    let b_values = vec![3, 4, 5, 16];
    let c_values = vec![9, 16, 25, 25];

    // Verify the constraints
    for i in 0..4 {
        let a = FieldElement::new(a_values[i]);
        let b = FieldElement::new(b_values[i]);
        let c = FieldElement::new(c_values[i]);

        if i < 3 {
            // Multiplication gate: a * b = c
            let lhs = &a * &b;
            if lhs != c {
                println!(
                    "Constraint {} failed: {} * {} != {}",
                    i + 1,
                    a.value,
                    b.value,
                    c.value
                );
            } else {
                println!(
                    "Constraint {} passed: {} * {} = {}",
                    i + 1,
                    a.value,
                    b.value,
                    c.value
                );
            }
        } else {
            // Addition gate: a + b = c
            let lhs = &a + &b;
            if lhs != c {
                println!(
                    "Constraint {} failed: {} + {} != {}",
                    i + 1,
                    a.value,
                    b.value,
                    c.value
                );
            } else {
                println!(
                    "Constraint {} passed: {} + {} = {}",
                    i + 1,
                    a.value,
                    b.value,
                    c.value
                );
            }
        }
    }
}