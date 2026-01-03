use algebra::ring::RingBase;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MyF64(pub f64);

impl RingBase for MyF64 {
    fn zero() -> Self {
        MyF64(0.0)
    }
    fn one() -> Self {
        MyF64(1.0)
    }
}

impl Add for MyF64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        MyF64(self.0 + rhs.0)
    }
}
impl Sub for MyF64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        MyF64(self.0 - rhs.0)
    }
}
impl Mul for MyF64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        MyF64(self.0 * rhs.0)
    }
}
impl Div for MyF64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        MyF64(self.0 / rhs.0)
    }
}
impl Neg for MyF64 {
    type Output = Self;
    fn neg(self) -> Self {
        MyF64(-self.0)
    }
}
impl AddAssign for MyF64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl SubAssign for MyF64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl MulAssign for MyF64 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl DivAssign for MyF64 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<'a> Add<&'a MyF64> for &'a MyF64 {
    type Output = MyF64;
    fn add(self, rhs: &'a MyF64) -> MyF64 {
        MyF64(self.0 + rhs.0)
    }
}
impl<'a> Sub<&'a MyF64> for &'a MyF64 {
    type Output = MyF64;
    fn sub(self, rhs: &'a MyF64) -> MyF64 {
        MyF64(self.0 - rhs.0)
    }
}
impl<'a> Mul<&'a MyF64> for &'a MyF64 {
    type Output = MyF64;
    fn mul(self, rhs: &'a MyF64) -> MyF64 {
        MyF64(self.0 * rhs.0)
    }
}
