use super::*;
use crate::field::{FieldElement, primitive_root_of_unity};
use std::array;
use std::fmt;
use std::ops::{Add, Mul};

/**
 * This code is a slight modification of https://github.com/pluto/ronkathon/blob/main/src/polynomial/mod.rs
 */
pub trait Basis {
  type Data;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
 pub struct Monomial;
 impl Basis for Monomial {
  type Data = ();
 }

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lagrange<const P: usize> {
  pub nodes: Vec<FieldElement<P>>
}
impl<const P: usize> Basis for Lagrange<P> {
  type Data = Self;
}

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 pub struct Polynomial<B: Basis, const P: usize, const D: usize> {
  pub coefficients: [FieldElement<P>; D],

  pub basis: B,
 }

 impl<B: Basis, const P: usize, const D: usize> Polynomial<B, P, D> {
  /// A polynomial in any basis has a fixed number of independent terms.
  /// For example, in [`Monomial`] basis, the number of terms is one more than the degree of the
  /// polynomial.
  ///
  /// ## Arguments:
  /// - `self`: The polynomial in any basis.
  ///
  /// ## Returns:
  /// - The number of terms in the polynomial as `usize`.
  pub fn num_terms(&self) -> usize { self.coefficients.len() }
}

impl<const P: usize, const D: usize> Polynomial<Monomial, P, D> {
  /// Create a new polynomial in [`Monomial`] basis.
  ///
  /// ## Arguments:
  /// - `coefficients`: A vector of field elements representing the coefficients of the polynomial
  ///   on each monomial term, e.g., x^0, x^1, ....
  ///
  /// ## Returns:
  /// - A new polynomial in [`Monomial`] basis with the given coefficients.
  /// - The polynomial is automatically simplified to have a non-zero leading coefficient, that is
  ///   coefficient on the highest power term x^d.
  pub fn new(coefficients: [FieldElement<P>; D]) -> Self { Self { coefficients, basis: Monomial } }

  /// Helper method to remove leading zeros from coefficients
  fn trim_zeros(coefficients: &mut Vec<FieldElement<P>>) {
    while coefficients.last().cloned() == Some(FieldElement::<P>::ZERO) {
      coefficients.pop();
    }
  }

  /// Gets the degree of the polynomial in the [`Monomial`] [`Basis`].
  /// ## Arguments:
  /// - `self`: The polynomial in the [`Monomial`] [`Basis`].
  ///
  /// ## Returns:
  /// - The degree of the polynomial as a `usize`.
  pub fn degree(&self) -> usize {
    self.coefficients.iter().rposition(|&coeff| coeff != FieldElement::<P>::ZERO).unwrap_or(0)
  }

  /// Retrieves the coefficient on the highest degree monomial term of a polynomial in the
  /// [`Monomial`] [`Basis`].
  // pub const fn leading_coefficient(&self) -> F { *self.coefficients.last().unwrap() }
  pub fn leading_coefficient(&self) -> FieldElement<P> {
    self.coefficients.iter().rev().find(|&&coeff| coeff != FieldElement::<P>::ZERO).copied().unwrap_or(FieldElement::<P>::ZERO)
  }

  /// Evaluates the polynomial at a given [`FiniteField`] element `x` using the [`Monomial`]
  /// basis. This is not using Horner's method or any other optimization.
  ///
  /// ## Arguments:
  /// - `x`: The field element at which to evaluate the polynomial.
  ///
  /// ## Returns:
  /// - The result of evaluating the polynomial at `x` which is an element of the associated
  ///   [`FiniteFiniteField`].
  pub fn evaluate(&self, x: FieldElement<P>) -> FieldElement<P> {
    let mut result = FieldElement::<P>::ZERO;
    for (i, c) in self.coefficients.iter().enumerate() {
      result += *c * x.pow(i);
    }
    result
  }

  /// Multiplies a polynomial by a scalar `coeff` and shifts its terms by adding `D2` to each exponent.
  /// This effectively multiplies the polynomial by `coeff * x^D2`.
  ///
  /// ## Arguments:
  /// - `coeff`: The scalar coefficient to multiply the polynomial by
  /// - The generic parameter `D2` determines how many positions to shift the terms
  ///
  /// ## Returns:
  /// - A new polynomial where each term x^i has been transformed to coeff * x^(i+D2)
  pub fn pow_mult<const D2: usize>(&self, coeff: FieldElement<P>) -> Polynomial<Monomial, P, D> {
    // Create a vector to store the shifted coefficients
    let mut coefficients = vec![FieldElement::<P>::ZERO; D];
    
    // For each coefficient, multiply by coeff and shift its position by D2
    for i in 0..self.coefficients.len() {
        if i + D2 < D {
            coefficients[i + D2] = self.coefficients[i] * coeff;
        }
    }
    
    // Convert to fixed-size array
    Polynomial::<Monomial, P, D>::new(coefficients.try_into().unwrap())
  }

  /// [Euclidean division](https://en.wikipedia.org/wiki/Euclidean_division) of two polynomials in [`Monomial`] basis.
  /// Used explicitly in implementing the [`Div`] and [`Rem`] traits.
  ///
  /// ## Arguments:
  /// - `self`: The dividend polynomial in [`Monomial`] basis.
  /// - `rhs`: The divisor polynomial in [`Monomial`] basis.
  ///
  /// ## Returns:
  /// - A tuple of two polynomials in [`Monomial`] basis:
  ///   - The first element is the quotient polynomial.
  ///   - The second element is the remainder polynomial.
  fn quotient_and_remainder<const D2: usize>(
    self,
    rhs: Polynomial<Monomial, P, D2>,
  ) -> (Self, Self) {
    // Initial quotient value
    let mut q_coeffs = vec![FieldElement::<P>::ZERO; D];

    // Initial remainder value is our numerator polynomial
    let mut p_coeffs = self.coefficients.to_vec();

    // Leading coefficient of the denominator
    let c = rhs.leading_coefficient();

    // Perform the repeated long division algorithm
    while p_coeffs.iter().filter(|&&x| x != FieldElement::<P>::ZERO).count() > 0
      && p_coeffs.len() >= rhs.coefficients.len()
    {
      // find degree of dividend, divisor
      let p_degree = p_coeffs.iter().rposition(|&x| x != FieldElement::<P>::ZERO).unwrap();
      let rhs_degree = rhs.coefficients.iter().rposition(|&x| x != FieldElement::<P>::ZERO).unwrap();

      if p_degree < rhs_degree {
        break;
      }

      let diff = p_degree - rhs_degree;
      let s = p_coeffs[p_degree] * c.inverse().unwrap();
      q_coeffs[diff] = s;

      for (i, &coeff) in rhs.coefficients.iter().enumerate() {
        p_coeffs[diff + i] -= coeff * s;
      }

      Polynomial::<Monomial, P, D>::trim_zeros(&mut p_coeffs);
    }

    let quotient = Polynomial {
      coefficients: q_coeffs.try_into().unwrap_or_else(|v: Vec<FieldElement<P>>| {
        let mut arr = [FieldElement::<P>::ZERO; D];
        arr.copy_from_slice(&v[..D]);
        arr
      }),
      basis:        self.basis,
    };

    let remainder = Polynomial {
      coefficients: p_coeffs.try_into().unwrap_or_else(|v: Vec<FieldElement<P>>| {
        let mut arr = [FieldElement::<P>::ZERO; D];
        arr[..v.len()].copy_from_slice(&v[..]);
        arr
      }),
      basis:        self.basis,
    };

    (quotient, remainder)
  }

  /// Computes the [Discrete Fourier Transform](https://en.wikipedia.org/wiki/Discrete_Fourier_transform)
  /// of the polynomial in the [`Monomial`] basis by evaluating the polynomial at the roots of
  /// unity.
  /// This also converts a polynomial from [`Monomial`] to [`Lagrange`] [`Basis`] with node points
  /// given by the roots of unity.
  ///
  /// ## Returns:
  /// - A new polynomial in the [`Lagrange`] [`Basis`] that is the result of converting the
  ///   evaluation of the polynomial at the roots of unity.
  ///
  /// ## Panics
  /// - This function will panic in calling [`FiniteField::primitive_root_of_unity`] if the field
  ///   does not have roots of unity for the degree of the polynomial.
  pub fn dft(&self) -> Polynomial<Lagrange<P>, P, D> {
    let n = self.num_terms();
    let primitive_root_of_unity = primitive_root_of_unity(n);

    let coeffs: Vec<FieldElement<P>> = (0..n)
      .map(|i| {
        self
          .coefficients
          .iter()
          .enumerate()
          .fold(FieldElement::<P>::ZERO, |acc, (j, &coeff)| acc + coeff * primitive_root_of_unity.pow(i * j))
      })
      .collect();
    Polynomial::<Lagrange<P>, P, D>::new(
      coeffs.try_into().unwrap_or_else(|v: Vec<FieldElement<P>>| {
        panic!("Expected a Vec of length {} but it was {}", D, v.len())
      }),
    )
  }
}

impl<const P: usize, const D: usize> fmt::Display for Polynomial<Monomial, P, D> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut first = true;
    for (i, c) in self.coefficients.iter().enumerate() {
      if !first {
        write!(f, " + ")?;
      }
      first = false;
      if i == 0 {
        write!(f, "{}", c)?;
      } else {
        write!(f, "{}x^{}", c, i)?;
      }
    }
    Ok(())
  }
}


impl<const P: usize, const D: usize> Polynomial<Lagrange<P>, P, D> {
  /// Create a new polynomial in [`Lagrange`] basis by supplying a number of coefficients.
  /// Assumes that a field has a root of unity for the amount of terms given in the coefficients.
  ///
  /// ## Arguments:
  /// - `coefficients`: A vector of field elements representing the coefficients of the polynomial
  ///   in the [`Lagrange`] basis.
  ///
  /// ## Returns:
  /// - A new polynomial in the [`Lagrange`] basis with the given coefficients.
  ///
  /// ## Panics
  /// - This function will panic if the field does not have roots of unity for the length of the
  ///   polynomial.
  pub fn new(coefficients: [FieldElement<P>; D]) -> Self {
    // Check that the polynomial degree divides the field order so that there are roots of unity.
    let n = coefficients.len();
    assert_eq!((FieldElement::<P>::ORDER - 1) % n, 0);
    let primitive_root = primitive_root_of_unity(n);
    let nodes: Vec<FieldElement<P>> = (0..n).map(|i| primitive_root.pow(i)).collect();

    Self { coefficients, basis: Lagrange { nodes } }
  }

  /// Evaluate the polynomial in the [`Lagrange`] basis at a given field element `x`.
  /// This is done by evaluating the Lagrange polynomial at `x` using the nodes of the Lagrange
  /// basis. The Lagrange polynomial is given by:
  /// $$
  /// L(x) = \sum_{j=0}^{n-1} \left( \frac{w_j}{x - x_j} \right) y_j
  /// $$
  /// where $w_j = \prod_{m \neq j} (x_j - x_m)^{-1}$ and $y_j$ are the coefficients of the
  /// polynomial. The evaluation of the polynomial at `x` is then given by $L(x)$.
  ///
  /// ## Arguments:
  /// - `x`: The field element as [`FiniteField`] at which to evaluate the polynomial.
  ///
  /// ## Returns:
  /// - The result of evaluating the polynomial at `x` which is an element of the associated
  ///   [`FiniteField`].
  pub fn evaluate(&self, x: FieldElement<P>) -> FieldElement<P> {
    let n = self.coefficients.len();

    // w_j = \Pi_{m \neq j} (x_j - x_m)^{-1}
    let mut weights = vec![FieldElement::<P>::ONE; n];
    weights.iter_mut().enumerate().for_each(|(idx, w)| {
      for m in 0..n {
        if idx != m {
          *w = *w * FieldElement::<P>::ONE.div(self.basis.nodes[idx] - self.basis.nodes[m]).unwrap();
        }
      }
    });

    // l(x) = \Pi_{i=0}^{n-1} (x - x_i)
    let l = move |x: FieldElement<P>| {
      let mut val = FieldElement::<P>::ONE;
      for i in 0..n {
        val = val * x - self.basis.nodes[i];
      }
      val
    };

    // L(x) = l(x) * \Sigma_{j=0}^{n-1}  (w_j / (x - x_j)) y_j
    l(x)
      * weights.iter().zip(self.coefficients.iter()).zip(self.basis.nodes.iter()).fold(
        FieldElement::<P>::ZERO,
        |acc, ((w, &c), &n)| {
          if n == x {
            return c;
          }
          acc + c * *w / (x - n)
        },
      )
  }
}


impl<const P: usize, const D: usize> fmt::Display
  for Polynomial<Lagrange<P>, P, D>
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let d = self.num_terms() - 1;
    for (idx, (coeff, node)) in self.coefficients.iter().zip(self.basis.nodes.iter()).enumerate() {
      if idx == d {
        write!(f, "{}*l_{}(x)", coeff, node)?;
        break;
      }
      write!(f, "{}*l_{}(x) + ", coeff, node)?;
    }
    Ok(())
  }
}

impl<const P: usize, const D: usize, const D2: usize> Add<Polynomial<Monomial, P, D2>>
  for Polynomial<Monomial, P, D>
{
  type Output = Self;

  /// Implements addition of two polynomials by adding their coefficients.
  /// Note: degree of first operand > deg of second operand.
  fn add(self, rhs: Polynomial<Monomial, P, D2>) -> Self {
    let coefficients = self
      .coefficients
      .iter()
      .zip(rhs.coefficients.iter().chain(std::iter::repeat(&FieldElement::<P>::ZERO)))
      .map(|(&a, &b)| a + b)
      .take(D)
      .collect::<Vec<FieldElement<P>>>()
      .try_into()
      .unwrap_or_else(|v: Vec<FieldElement<P>>| panic!("Expected a Vec of length {} but it was {}", D, v.len()));
    Self { coefficients, basis: self.basis }
  }
}


impl<const P: usize, const D: usize, const D2: usize> Mul<Polynomial<Monomial, P, D2>>
  for Polynomial<Monomial, P, D>
where [(); D + D2 - 1]:
{
  type Output = Polynomial<Monomial, P, { D + D2 - 1 }>;

  /// Implements multiplication of two polynomials by computing:
  /// $$
  /// (a_0 + a_1 x + a_2 x^2 + \ldots) \times (b_0 + b_1 x + b_2 x^2 + \ldots) = c_0 + c_1 x + c_2
  /// x^2
  /// + \ldots $$ where $c_i = \sum_{j=0}^{i} a_j b_{i-j}$.
  ///
  /// Note: Returns a polynomial of degree $D1+D2-1$
  fn mul(self, rhs: Polynomial<Monomial, P, D2>) -> Self::Output {
    let mut coefficients = [FieldElement::<P>::ZERO; D + D2 - 1];
    for i in 0..D {
      for j in 0..D2 {
        coefficients[i + j] += self.coefficients[i] * rhs.coefficients[j];
      }
    }
    Polynomial::<Monomial, P, { D + D2 - 1 }>::new(coefficients)
  }
}