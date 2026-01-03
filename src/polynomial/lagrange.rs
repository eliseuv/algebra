//! Lagrange Interpolation
//!

use std::ops::{AddAssign, Div, Mul, MulAssign, Neg, Sub};

use crate::polynomial::Polynomial;
use crate::ring::RingBase;

/// Lagrange interpolation
/// https://en.wikipedia.org/wiki/Lagrange_polynomial
/// Given a set of n points (x_k, y_k), which must be distinct x_i != x_j for i != j, the Lagrange interpolation polynomial L(x) is the unique polynomial of degree <= n - 1 that passes through all the points.
/// This is done by constructing the Lagrange basis polynomials l_i(x) that are equal to 1 at x_i and 0 at all other x_j.
/// $$ l_i(x) = \frac{\prod_{j \neq i} (x - x_j)}{\prod_{j \neq i} (x_i - x_j)} $$
/// The resulting Lagrange interpolation polynomial is then given by the linear combination of the basis polynomials weighted by the y_i values.
/// $$ L(x) = \sum_{i=0}^{n-1} y_i l_i(x) $$
pub fn lagrange_interpolation<T>(points: &[(T, T)]) -> Polynomial<T>
where
    T: RingBase
        + Copy
        + MulAssign<T>
        + AddAssign<T>
        + Neg<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>,
    Polynomial<T>: Mul<T, Output = Polynomial<T>>
        + MulAssign<T>
        + MulAssign<Polynomial<T>>
        + AddAssign<Polynomial<T>>,
{
    let mut poly = Polynomial::zero();

    // Loop over basis polynomials
    for (i, (x_i, y_i)) in points.iter().enumerate() {
        // Initialize basis polynomial $l_i(x) = 1$
        let mut poly_i = Polynomial {
            coeffs: vec![T::one()],
        };
        let mut denom = T::one();

        // Product loop
        for (j, (x_j, _)) in points.iter().enumerate() {
            // Skip i == j
            if i == j {
                continue;
            }
            // Accumulate roots
            poly_i *= Polynomial::single_root(*x_j);
            // Accumulate denominator
            denom *= *x_i - *x_j;
        }
        poly += poly_i * (*y_i / denom);
    }

    poly
}
