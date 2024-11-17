use crate::{field::*, mod_inverse, mod_prime, PRIME};
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}

impl Point {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Point { x, y }
    }

    pub fn double(&self) -> Self {
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

    pub fn add(&self, other: &Point) -> Self {
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

    pub fn scale(&self, scalar: &BigInt) -> Self {
        let mut result = self.clone();
        let one = BigInt::one();
        let mut current = one.clone();

        while &current < scalar {
            result = result.add(self);
            current += &one;
        }

        result
    }

    pub fn invert(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: mod_prime(&-&self.y, &BigInt::from(PRIME)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct G2Point {
    pub x: FieldElement,
    pub y: FieldElement,
}

impl G2Point {
    pub fn new(x: FieldElement, y: FieldElement) -> Self {
        G2Point { x, y }
    }

    pub fn double(&self) -> Self {
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

    pub fn add(&self, other: &G2Point) -> Self {
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

    pub fn scale(&self, scalar: &BigInt) -> Self {
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

// ... implementations ...
