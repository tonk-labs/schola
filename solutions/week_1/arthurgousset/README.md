# Implement naive RSA

Uses [`primes`](https://docs.rs/primes/latest/primes/) dependency to generate prime numbers.

## Raw notes

> the totatives of *n* \= 9 are the six numbers 1, 2, 4, 5, 7 and 8. They are all relatively prime
> to 9, but the other three numbers in this range, 3, 6, and 9 are not, since gcd(9, 3) = gcd(9, 6)
> = 3 and gcd(9, 9) = 9. Therefore, *φ*(9) = 6.

Source: [wikipedia.org](https://en.wikipedia.org/wiki/Euler%27s_totient_function)

> Setting up an RSA system involves choosing large prime numbers p and q,
> computing *n* \= *pq* and *k* \= *φ*(_n_), and finding two numbers e and d such that *ed* ≡ 1
> (mod *k*). The numbers n and e (the "encryption key") are released to the public, and d (the
> "decryption key") is kept private.
>
> A message, represented by an integer m, where 0 < *m* < *n*, is encrypted by
> computing *S* \= *m*^_e_^ (mod *n*).
>
> It is decrypted by computing *t* \= *S*^_d_^ (mod *n*). Euler's Theorem can be used to show that
> if 0 < *t* < *n*, then *t* \= *m*.
>
> The security of an RSA system would be compromised if the number n could be efficiently factored
> or if *φ*(_n_) could be efficiently computed without factoring n.

Source: [wikipedia.org](https://en.wikipedia.org/wiki/Euler%27s_totient_function#The_RSA_cryptosystem)

