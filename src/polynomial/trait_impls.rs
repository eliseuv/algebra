use super::Polynomial;
use crate::ring::RingBase;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ============================================================================
// 1. Unital and Display Implementations
// ============================================================================

/// Implementation of Display trait for pretty printing polynomials.
/// Formats as "a + b x + c x^2 + ..." skipping zero coefficients.
impl<T> Display for Polynomial<T>
where
    T: RingBase + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coeffs.is_empty() {
            return write!(f, "0");
        }

        let repr = self
            .coeffs
            .iter()
            .enumerate()
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

        if repr.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{repr}")
        }
    }
}

/// Implementation of RingBase for Polynomial<T>.
/// Defines zero and one elements.
impl<T> RingBase for Polynomial<T>
where
    T: RingBase,
{
    fn zero() -> Self {
        Self::order_zero()
    }

    fn one() -> Self {
        Self {
            coeffs: vec![T::one()],
        }
    }
}

impl<T: RingBase> Polynomial<T> {
    // Internal helper for explicit zero
    fn order_zero() -> Self {
        Self { coeffs: vec![] }
    }
}

// ============================================================================
// 2. Unary Operations
// ============================================================================

/// Negate a polynomial: -P(x)
impl<T> Neg for Polynomial<T>
where
    T: RingBase + Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for c in self.coeffs.iter_mut() {
            *c = -(*c);
        }
        self
    }
}

/// Negate a polynomial reference: -&P(x)
impl<'a, T> Neg for &'a Polynomial<T>
where
    T: RingBase + Neg<Output = T> + Copy,
{
    type Output = Polynomial<T>;

    fn neg(self) -> Self::Output {
        // Clone and negate
        let mut result = self.clone();
        for c in result.coeffs.iter_mut() {
            *c = -(*c);
        }
        result
    }
}

// ============================================================================
// 3. Polynomial-Polynomial Operations
// ============================================================================

// --- AddAssign ---

/// In-place addition: P(x) += Q(x)
/// Requires owned arithmetic bounds on T to avoid recursion.
impl<T> AddAssign for Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        if rhs.coeffs.len() > self.coeffs.len() {
            self.coeffs.resize(rhs.coeffs.len(), T::zero());
        }
        for (i, c) in rhs.coeffs.iter().enumerate() {
            self.coeffs[i] += *c;
        }
        self.normalize();
    }
}

impl<'a, T> AddAssign<&'a Polynomial<T>> for Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: &'a Polynomial<T>) {
        if rhs.coeffs.len() > self.coeffs.len() {
            self.coeffs.resize(rhs.coeffs.len(), T::zero());
        }
        for (i, c) in rhs.coeffs.iter().enumerate() {
            self.coeffs[i] += *c;
        }
        self.normalize();
    }
}

// --- SubAssign ---

/// In-place subtraction: P(x) -= Q(x)
impl<T> SubAssign for Polynomial<T>
where
    T: RingBase + Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        if rhs.coeffs.len() > self.coeffs.len() {
            self.coeffs.resize(rhs.coeffs.len(), T::zero());
        }
        for (i, c) in rhs.coeffs.iter().enumerate() {
            self.coeffs[i] -= *c;
        }
        self.normalize();
    }
}

impl<'a, T> SubAssign<&'a Polynomial<T>> for Polynomial<T>
where
    T: RingBase + Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: &'a Polynomial<T>) {
        if rhs.coeffs.len() > self.coeffs.len() {
            self.coeffs.resize(rhs.coeffs.len(), T::zero());
        }
        for (i, c) in rhs.coeffs.iter().enumerate() {
            self.coeffs[i] -= *c;
        }
        self.normalize();
    }
}

// --- MulAssign ---

/// In-place multiplication: P(x) *= Q(x)
impl<T> MulAssign for Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T> + Mul<Output = T> + AddAssign<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        if self.degree().is_none() || rhs.degree().is_none() {
            *self = Polynomial::zero();
            return;
        }
        let mut new_coeffs = vec![T::zero(); self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, c1) in self.coeffs.iter().enumerate() {
            for (j, c2) in rhs.coeffs.iter().enumerate() {
                new_coeffs[i + j] += *c1 * *c2;
            }
        }
        self.coeffs = new_coeffs;
        self.normalize();
    }
}

impl<'a, T> MulAssign<&'a Polynomial<T>> for Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T> + Mul<Output = T> + AddAssign<T>,
{
    fn mul_assign(&mut self, rhs: &'a Polynomial<T>) {
        // Optimization: if rhs is small, maybe don't clone? But we need a new buffer anyway.
        // Effectively same logic as owned.
        if self.degree().is_none() || rhs.degree().is_none() {
            *self = Polynomial::zero();
            return;
        }
        let mut new_coeffs = vec![T::zero(); self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, c1) in self.coeffs.iter().enumerate() {
            for (j, c2) in rhs.coeffs.iter().enumerate() {
                new_coeffs[i + j] += *c1 * *c2;
            }
        }
        self.coeffs = new_coeffs;
        self.normalize();
    }
}

// --- Add ---

/// P(x) + Q(x)
impl<T> Add for Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

/// &P(x) + &Q(x)
impl<'a, 'b, T> Add<&'b Polynomial<T>> for &'a Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    type Output = Polynomial<T>;
    fn add(self, rhs: &'b Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

/// P(x) + &Q(x)
impl<'a, T> Add<&'a Polynomial<T>> for Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    type Output = Polynomial<T>;
    fn add(mut self, rhs: &'a Polynomial<T>) -> Polynomial<T> {
        self += rhs;
        self
    }
}

/// &P(x) + Q(x)
impl<'a, T> Add<Polynomial<T>> for &'a Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    type Output = Polynomial<T>;
    fn add(self, mut rhs: Polynomial<T>) -> Polynomial<T> {
        rhs += self;
        rhs
    }
}

// --- Sub ---

/// P(x) - Q(x)
impl<T> Sub for Polynomial<T>
where
    T: RingBase + Copy + SubAssign,
{
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

/// &P(x) - &Q(x)
impl<'a, 'b, T> Sub<&'b Polynomial<T>> for &'a Polynomial<T>
where
    T: RingBase + Copy + SubAssign,
{
    type Output = Polynomial<T>;
    fn sub(self, rhs: &'b Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

/// P(x) - &Q(x)
impl<'a, T> Sub<&'a Polynomial<T>> for Polynomial<T>
where
    T: RingBase + Copy + SubAssign,
{
    type Output = Polynomial<T>;
    fn sub(mut self, rhs: &'a Polynomial<T>) -> Polynomial<T> {
        self -= rhs;
        self
    }
}

/// &P(x) - Q(x)
impl<'a, T> Sub<Polynomial<T>> for &'a Polynomial<T>
where
    T: RingBase + Copy + SubAssign + Neg<Output = T>,
{
    type Output = Polynomial<T>;
    fn sub(self, rhs: Polynomial<T>) -> Polynomial<T> {
        // P - Q = -(Q - P)
        let mut result = rhs;
        result -= self;
        -result
    }
}

// --- Mul ---

/// P(x) * Q(x)
impl<T> Mul for Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T> + Mul<Output = T> + AddAssign<T>,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

/// &P(x) * &Q(x)
impl<'a, 'b, T> Mul<&'b Polynomial<T>> for &'a Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T> + Mul<Output = T> + AddAssign<T>,
{
    type Output = Polynomial<T>;
    fn mul(self, rhs: &'b Polynomial<T>) -> Polynomial<T> {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

/// P(x) * &Q(x)
impl<'a, T> Mul<&'a Polynomial<T>> for Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T> + Mul<Output = T> + AddAssign<T>,
{
    type Output = Polynomial<T>;
    fn mul(mut self, rhs: &'a Polynomial<T>) -> Polynomial<T> {
        self *= rhs;
        self
    }
}

/// &P(x) * Q(x)
impl<'a, T> Mul<Polynomial<T>> for &'a Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T> + Mul<Output = T> + AddAssign<T>,
{
    type Output = Polynomial<T>;
    fn mul(self, mut rhs: Polynomial<T>) -> Polynomial<T> {
        rhs *= self;
        rhs
    }
}

// ============================================================================
// 4. Polynomial-Scalar Operations
// ============================================================================

// --- MulAssign Scalar ---

/// P(x) *= scalar
impl<T> MulAssign<T> for Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        for c in self.coeffs.iter_mut() {
            *c *= rhs;
        }
        self.normalize();
    }
}

// --- DivAssign Scalar ---

/// P(x) /= scalar
impl<T> DivAssign<T> for Polynomial<T>
where
    T: RingBase + Copy + DivAssign<T>,
{
    fn div_assign(&mut self, rhs: T) {
        for c in self.coeffs.iter_mut() {
            *c /= rhs;
        }
        self.normalize();
    }
}

// --- Add/Sub/Mul/Div Scalar ---

/// P(x) + scalar
impl<T> Add<T> for Polynomial<T>
where
    T: RingBase + Copy + AddAssign,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self {
        if self.coeffs.is_empty() {
            self.coeffs.push(rhs);
        } else {
            self.coeffs[0] += rhs;
        }
        self.normalize();
        self
    }
}

/// P(x) - scalar
impl<T> Sub<T> for Polynomial<T>
where
    T: RingBase + Copy + SubAssign,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        if self.coeffs.is_empty() {
            // 0 - s = -s. Can't easy do with SubAssign avoiding Neg.
            // Assume we can store -rhs.
            // But SubAssign is easier: 0 -= rhs
            self.coeffs.push(T::zero());
        }
        self.coeffs[0] -= rhs;
        self.normalize();
        self
    }
}

/// P(x) * scalar
impl<T> Mul<T> for Polynomial<T>
where
    T: RingBase + Copy + MulAssign<T>,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}

/// P(x) / scalar
impl<T> Div<T> for Polynomial<T>
where
    T: RingBase + Copy + DivAssign<T>,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        self /= rhs;
        self
    }
}
