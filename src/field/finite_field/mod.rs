use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{field::FieldBase, ring::RingBase};

/// Finite field over P
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fp<const P: u64>(pub(crate) u64);

impl<const P: u64> Display for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "F{P}({n})", n = self.0)
    }
}

impl<const P: u64> Fp<P> {
    /// Create a new element
    pub fn new(value: u64) -> Self {
        Self(value % P)
    }

    /// Unwrap the inner value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl<const P: u64> Add for Fp<P> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let sum = self.0 + other.0;
        let sum = if sum >= P { sum - P } else { sum };
        Self(sum)
    }
}

impl<const P: u64> Sub for Fp<P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.0 >= other.0 {
            Self(self.0 - other.0)
        } else {
            Self((self.0 + P) - other.0)
        }
    }
}

impl<const P: u64> Mul for Fp<P> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // Cast to u128 to avoid overflow
        let prod = (self.0 as u128) * (other.0 as u128);
        Fp((prod % P as u128) as u64)
    }
}

impl<const P: u64> RingBase for Fp<P> {
    fn zero() -> Self {
        Fp(0)
    }

    fn one() -> Self {
        Fp(1)
    }
}

impl<const P: u64> Fp<P> {
    /// Power self^exp % P using square and multiply
    pub fn pow(&self, mut exp: u64) -> Self {
        let mut base = *self;
        let mut result = Self::one();

        while exp > 0 {
            // If the current bit in 1, multiply the result by the base
            if exp % 2 == 1 {
                result *= &base;
            }

            // Square the base for next bit
            base = base * base;

            // Shift to next bit
            exp /= 2;
        }

        result
    }
}

impl<const P: u64> FieldBase for Fp<P> {
    /// Inverse using Fermat's little theorem: x^{-1} = x^{P-2}
    fn inverse(&self) -> Self {
        match self {
            Fp(0) => panic!("Inverse of zero is undefined"),
            n => n.pow(P - 2),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const P: u64> Div for Fp<P>
where
    Self: FieldBase,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.inverse()
    }
}

// Reference arithmetic
impl_ref_ops!(Add, add);
impl_ref_ops!(Sub, sub);
impl_ref_ops!(Mul, mul);
impl_ref_ops!(Div, div);

// Assign arithmetic
impl_assign_ops!(AddAssign, add_assign, add);
impl_assign_ops!(SubAssign, sub_assign, sub);
impl_assign_ops!(MulAssign, mul_assign, mul);
impl_assign_ops!(DivAssign, div_assign, div);
