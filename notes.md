# To do

- Shamir's secret sharing algorithm
- Reed-Solomon error correction
- Implement `Ring` trait:
  - Primitive `i32` and `f64` using macros
  - `FiniteField` struct
- Karatsuba and FFT polynomial multiplication
- Support large primes (like the BLS12-381 curve used in ZK-rollups)
  - Replace u64 with a BigInt struct and implement finite field division and power algorithms differently (often using Montgomery Reduction for speed)
  - Elliptic curve with 256-bit fields
- Zero-Knowledge Proofs
