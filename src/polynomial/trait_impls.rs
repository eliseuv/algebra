use super::Polynomial;
use crate::ring::RingBase;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

// Display implementation
impl<T> Display for Polynomial<T>
where
    T: RingBase + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self
            .coeffs
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, coeff)| **coeff != T::zero())
            .map(|(deg, coeff)| {
                let term = format!("{}", coeff);
                match deg {
                    0 => term,
                    1 => term + " x",
                    _ => format!("{term} x^{deg}"),
                }
            })
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "{repr}")
    }
}

impl<T> RingBase for Polynomial<T>
where
    T: RingBase,
{
    fn zero() -> Self {
        Self::zero()
    }

    fn one() -> Self {
        Self::from_coeffs(vec![T::one()])
    }
}

/// Polynomial addition
impl<T> Add for &Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Polynomial<T>;

    fn add(self, other: Self) -> Polynomial<T> {
        // Clone the largest degree polynomial as result
        let (mut result, smaller) = if self.coeffs.len() >= other.coeffs.len() {
            (self.clone(), other)
        } else {
            (other.clone(), self)
        };

        // Add coefficients
        for (c_result, c_other) in result.coeffs.iter_mut().zip(smaller.coeffs.iter()) {
            *c_result = &*c_result + c_other;
        }

        result.normalize();
        result
    }
}

/// Polynomial subtraction
impl<T> Sub for &Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Polynomial<T>;

    fn sub(self, other: Self) -> Polynomial<T> {
        let max_len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let left = if i < self.coeffs.len() {
                &self.coeffs[i]
            } else {
                &T::zero()
            };
            let right = if i < other.coeffs.len() {
                &other.coeffs[i]
            } else {
                &T::zero()
            };
            result_coeffs.push(left - right);
        }

        let mut result = Polynomial::from_coeffs(result_coeffs);
        result.normalize();
        result
    }
}

/// Naive polynomial multiplication
/// TODO: Implement FFT multiplication
impl<T> Mul for &Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Polynomial<T>;

    fn mul(self, other: Self) -> Self::Output {
        // If any of the polynomials is zero the result is zero
        if self.degree().is_none() || other.degree().is_none() {
            return Polynomial::zero();
        }

        let mut result =
            Polynomial::from_coeffs(vec![T::zero(); self.coeffs.len() + other.coeffs.len() - 1]);
        for (i, c_self) in self.coeffs.iter().enumerate() {
            for (j, c_other) in other.coeffs.iter().enumerate() {
                result.coeffs[i + j] = &result.coeffs[i + j] + &(c_self * c_other);
            }
        }

        result
    }
}

// Owned implementations reusing reference ones
impl<T> Add for Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        &self + &other
    }
}

impl<T> Sub for Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        &self - &other
    }
}

impl<T> Mul for Polynomial<T>
where
    T: RingBase,
    for<'a> &'a T: Add<&'a T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        &self * &other
    }
}
