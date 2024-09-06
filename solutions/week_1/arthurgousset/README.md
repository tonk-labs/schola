# Implement naive RSA

## Implementation

### Dependencies

Uses `rand` for random number generation to ensure the generated primes are different.
Uses [`primes`](https://docs.rs/primes/latest/primes/) dependency to generate prime numbers.
Uses `num-bigint` and `num-traits` to work with large integers.

### Limitations

1. The prime number generation is super naive.
2. I'm not sure if I support encrypting 

## Theory

### RSA algorithm

#### 1. **Generate Prime Numbers (p and q)**

- **Inputs**: Bit length for primes (e.g., 8 bits for simplicity)
- **Outputs**: Two large prime numbers, $p$ and $q$
- **Function**: `generate_primes(bit_length)`

#### 2. **Calculate the Modulus (n)**

- **Inputs**: Prime numbers $p$ and $q$
- **Outputs**: Modulus $n$, where $n = p \times q$
- **Function**: `calculate_modulus(p, q)`

#### 3. **Calculate Euler’s Totient (φ(n))**

- **Inputs**: Prime numbers $p$ and $q$
- **Outputs**: Totient $φ(n) = (p-1) \times (q-1)$
- **Function**: `calculate_totient(p, q)`

#### 4. **Choose Public Exponent (e)**

- **Inputs**: Totient $φ(n)$
- **Outputs**: Public exponent $e$, which should be relatively prime to $φ(n)$ (commonly chosen
  small values are 3, 17, 65537)
- **Function**: `choose_public_exponent(totient)`

#### 5. **Calculate Private Exponent (d)**

- **Inputs**: Public exponent $e$, totient $φ(n)$
- **Outputs**: Private exponent $d$, which is the modular multiplicative inverse of $e$ modulo
  $φ(n)$
- **Function**: `calculate_private_exponent(e, totient)`

#### 6. **Key Generation**

- **Inputs**: None (will internally use the above functions)
- **Outputs**: Public key $(n, e)$ and private key $(n, d)$
- **Function**: `generate_keys(bit_length)`

#### 7. **Encryption Function**

- **Inputs**: Plaintext message $M$ and public key $(n, e)$
- **Outputs**: Ciphertext $C = M^e \mod n$
- **Function**: `encrypt(message, public_key)`

#### 8. **Decryption Function**

- **Inputs**: Ciphertext $C$ and private key $(n, d)$
- **Outputs**: Decrypted message $M = C^d \mod n$
- **Function**: `decrypt(ciphertext, private_key)`

### TLDR on RSA (Rivest-Shamir-Adleman) Algorithm

#### **1. Overview of RSA:**

- **RSA** is a widely-used public-key cryptographic algorithm that facilitates secure data
  transmission and digital signatures. Named after its inventors, Ron Rivest, Adi Shamir, and
  Leonard Adleman, RSA is based on the mathematical principles of number theory, specifically the
  difficulty of factoring large composite numbers.

#### **2. Theoretical Perspective:**

- **Public-Key Cryptography**: RSA is an example of an asymmetric cryptographic system, meaning it
  uses a pair of keys: a public key (which can be shared openly) and a private key (which is kept
  secret). Messages encrypted with the public key can only be decrypted with the corresponding
  private key, and vice versa.

- **Core Concept - Factoring Problem**: RSA's security is grounded in the fact that, while it is
  easy to multiply two large prime numbers together to form a composite number, it is
  computationally hard to reverse the process (i.e., to factorize the composite number back into its
  prime factors).

- **Mathematical Foundations**:

  - **Prime Numbers (p and q)**: RSA starts with two large prime numbers.
  - **Modulus (n)**: The product of these primes, $n = p \times q$, forms the modulus for both the
    public and private keys.
  - **Totient Function (φ(n))**: Euler's totient function is calculated as
    $φ(n) = (p-1) \times (q-1)$. It represents the count of numbers less than $n$ that are coprime
    to $n$.
  - **Public Exponent (e)**: A number $e$ is chosen such that $1 < e < φ(n)$ and
    $\text{gcd}(e, φ(n)) = 1$. Common choices for $e$ include 3, 17, and 65537.
  - **Private Exponent (d)**: The private key exponent $d$ is computed as the modular multiplicative
    inverse of $e$ modulo $φ(n)$, ensuring $d \times e \equiv 1 \, (\text{mod} \, φ(n))$.

- **Key Pair Generation**: The public key consists of $(n, e)$, while the private key consists of
  $(n, d)$.

- **Encryption and Decryption**:
  - **Encryption**: A plaintext message $M$ is encrypted to ciphertext $C$ using the recipient’s
    public key: $C = M^e \mod n$.
  - **Decryption**: The ciphertext $C$ is decrypted back to plaintext $M$ using the private key:
    $M = C^d \mod n$.

#### **3. Practical Perspective:**

- **Key Size**: In practice, RSA keys are typically 2048 bits or longer to ensure security against
  modern computational power. Larger key sizes increase security but also computational
  requirements.

- **Applications**:

  - **Secure Communication**: RSA is used in protocols like HTTPS, SSL/TLS to secure communication
    over the internet.
  - **Digital Signatures**: RSA can be used to sign a document. The sender encrypts a hash of the
    document with their private key. The recipient can verify the signature using the sender's
    public key.
  - **Key Exchange**: While RSA can encrypt data directly, it's often used in conjunction with
    symmetric encryption algorithms (e.g., AES) for key exchange due to efficiency.

- **Performance**: RSA is slower than symmetric key algorithms because it involves large integer
  arithmetic operations. Therefore, it's primarily used for encrypting small amounts of data (e.g.,
  keys) rather than large volumes.

- **Security Considerations**:
  - **Padding Schemes**: To mitigate certain types of attacks (e.g., timing attacks), RSA
    implementations often use padding schemes like OAEP (Optimal Asymmetric Encryption Padding).
  - **Quantum Threat**: RSA’s security is threatened by advances in quantum computing, which could
    factorize large numbers efficiently. Post-quantum cryptography is an area of research focusing
    on cryptographic systems resilient to quantum attacks.

#### **4. Key Terms and Concepts**:

- **Asymmetric Encryption**: Involves two separate keys (public and private) for encryption and
  decryption.
- **Public Key and Private Key**: Public key is shared openly for encryption, and the private key is
  kept secret for decryption.
- **Modulus (n)**: Product of two large prime numbers used in both public and private keys.
- **Totient (φ(n))**: A function representing the number of integers less than $n$ that are coprime
  to $n$.
- **Exponent (e, d)**: Numbers used in the encryption (public exponent $e$) and decryption (private
  exponent $d$) processes.
- **Digital Signatures**: Verifies the authenticity and integrity of a message using the sender's
  private key.
- **Padding Schemes (e.g., OAEP)**: Techniques to securely encode plaintext before encryption,
  providing security against certain attacks.

Implementing a "naive RSA" is a great exercise to solidify your understanding of the RSA algorithm.
In this context, "naive" means that you'll be focusing on the basic, straightforward implementation
without optimizations, advanced security features, or handling of edge cases (such as very large
primes, padding, etc.). This approach will help you grasp the core concepts and steps involved in
RSA encryption and decryption.

Absolutely! Understanding Euler's totient function is crucial for implementing RSA correctly. In the
context of RSA, Euler's totient function helps in generating the public and private keys, which are
essential for secure encryption and decryption.

### TLDR on Euler’s Totient Function

Euler's totient function, denoted as $\phi(n)$, counts the number of positive integers up to $n$
that are relatively prime to $n$ (i.e., the numbers less than $n$ that do not share any common
factors with $n$ other than 1). In the RSA algorithm, $n$ is the product of two large prime numbers
$p$ and $q$.

In the RSA algorithm, $n$ is defined as $n = p \times q$, where $p$ and $q$ are two distinct prime
numbers.

#### **Formula for Euler's Totient Function $\phi(n)$ with Two Primes:**

When $n$ is the product of two distinct prime numbers $p$ and $q$:

$\phi(n) = \phi(p \times q) = (p-1) \times (q-1)$

#### **Why This Formula Works:**

- Since $p$ and $q$ are prime, every number less than $p$ is relatively prime to $p$, except for
  multiples of $p$.
- Similarly, every number less than $q$ is relatively prime to $q$, except for multiples of $q$.
- When $n = p \times q$, the totient $\phi(n)$ is the count of integers less than $n$ that are not
  divisible by either $p$ or $q$. This count is given by the product $(p-1) \times (q-1)$.

#### Example

Let’s calculate $\phi(n)$ for a specific example where $p = 5$ and $q = 11$:

1. **Step 1: Compute $n$**

   $n = p \times q = 5 \times 11 = 55$

2. **Step 2: Compute $\phi(n)$**

   $\phi(n) = (p-1) \times (q-1)$ $\phi(55) = (5-1) \times (11-1) = 4 \times 10 = 40$

So, $\phi(55) = 40$, meaning there are 40 numbers less than 55 that are relatively prime to 55.

Certainly! Implementing the function to find the modular multiplicative inverse is an important step
in the RSA algorithm. In RSA, you typically need to compute the private key $d$ as the modular
multiplicative inverse of the public exponent $e$ modulo $\phi(n)$. This means finding $d$ such
that:

$d \times e \equiv 1 \ (\text{mod} \ k)$

In other words, $d$ is the value such that when multiplied by $e$, the result is congruent to 1
modulo $k$. The extended Euclidean algorithm is commonly used to compute this efficiently.

### TLDR on modular multiplicative inverse

The modular multiplicative inverse $d$ is found using the Extended Euclidean Algorithm. The function
checks whether $e$ and $k$ are coprime (i.e., their gcd is 1). It computes $d$ such that
$d \times e \equiv 1 \ (\text{mod} \ k)$.

The Extended Euclidean Algorithm is an extension of the Euclidean algorithm. It not only finds the
greatest common divisor (gcd) of two integers but also finds coefficients $x$ and $y$ (known as the
Bézout coefficients) such that:

$x \cdot a + y \cdot b = \text{gcd}(a, b)$

For finding the modular multiplicative inverse, you specifically want:

$d \times e \equiv 1 \ (\text{mod} \ k)$

This is equivalent to:

$d \cdot e + k \cdot y = 1$

Here, $d$ will be the inverse of $e$ modulo $k$.

