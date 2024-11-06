use num_bigint::BigInt;
use num_traits::{One, Zero};
use rand::Rng;

// Uses Plonk by Hand tutorials
// 1. https://research.metastate.dev/plonk-by-hand-part-1/
// 2. https://research.metastate.dev/plonk-by-hand-part-2-the-proof/

/// Prime modulus
const PRIME: i32 = 101;

/// Returns an integer equivalent in the finite field F_101 (i.e., between 0 and 100)
fn mod_prime(x: &BigInt, modulus: &BigInt) -> BigInt {
    let result = x % modulus;

    // Ensure the result is positive
    if result < BigInt::zero() {
        result + modulus
    } else {
        result
    }
}

/// Calculates the modular inverse of `a` mod `p` using Fermat's Little Theorem
fn mod_inverse(a: &BigInt, p: &BigInt) -> BigInt {
    let exp = p - 2;
    a.modpow(&exp, p)
}

/// Represents an element of the extension field F_{101^2}, as `a + bu`
#[derive(Debug, Clone, PartialEq)]
struct FieldElement {
    a: BigInt,
    b: BigInt,
}

impl FieldElement {
    fn new(a: BigInt, b: BigInt) -> Self {
        FieldElement {
            a: mod_prime(&a, &BigInt::from(PRIME)),
            b: mod_prime(&b, &BigInt::from(PRIME)),
        }
    }

    fn add(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new(&self.a + &other.a, &self.b + &other.b)
    }

    fn sub(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new(&self.a - &other.a, &self.b - &other.b)
    }

    fn mul(&self, other: &FieldElement) -> FieldElement {
        let a = &self.a * &other.a - &self.b * &other.b * 2; // since u^2 = -2
        let b = &self.a * &other.b + &self.b * &other.a;
        FieldElement::new(a, b)
    }

    fn pow(&self, exp: &BigInt) -> FieldElement {
        let mut result = FieldElement::new(BigInt::one(), BigInt::zero());
        let mut base = self.clone();
        let mut exp = exp.clone();
        
        while exp > BigInt::zero() {
            if &exp % 2 == BigInt::one() {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2;
        }
        result
    }
}

/// Elliptic curve point in F_{101^2} with coordinates (x, y) where x, y are FieldElements
#[derive(Debug, Clone, PartialEq)]
struct G2Point {
    x: FieldElement,
    y: FieldElement,
}

impl G2Point {
    fn new(x: FieldElement, y: FieldElement) -> Self {
        G2Point { x, y }
    }

    fn double(&self) -> Self {
        let two = FieldElement::new(BigInt::from(2), BigInt::zero());
        let three = FieldElement::new(BigInt::from(3), BigInt::zero());

        let numerator = self.x.mul(&self.x).mul(&three);
        let denominator = self.y.mul(&two);
        let denominator_inv = mod_inverse_field(&denominator);
        let m = numerator.mul(&denominator_inv);

        let new_x = m.mul(&m).sub(&self.x).sub(&self.x);
        let new_y = m.mul(&self.x.sub(&new_x)).sub(&self.y);

        G2Point { x: new_x, y: new_y }
    }

    fn add(&self, other: &G2Point) -> Self {
        if self == other {
            return self.double();
        }

        let numerator = other.y.sub(&self.y);
        let denominator = other.x.sub(&self.x);
        let denominator_inv = mod_inverse_field(&denominator);
        let m = numerator.mul(&denominator_inv);

        let new_x = m.mul(&m).sub(&self.x).sub(&other.x);
        let new_y = m.mul(&self.x.sub(&new_x)).sub(&self.y);

        G2Point { x: new_x, y: new_y }
    }

    fn scale(&self, scalar: &BigInt) -> Self {
        let mut result = self.clone();
        let one = BigInt::one();
        let mut current = one.clone();

        while &current < scalar {
            result = result.add(self);
            current += &one;
        }

        result
    }
}

/// Calculates the modular inverse in F_{101^2} using Fermat's Little Theorem
fn mod_inverse_field(fe: &FieldElement) -> FieldElement {
    let exp = BigInt::from(PRIME * PRIME - 2);
    fe.pow(&exp)
}

/// Standard Point struct in F_101
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    fn new(x: BigInt, y: BigInt) -> Self {
        Point { x, y }
    }

    fn double(&self) -> Self {
        let prime = BigInt::from(PRIME);
        let three = BigInt::from(3);
        let two = BigInt::from(2);
        let numerator = &three * &self.x * &self.x;
        let denominator = &two * &self.y;
        let denominator_inv = mod_inverse(&denominator, &prime);
        let m = mod_prime(&(&numerator * &denominator_inv), &prime);

        let new_x = mod_prime(&(&m * &m - &two * &self.x), &prime);
        let new_y = mod_prime(&(&m * (&self.x - &new_x) - &self.y), &prime);

        Point { x: new_x, y: new_y }
    }

    fn add(&self, other: &Point) -> Self {
        if self == other {
            return self.double();
        }

        let prime = BigInt::from(PRIME);
        let numerator = &other.y - &self.y;
        let denominator = &other.x - &self.x;
        let denominator_inv = mod_inverse(&denominator, &prime);

        let m = mod_prime(&(&numerator * &denominator_inv), &prime);

        let new_x = mod_prime(&(&m * &m - &self.x - &other.x), &prime);
        let new_y = mod_prime(&(&m * (&self.x - &new_x) - &self.y), &prime);

        Point { x: new_x, y: new_y }
    }

    fn scale(&self, scalar: &BigInt) -> Self {
        let mut result = self.clone();
        let one = BigInt::one();
        let mut current = one.clone();

        while &current < scalar {
            result = result.add(self);
            current += &one;
        }

        result
    }

    fn invert(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: mod_prime(&-&self.y, &BigInt::from(PRIME)),
        }
    }
}

#[derive(Debug, Clone)]
enum SrsPoint {
    G1(Point),
    G2(G2Point),
}

fn main() {

    // G1 points

    // Initialize G1 generator in F_101
    let g1 = Point::new(BigInt::from(1), BigInt::from(2));
    println!("G1: {:?}", g1);

    // Test point doubling
    let doubled = g1.double();
    println!("Doubled: {:?}", doubled);

    // Test point inversion
    let inverted = g1.invert();
    println!("Inverted: {:?}", inverted);

    // Test point addition
    let p1 = g1.clone();
    let added = p1.add(&doubled);
    println!("Added (G1 + 2G1): {:?}", added);

    // Test point scalar multiplication
    println!("Scaled G1 by 2: {:?}", g1.clone().scale(&BigInt::from(2)));
    println!("Scaled G1 by 3: {:?}", g1.clone().scale(&BigInt::from(3)));

    // Compute subgroup generated by G1
    let mut current = g1.clone();
    println!("{:?}: {:?}", 1, current);
    for i in 2..=16 {
        current = current.add(&g1);
        println!("{:?}: {:?}", i, current);
    }

    // G2 points

    // Initialize G2 generator in extension field F_{101^2}.
    // We happen to know (36, 31u) is a generator point.
    let g2 = G2Point::new(
        FieldElement::new(BigInt::from(36), BigInt::zero()),
        FieldElement::new(BigInt::zero(), BigInt::from(31)),
    );
    println!("G2: {:?}", g2);

    // Test doubled G2 point
    let doubled_g2 = g2.double();
    println!("2 * G2: {:?}", doubled_g2);

    // Test scaled G2 point
    let s = BigInt::from(2);
    let scaled_g2 = g2.scale(&s);
    println!("Scaled G2 by 2: {:?}", scaled_g2);

    // Generate random s < order of group (= 17)
    let s = BigInt::from(2); // Simplification to follow Plonk by Hand tutorial

    // Initialize structure reference string
    let mut srs: Vec<SrsPoint> = Vec::new();

    let nr_gates = 4;
    let nr_SRS_elements = nr_gates + 5;
    let subgroup_order = BigInt::from(17);

    // $1 \times G_1$
    srs.push(SrsPoint::G1(g1.clone()));

    // $S \times G_1$, $S^2 \times G_1$, $S^3 \times G_1$, $S^4 \times G_1$, $S^5 \times G_1$, $S^6 \times G_1$
    for i in 1..=nr_gates + 2 {
        let s_power = s.modpow(&BigInt::from(i), &subgroup_order);
        let g1_times_s_power = g1.clone().scale(&s_power);
        println!(
            "S to the power of {:?} is {:?}: {:?}",
            i, s_power, g1_times_s_power
        );
        srs.push(SrsPoint::G1(g1_times_s_power));
    }

    // $1 \times G_2$
    println!("1 * G2: {:?}", g2);
    srs.push(SrsPoint::G2(g2.clone()));

    // $S \times G_2$
    let s_power = s.modpow(&BigInt::from(1), &subgroup_order);
    let g2_times_s_power = g2.clone().scale(&s_power);
    println!("S * G2: {:?}", g2_times_s_power);
    srs.push(SrsPoint::G2(g2_times_s_power));

    println!("\nStructured Reference String (SRS):");
    for (i, element) in srs.iter().enumerate() {
        println!("[{}]: {:?}", i, element);
    }
}
