use super::field::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div};
use std::array;

//some code borrowed from
//https://github.com/pluto/ronkathon/blob/main/src/algebra/field/extension/mod.rs

/// A struct that represents an element of an extension field. The element is represented as
/// [`Monomial`] coefficients of a [`Polynomial`] of degree `N - 1` over the base [`FiniteField`]
/// `F`.
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub struct GaloisField<const N: usize, const P: usize> {
  pub coeffs: [FieldElement<P>; N],
}

pub type BaseFieldExtension = GaloisField<2, 101>;

impl<const N: usize, const P: usize> GaloisField<N, P> {
    pub const ONE: Self = {
        let mut coeffs = [FieldElement::<P>::ZERO; N];
        coeffs[0] = FieldElement::<P>::ONE;
        Self { coeffs }
    };
    pub const ZERO: Self = Self { coeffs: [FieldElement::<P>::ZERO; N] };
    /// Create a new extension field element from the given coefficients of the field in polynomial
    /// form. The coefficients are expected to be from [`FiniteField`] you are extending over in the
    /// order of increasing degree. For example, for a quadratic (`N=2`) extension field, the
    /// coefficients are `[a, b]` where `a + b * t`.
    pub fn new(coeffs: [FieldElement<P>; N]) -> Self { Self { coeffs } }
    pub fn inverse(&self) -> Option<Self> {
      if *self == Self::ZERO {
        return None;
      }

      let mut res = Self::default();
      let norm = self.coeffs[0].pow(2) + FieldElement::<P>::new(2u32 as usize) * self.coeffs[1].pow(2);
      let scalar = match norm.inverse() {
          Some(inv) => inv,
          None => return None,
      };
      
      res.coeffs[0] = self.coeffs[0] * scalar;
      res.coeffs[1] = -self.coeffs[1] * scalar;
      Some(res)
    }

    pub fn pow(self, power: usize) -> Self {
      if power == 0 {
        Self::ONE
      } else if power == 1 {
        self
      } else if power % 2 == 0 {
        self.pow(power / 2) * self.pow(power / 2)
      } else {
        self.pow(power / 2) * self.pow(power / 2) * self
      }
  }
}


/// Convert from a [`FiniteField`] element into the [`GaloisField`] field element in the natural
/// way.
impl<const N: usize, const P: usize> From<FieldElement<P>> for GaloisField<N, P> {
  fn from(value: FieldElement<P>) -> Self {
    let mut coeffs = [FieldElement::<P>::ZERO; N];
    coeffs[0] = value;
    Self { coeffs }
  }
}


/// Addition of two [`GaloisField`] elements.
impl<const N: usize, const P: usize> Add for GaloisField<N, P> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let mut coeffs = self.coeffs;
    for (r, rhs_val) in coeffs.iter_mut().zip(rhs.coeffs) {
      *r += rhs_val;
    }
    Self { coeffs }
  }
}

/// Addition assignment of two [`GaloisField`] elements.
impl<const N: usize, const P: usize> AddAssign for GaloisField<N, P> {
  fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

/// Negation of a [`GaloisField`] element.
impl<const N: usize, const P: usize> std::ops::Neg for GaloisField<N, P> {
    type Output = Self;

    fn neg(self) -> Self {
      let zero = Self::from(FieldElement::<P>::ZERO);
      Self { coeffs: array::from_fn(|i| zero.coeffs[i] - self.coeffs[i]) }
    }
}

/// Subtraction of two [`GaloisField`] elements.
impl<const N: usize, const P: usize> Sub for GaloisField<N, P> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self { self + (-rhs) }
}

/// Subtraction assignment of two [`GaloisField`] elements.
impl<const N: usize, const P: usize> SubAssign for GaloisField<N, P> {
  fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}


/// ## Inclusion of [`FiniteField`] operations

/// Addition of a [`FiniteField`] element to an [`GaloisField`] element.
impl<const N: usize, const P: usize> Add<FieldElement<P>> for GaloisField<N, P> {
  type Output = Self;

  fn add(mut self, rhs: FieldElement<P>) -> Self::Output {
    self.coeffs[0] += rhs;
    self
  }
}

/// Addition assignment of a [`FiniteField`] element to an [`GaloisField`] element.
impl<const N: usize, const P: usize> AddAssign<FieldElement<P>> for GaloisField<N, P> {
  fn add_assign(&mut self, rhs: FieldElement<P>) { *self = *self + rhs; }
}

/// Subtraction of a [`FiniteField`] element from an [`GaloisField`] element.
impl<const N: usize, const P: usize> Sub<FieldElement<P>> for GaloisField<N, P> {
  type Output = Self;

  fn sub(mut self, rhs: FieldElement<P>) -> Self::Output {
    self.coeffs[0] -= rhs;
    self
  }
}

/// Subtraction assignment of a [`FiniteField`] element from an [`GaloisField`] element.
impl<const N: usize, const P: usize> SubAssign<FieldElement<P>> for GaloisField<N, P> {
  fn sub_assign(&mut self, rhs: FieldElement<P>) { *self = *self - rhs; }
}

/// Multiplication of an [`GaloisField`] element by a [`FiniteField`] element.
impl<const N: usize, const P: usize> Mul<FieldElement<P>> for GaloisField<N, P> {
  type Output = Self;

  fn mul(mut self, rhs: FieldElement<P>) -> Self::Output {
    self.coeffs.iter_mut().for_each(|c| *c = *c * rhs);
    self
  }
}

/// Multiplication assignment of an [`GaloisField`] element by a [`FiniteField`] element.
impl<const N: usize, const P: usize> MulAssign<FieldElement<P>> for GaloisField<N, P> {
  fn mul_assign(&mut self, rhs: FieldElement<P>) { *self = *self * rhs; }
}

impl<const N: usize, const P: usize> Mul<GaloisField<N, P>> for FieldElement<P> {
  type Output = GaloisField<N, P>;

  fn mul(self, rhs: GaloisField<N, P>) -> Self::Output {
    let mut res = rhs;
    res *= self;
    res
  }
}

impl<const N: usize, const P: usize> Div for GaloisField<N,P> {
  type Output = Self;

  #[allow(clippy::suspicious_arithmetic_impl)]
  fn div(self, rhs: Self) -> Self::Output { self * rhs.inverse().expect("invalid inverse") }
}

impl<const N: usize, const P: usize> Default for GaloisField<N, P> {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Multiplication of two GaloisField elements
impl<const N: usize, const P: usize> Mul for GaloisField<N, P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if N > 2 {
            panic!("Multiplication is only implemented for quadratic extensions (N=2)");
        }

        let mut res = Self::default();
        // For quadratic extension (N=2), implement multiplication modulo x^2 + 2
        // this will only work for N = 2 yeah
        res.coeffs[0] = self.coeffs[0] * rhs.coeffs[0] - FieldElement::<P>::new(2) * self.coeffs[1] * rhs.coeffs[1];
        res.coeffs[1] = self.coeffs[0] * rhs.coeffs[1] + self.coeffs[1] * rhs.coeffs[0];
        res
    }
}

/// Multiplication assignment of two GaloisField elements
impl<const N: usize, const P: usize> MulAssign for GaloisField<N, P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}