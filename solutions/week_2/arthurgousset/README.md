# Shamir's Secret Sharing

## Implementation

### Dependencies

1. Uses `rand` for random number generation to ensure the generated primes are different.
1. Uses [`primes`](https://docs.rs/primes/latest/primes/) dependency to generate prime numbers.
1. Uses `num-bigint` and `num-traits` to work with large integers.

### Usage

```sh
$ cargo run

Secret is: 42
Number of shares are: 3
Random prime number is: 811
Polynomial coefficients are: [42, 211, 277]
Secret shares: [(1, 530), (2, 761), (3, 735)]
Reconstructed secret: 42
```

```sh
$ cargo run

Secret is: 1559
Number of shares are: 6
Random prime number is: 13327
Polynomial coefficients are: [1559, 7800, 9708, 6656, 10142, 2867]
Secret shares: [(1, 12078), (2, 3426), (3, 11081), (4, 2484), (5, 7847), (6, 11075)]
Reconstructed secret: 1559
```

### Learnings

Difference between `&[u64]` (borrowed slice) and `Vec<u64>` (owned vector).

### Improvements

1. The prime number does not have to be randomly generated. In fact, it can be publicly known. I can
   simply use a large known prime number. I can hardcode it at the start.
2. The operations should be performed with `BigInt` using the large prime number.
3. I should introduce a distinction between the shares and the threshold.

## Theory

Source: ChatGPT

### TLDR on Shamir's Secret Sharing

#### Theory Perspective

Shamir's Secret Sharing is a cryptographic algorithm designed to split a secret into multiple parts,
such that only a subset of those parts can reconstruct the secret. It's based on the mathematical
concept of **polynomial interpolation** over a finite field, ensuring security even if some parts
are lost or compromised.

The key theoretical ideas are:

1. **Secret Splitting with Polynomial**: The secret is represented as the constant term of a
   polynomial, and the other terms of the polynomial are generated randomly. This polynomial is
   evaluated at different points to produce shares.

2. **Threshold Scheme**: The algorithm works on a `(t, n)` threshold scheme. This means:

   - **n**: The secret is split into `n` parts (shares).
   - **t**: Any subset of `t` shares (where `t ≤ n`) can reconstruct the secret.
   - Fewer than `t` shares are useless and don't reveal any information about the secret.

3. **Polynomial Interpolation**: The reconstruction of the secret involves using **Lagrange
   interpolation** to determine the polynomial that generated the shares. Once the polynomial is
   reconstructed, the secret (the constant term) can be extracted.

4. **Finite Fields (Galois Fields)**: For security and ease of computation, the operations are done
   over a finite field (such as **GF(2^k)**, where `k` is a number that determines the size of the
   field), making the calculations both efficient and secure.

#### Practical Perspective

From a practical standpoint, Shamir's Secret Sharing is used when secure data needs to be
distributed among multiple parties, but no single party should have access to the full secret unless
a quorum (threshold) of parties agrees to combine their shares.

Key terms and practical considerations:

1. **Secret**: The data to be protected (e.g., a cryptographic key, password, or sensitive
   information).

2. **Shares**: The `n` parts that are distributed to different parties. These shares are
   independently useless until `t` or more of them are combined to reconstruct the secret.

3. **Threshold**: The minimum number (`t`) of shares required to reconstruct the secret. This allows
   flexibility in access control — for example, in a company, you could require at least three
   executives out of five to approve accessing sensitive data.

4. **Use Cases**:

   - **Distributed key management**: Useful in distributed systems where a key must be split for
     security and resilience.
   - **Multi-party secure computing**: Allows collaborative computation while ensuring that no
     single party can access sensitive data unless a threshold of cooperation is reached.
   - **Backup security**: Protects data backups by splitting encryption keys among several
     custodians, ensuring data can only be restored with cooperation.

5. **Reconstruction**: To reconstruct the secret, gather at least `t` shares and apply polynomial
   interpolation to recover the polynomial, and ultimately, the secret (the constant term).

6. **Security**: If fewer than `t` shares are gathered, no information about the secret is leaked
   due to the properties of polynomial interpolation in finite fields.

### High-Level Example

- **Setup**: We want to split a secret `S` among 5 participants, but require any 3 participants to
  reconstruct it. This is a `(3, 5)` scheme.
  1.  Choose a random polynomial of degree `t-1 = 2`, such as `f(x) = S + a1*x + a2*x^2`, where `S`
      is the secret, and `a1`, `a2` are random coefficients.
  2.  Create 5 shares by evaluating the polynomial at 5 different points (e.g.,
      `f(1), f(2), ..., f(5)`).
  3.  Distribute these shares to 5 participants.
- **Reconstruction**: Any 3 participants combine their shares, use polynomial interpolation to
  reconstruct the polynomial `f(x)`, and retrieve the secret `S` from the constant term.

### Summary of Key Terms and Concepts:

- **Secret**: The data to be protected.
- **Shares**: Parts of the secret distributed to participants.
- **Threshold `(t, n)`**: Minimum `t` shares needed to reconstruct the secret from `n` total shares.
- **Polynomial Interpolation**: Mathematical technique used to reconstruct the secret from `t`
  shares.
- **Finite Field**: The mathematical space used to ensure secure operations and efficient
  computation.

Shamir's Secret Sharing is widely used in scenarios where distributed access control or resilient,
decentralized systems are needed to secure sensitive data while ensuring redundancy and fault
tolerance.

### TLDR on Lagrange Interpolation

#### Theoretical Perspective

Polynomial interpolation is the process of finding a **polynomial** that passes through a given set of points (data pairs). More formally, given `n` points `(x₁, y₁), (x₂, y₂), ..., (xₙ, yₙ)`, the goal is to find a polynomial `P(x)` of degree `n-1` that satisfies the condition `P(xᵢ) = yᵢ` for each point.

Key ideas:

1. **Polynomial Definition**: 
   A polynomial is an expression of the form:
   $
   P(x) = a₀ + a₁x + a₂x² + ... + aₙxⁿ
   $
   where `a₀, a₁, a₂, ..., aₙ` are constants (called coefficients) and `x` is the variable.

2. **Unique Polynomial**: 
   Given `n` distinct points, there is **exactly one polynomial** of degree `n-1` that passes through all those points. This is guaranteed by the **Fundamental Theorem of Algebra**.

3. **Lagrange Interpolation**: 
   One method of constructing the polynomial is **Lagrange interpolation**, where the interpolating polynomial is constructed as a weighted sum of **basis polynomials**. Each basis polynomial is designed to be 1 at its corresponding `xᵢ` and 0 at all other `xⱼ` (j ≠ i).

4. **Newton Interpolation**: 
   Another method is **Newton's interpolation**, which constructs the polynomial incrementally using **divided differences**.

#### Practical Perspective

Polynomial interpolation has practical applications in fields where we need to estimate or model data from a small number of sample points, or reconstruct continuous functions from discrete data points.

Key concepts and terms:

1. **Interpolation vs. Extrapolation**: 
   - **Interpolation** is estimating the values between known data points.
   - **Extrapolation** is estimating values beyond the known data points (but less reliable, as it can lead to inaccurate predictions).

2. **Fitting Points**: 
   In practice, given a set of `n` data points, polynomial interpolation allows us to construct a smooth curve (polynomial) that passes exactly through all points. This is useful for:
   - Signal reconstruction.
   - Data smoothing.
   - Estimating missing values.

3. **Overfitting**: 
   High-degree polynomials can be sensitive to small changes in data and lead to overfitting (producing a curve with extreme oscillations). In such cases, simpler methods like piecewise interpolation (e.g., **splines**) are often preferred.

4. **Numerical Stability**: 
   Large degree polynomials can also suffer from numerical instability, where small errors in data or calculation lead to significant deviations. Algorithms like Newton's method or Chebyshev polynomials can help mitigate these issues.

### Examples

#### 1. Simple Interpolation with Two Points

Given two points: `(x₁ = 1, y₁ = 3)` and `(x₂ = 2, y₂ = 5)`, the goal is to find a **linear polynomial** `P(x) = a₀ + a₁x` that passes through these points.

- Set up two equations:
  - `P(1) = 3` → `a₀ + a₁(1) = 3`
  - `P(2) = 5` → `a₀ + a₁(2) = 5`

- Solve the system:
  - From `P(1)`, we get `a₀ + a₁ = 3`.
  - From `P(2)`, we get `a₀ + 2a₁ = 5`.
  
  Solving this, `a₀ = 1` and `a₁ = 2`, so the polynomial is:
  $
  P(x) = 1 + 2x
  $
  This is a straight line passing through `(1, 3)` and `(2, 5)`.

#### 2. Quadratic Interpolation with Three Points

Given three points: `(x₁ = 0, y₁ = 1)`, `(x₂ = 1, y₂ = 3)`, `(x₃ = 2, y₃ = 2)`, find a **quadratic polynomial** `P(x) = a₀ + a₁x + a₂x²`.

- Set up three equations:
  - `P(0) = 1` → `a₀ = 1`
  - `P(1) = 3` → `1 + a₁(1) + a₂(1)² = 3` → `a₁ + a₂ = 2`
  - `P(2) = 2` → `1 + a₁(2) + a₂(2)² = 2` → `2a₁ + 4a₂ = 1`

- Solve the system:
  - From `a₁ + a₂ = 2` and `2a₁ + 4a₂ = 1`, solve for `a₁` and `a₂`.
  - `a₁ = 5`, `a₂ = -3`.

  So the quadratic polynomial is:
  $
  P(x) = 1 + 5x - 3x²
  $
  This is the curve that passes through `(0, 1)`, `(1, 3)`, and `(2, 2)`.

#### 3. Lagrange Interpolation Example

Given the points `(x₁ = 1, y₁ = 2)`, `(x₂ = 3, y₂ = 4)`, and `(x₃ = 4, y₃ = 3)`, we can construct the **Lagrange polynomial**.

- Basis polynomials:
  $
  L₁(x) = \frac{(x - 3)(x - 4)}{(1 - 3)(1 - 4)},\quad L₂(x) = \frac{(x - 1)(x - 4)}{(3 - 1)(3 - 4)},\quad L₃(x) = \frac{(x - 1)(x - 3)}{(4 - 1)(4 - 3)}
  $

- The interpolating polynomial `P(x)` is:
  $
  P(x) = y₁L₁(x) + y₂L₂(x) + y₃L₃(x)
  $

  After calculating and simplifying, this yields the polynomial that passes through all three points.

### Summary of Key Terms and Concepts:
- **Polynomial**: A mathematical expression consisting of terms involving powers of `x`.
- **Interpolation**: Finding a polynomial that passes through given data points.
- **Degree of Polynomial**: For `n` points, the polynomial has degree `n-1`.
- **Lagrange Interpolation**: A method to construct the interpolating polynomial using basis polynomials.
- **Newton Interpolation**: Another method based on incremental construction using divided differences.
- **Numerical Stability**: Large polynomials can be unstable, so careful algorithms are often needed.
- **Overfitting**: High-degree polynomials may fit the points but can behave erratically between or beyond them.

Polynomial interpolation is a foundational concept in numerical analysis and data science for reconstructing smooth functions from discrete data points.

### Algorithm 

#### Inputs:

1. **Secret** (`S`): 
   The secret to be shared, usually an integer (e.g., a cryptographic key or a number).
   
2. **Number of Shares** (`n`): 
   The total number of shares you want to generate.
   
3. **Threshold** (`t`): 
   The minimum number of shares needed to reconstruct the secret. This is the key number in Shamir's Secret Sharing: any subset of `t` shares can reconstruct the secret, while fewer than `t` cannot.

4. **Finite Field** (`p`): 
   A prime number to define the finite field over which all arithmetic will take place. This ensures security and allows us to deal with numbers without overflow issues. Typically, we use a large prime number greater than the secret.

#### Functions:

1. **Polynomial Construction**:
   - A function that constructs a random polynomial `P(x)` of degree `t-1` with the secret as the constant term. The form of the polynomial is:
     $
     P(x) = S + a₁x + a₂x² + ... + a_{t-1}x^{t-1}
     $
   - The coefficients `a₁, a₂, ..., a_{t-1}` are randomly chosen, and `S` is the constant term, i.e., the secret.
   
2. **Share Generation**:
   - A function that evaluates the polynomial `P(x)` at different values of `x` (e.g., `x = 1, 2, ..., n`) to generate the shares. Each share is a tuple `(xᵢ, P(xᵢ))`, where `xᵢ` is the x-coordinate and `P(xᵢ)` is the y-coordinate (the evaluated result).

3. **Secret Reconstruction**:
   - A function that takes at least `t` shares and uses **Lagrange interpolation** to reconstruct the original polynomial, specifically the constant term (the secret `S`).

#### Outputs:

1. **Shares**:
   - A list of `n` shares, where each share is a pair `(xᵢ, P(xᵢ))`.
   
2. **Reconstructed Secret**:
   - The original secret `S`, reconstructed from any `t` shares.

#### Steps

##### Step 1: Generate Random Polynomial
You need to generate a random polynomial `P(x)` where the constant term is the secret, and the other coefficients are random.

##### Step 2: Evaluate Polynomial at `x` Values to Create Shares

Evaluate the polynomial at `x = 1, 2, ..., n` to generate `n` shares.

##### Step 3: Reconstruct the Secret using Lagrange Interpolation

To reconstruct the secret, you need at least `t` shares. You use **Lagrange interpolation** to recover the constant term `S` of the polynomial.

Lagrange interpolation formula:
$
S = \sum_{j=1}^{t} y_j \prod_{1 \leq m \leq t, m \neq j} \frac{x_m}{x_m - x_j} \mod p
$

This will reconstruct the secret from `t` shares.

##### Summary of Inputs, Functions, and Outputs:

- **Inputs**:
  - Secret (integer).
  - Number of shares (`n`).
  - Threshold (`t`).
  - Prime number (`p`).

- **Functions**:
  - `generatePolynomial`: Creates a random polynomial with the secret as the constant term.
  - `evaluatePolynomial`: Evaluates the polynomial at given points to generate shares.
  - `lagrangeInterpolation`: Reconstructs the secret using Lagrange interpolation with a set of `t` shares.

- **Outputs**:
  - A list of `n` shares.
  - The reconstructed secret from any subset of `t` shares.

This basic Shamir's Secret Sharing implementation securely distributes a secret among participants and ensures that only a sufficient number of shares can reveal the secret.