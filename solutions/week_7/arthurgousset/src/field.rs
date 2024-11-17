use crate::{mod_prime, PRIME};
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct FieldElement {
    pub a: BigInt,
    pub b: BigInt,
}

impl FieldElement {
    pub fn new(a: BigInt, b: BigInt) -> Self {
        FieldElement {
            a: mod_prime(&a, &BigInt::from(PRIME)),
            b: mod_prime(&b, &BigInt::from(PRIME)),
        }
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new(&self.a + &other.a, &self.b + &other.b)
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new(&self.a - &other.a, &self.b - &other.b)
    }

    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        let a = &self.a * &other.a - &self.b * &other.b * 2; // since u^2 = -2
        let b = &self.a * &other.b + &self.b * &other.a;
        FieldElement::new(a, b)
    }

    pub fn pow(&self, exp: &BigInt) -> FieldElement {
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

pub fn mod_inverse_field(fe: &FieldElement) -> FieldElement {
    let exp = BigInt::from(PRIME * PRIME - 2);
    fe.pow(&exp)
}
