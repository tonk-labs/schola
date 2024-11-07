use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
//using code from ronkathon here
//https://github.com/pluto/ronkathon/blob/main/src/algebra/field/prime/mod.rs

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default, PartialOrd)]
pub struct FieldElement<const P: usize> {
    pub value: usize,
  }

// Define the finite field F_p where p = 101
const ORDER: i64 = 101;
pub type BaseFieldElement = FieldElement<{ ORDER as usize }>;

// this is grossly simplified and incomplete
impl<const P: usize> FieldElement<P> {
    pub const ONE: Self = Self { value: 1 };
    pub const ZERO: Self = Self { value: 0 };

    /// Creates a new `FieldElement` with the given value and modulus.
    pub fn new(value: usize) -> Self {
        Self { value: value % P }
    }

    /// Computes the multiplicative inverse of the element.
    pub fn inverse(self) -> Option<Self> {
        if self.value == 0 {
            return None;
        }
      
        // By fermat's little theorem, in any prime field P, for any elem:
        //    e^(P-1) = 1 mod P
        // So,
        //    e^(P-2) = e^-1 mod P
        Some(self.pow(P - 2))
    }

    /// Raises the element to the power of `exponent`.
    pub fn pow(self, power: usize) -> Self {
        if power == 0 {
            Self::ONE
        } else if power == 1 {
            self
        } else if power % 2 == 0 {
            let half = self.pow(power / 2);
            Self::new(half.value * half.value)
        } else {
            let half = self.pow(power / 2);
            Self::new(half.value * half.value * self.value)
        }
    }

    /// Returns the value of the field element.
    pub fn value(&self) -> usize {
        self.value
    }
}

/// Extended Euclidean Algorithm to find the GCD and coefficients.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

/// Implement addition for FieldElement
impl<const P: usize> Add for FieldElement<P> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<const P: usize> AddAssign for FieldElement<P> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

/// Implement subtraction for FieldElement
impl<const P: usize> Sub for FieldElement<P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (mut diff, over) = self.value.overflowing_sub(other.value);
        let corr = if over { P } else { 0 };
        diff = diff.wrapping_add(corr);
        Self { value: diff }
    }
}

impl<const P: usize> SubAssign for FieldElement<P> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}


/// Implement multiplication for FieldElement
impl<const P: usize> Mul for FieldElement<P> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

/// Implement division for FieldElement
impl<const P: usize> Div for FieldElement<P> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse().unwrap()
    }
}

/// Implement negation for FieldElement
impl<const P: usize> Neg for FieldElement<P> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::ZERO - self
    }
}

