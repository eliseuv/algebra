//! Number Sets
//!

use std::ops::{Add, Div, Mul, Sub};

/// Greatest Common Divisor
/// Adapted from `uutils`
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Check for zero
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    // Extract common power of 2 divisor
    // gcd(2^k a, 2^k b) = 2^k gcd(a, b)
    // Also reduce both numbers to their odd parts
    let k = {
        let k_a = a.trailing_zeros();
        let k_b = b.trailing_zeros();
        a >>= k_a;
        b >>= k_b;
        k_a.min(k_b)
    };

    loop {
        // Subtract the smaller from the larger
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a -= b;

        // If a becomes zero, b is the GCD
        if a == 0 {
            // Put back common power of 2 divisor
            return b << k;
        }

        // Reduce to odd part
        a >>= a.trailing_zeros();
    }
}

pub struct UnsignedRational {
    num: u64,
    den: u64,
}

impl UnsignedRational {
    pub fn new(num: u64, den: u64) -> Self {
        Self { num, den }.reduce()
    }

    pub fn reduce(mut self) -> Self {
        let gcd = gcd(self.num, self.den);
        self.num /= gcd;
        self.den /= gcd;
        self
    }
}

impl Add for UnsignedRational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let num = self.num * other.den + self.den * other.num;
        let den = self.den * other.den;
        Self { num, den }.reduce()
    }
}

impl Sub for UnsignedRational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let num = self.num * other.den - self.den * other.num;
        let den = self.den * other.den;
        Self { num, den }.reduce()
    }
}

impl Mul for UnsignedRational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let num = self.num * other.num;
        let den = self.den * other.den;
        Self { num, den }
    }
}

impl Div for UnsignedRational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let num = self.num * other.den;
        let den = self.den * other.num;
        Self { num, den }.reduce()
    }
}
