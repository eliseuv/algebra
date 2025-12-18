//! Polynomial Algebra
//!

use std::ops::{Add, Mul};

use crate::ring::{Ring, RingBase};

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
        Self { coeffs }
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

/// Lagrange interpolation
/// https://en.wikipedia.org/wiki/Lagrange_polynomial
/// Given a set of n + 1 points (x_k, y_k), which must be distinct x_i != x_j for i != j, the Lagrange interpolation polynomial is the unique polynomial of degree <= n that passes through all the points.
pub fn lagrange_interpolation<T>(points: &[(T, T)]) -> Polynomial<T>
where
    T: Ring,
{
    // If no points are given, return the zero polynomial
    if points.is_empty() {
        return Polynomial::zero();
    }

    let mut poly = Polynomial::zero();

    // Outer loop
    for (j, &(x_j, y_j)) in points.iter().enumerate() {}

    poly
}

mod trait_impls;
