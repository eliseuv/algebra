//! Polynomial Algebra
//!

use std::ops::{Add, Mul, Neg};

use crate::ring::RingBase;

/// Dense Polynomial
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T> {
    pub(crate) coeffs: Vec<T>,
}

impl<T: RingBase> Polynomial<T> {
    /// Zero polynomial
    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }

    /// New polynomial with given coefficients
    pub fn from_coeffs(coeffs: Vec<T>) -> Self {
        let mut poly = Self { coeffs };
        poly.normalize();
        poly
    }

    /// Remove high-order zero terms
    pub fn normalize(&mut self) {
        while let Some(last) = self.coeffs.last() {
            if *last != T::zero() {
                break;
            }
            self.coeffs.pop();
        }
    }

    /// Get constant term
    pub fn constant_term(&self) -> Option<&T> {
        self.coeffs.first()
    }

    /// Get the degree of the polynomial
    pub fn degree(&self) -> Option<usize> {
        match self.coeffs.as_slice() {
            [] => None,
            coeffs => Some(coeffs.len() - 1),
        }
    }
}

impl<T> Polynomial<T>
where
    T: RingBase + Neg<Output = T>,
{
    /// Polynomial with single root
    /// $ p(x) = (x - x_0) $
    #[inline(always)]
    pub(crate) fn single_root(x_0: T) -> Self {
        Self {
            coeffs: vec![-x_0, T::one()],
        }
    }
}

impl<T> Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    /// Evaluate the polynomial at a given point
    pub fn evaluate(&self, x: &T) -> T {
        self.coeffs
            .iter()
            .rev()
            .fold(T::zero(), |acc, coeff| &(&acc * x) + coeff)
    }
}

mod trait_impls;

pub mod lagrange;
