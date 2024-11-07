use super::field::*;
use super::extension::*;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct ECPoint {
    pub x: BaseFieldElement,
    pub y: BaseFieldElement,
    pub infinity: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct ECPointExtended {
    pub x: BaseFieldExtension,
    pub y: BaseFieldExtension,
    pub infinity: bool
}

impl ECPoint {
    // Point doubling
    pub fn double(&self) -> Self {
        // Handle point at infinity
        if self.infinity {
            return ECPoint { x: BaseFieldElement::new(0), y: BaseFieldElement::new(0), infinity: true };
        }
        // Handle point with y=0 (doubling gives point at infinity)
        if self.y == BaseFieldElement::new(0) {
            return ECPoint { x: BaseFieldElement::new(0), y: BaseFieldElement::new(0), infinity: true };
        }

        let two: BaseFieldElement = BaseFieldElement::new(2);
        let three: BaseFieldElement = BaseFieldElement::new(3);

        // Slope = (3x^2) / (2y)
        let numerator = three * self.x.pow(2);
        let denominator = two * self.y;
        let slope = numerator / denominator;

        // x_r = slope^2 - 2x
        let x_r = slope.pow(2) - (two * self.x);

        // y_r = slope * (3x - m^2) - y
        let y_r = slope * (three*self.x - slope.pow(2)) - self.y;

        ECPoint { x: x_r, y: y_r, infinity: false }
    }

    pub fn add(&self, other: &Self) -> Self {
        // Handle points at infinity
        if self.infinity {
            return *other;
        }
        if other.infinity {
            return *self;
        }

        // Handle special cases
        if self.x == other.x {
            if self.y == other.y {
                return self.double();
            } else {
                return ECPoint { x: BaseFieldElement::new(0), y: BaseFieldElement::new(0), infinity: true };
            }
        }

        // Calculate slope = (y2-y1)/(x2-x1)
        let slope = (other.y - self.y) / (other.x - self.x);

        // x3 = slope^2 - x1 - x2
        let x3 = slope.pow(2) - self.x - other.x;

        // y3 = slope(x1 - x3) - y1
        let y3 = slope * (self.x - x3) - self.y;

        ECPoint { x: x3, y: y3, infinity: false }
    }

    pub fn invert(&self) -> Self {
        ECPoint { x: self.x, y: BaseFieldElement::new(0) - self.y, infinity: false }
    }
}

impl Mul<usize> for ECPoint {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        let mut result = ECPoint { x: BaseFieldElement::new(0), y: BaseFieldElement::new(0), infinity: true };
        let mut temp = self.clone();
        let mut n = scalar;

        while n > 0 {
            if n & 1 == 1 {
                result = result.add(&temp);
            }
            temp = temp.double();
            n >>= 1;
        }

        result
    }
}


impl ECPointExtended {
    pub fn double(&self) -> Self {
        // Handle point at infinity
        if self.infinity {
            return ECPointExtended { x: BaseFieldExtension::ZERO, y: BaseFieldExtension::ZERO, infinity: true };
        }
        // Handle point with y=0 (doubling gives point at infinity)
        if self.y == BaseFieldExtension::ZERO {
            return ECPointExtended { x: BaseFieldExtension::ZERO, y: BaseFieldExtension::ZERO, infinity: true };
        }

        let two = BaseFieldExtension::new([BaseFieldElement::new(2), BaseFieldElement::new(0)]);
        let three = BaseFieldExtension::new([BaseFieldElement::new(3), BaseFieldElement::new(0)]);

        // Slope = (3x^2) / (2y)
        let numerator = three * self.x.pow(2);
        let denominator = two * self.y;
        let slope = numerator / denominator;

        // x_r = slope^2 - 2x
        let x_r = slope.pow(2) - (two * self.x);

        // y_r = slope * (3x - m^2) - y
        let y_r = slope * (three*self.x - slope.pow(2)) - self.y;

        ECPointExtended { x: x_r, y: y_r, infinity: false }
    }

    pub fn add(&self, other: &Self) -> Self {
        // Handle points at infinity
        if self.infinity {
            return *other;
        }
        if other.infinity {
            return *self;
        }

        // Handle special cases
        if self.x == other.x {
            if self.y == other.y {
                return self.double();
            } else {
                return ECPointExtended { x: BaseFieldExtension::ZERO, y: BaseFieldExtension::ZERO, infinity: true };
            }
        }

        // Calculate slope = (y2-y1)/(x2-x1)
        let slope = (other.y - self.y) / (other.x - self.x);

        // x3 = slope^2 - x1 - x2
        let x3 = slope.pow(2) - self.x - other.x;

        // y3 = slope(x1 - x3) - y1
        let y3 = slope * (self.x - x3) - self.y;

        ECPointExtended { x: x3, y: y3, infinity: false }
    }

    pub fn invert(&self) -> Self {
        ECPointExtended { x: self.x, y: BaseFieldExtension::ZERO - self.y, infinity: false }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generatorTest() {
        let G1 = ECPoint { x: BaseFieldElement::new(1), y: BaseFieldElement::new(2), infinity: false };
        let mut last_double = G1;
        let mut odd_double = G1.add(&G1.double());
        for _ in 0..4 {
            println!("{:?}", last_double);
            println!("{:?}", last_double.invert());
            println!("{:?}", odd_double);
            println!("{:?}", odd_double.invert());
            last_double = last_double.double();
            odd_double = odd_double.double();
        }
    }
}