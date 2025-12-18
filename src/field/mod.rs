//! Field Algebra
//! https://ncatlab.org/nlab/show/field
//! A field is a commutative ring in which every non-zero element has a multiplicative inverse and 0 != 1

use std::ops::{Add, Mul};

use crate::ring::{Ring, RingBase};

/// Base trait for Fields
pub trait FieldBase: RingBase {
    /// Element inverse
    /// Panics for the zero element
    fn inverse(&self) -> Self;
}

macro_rules! impl_field_for_primitives {
    ($($t:ty),*) => {
        $(
            impl FieldBase for $t {
                #[inline(always)]
                fn inverse(&self) -> Self {
                    if *self == <$t as RingBase>::zero() {
                        panic!("Division by zero");
                    }
                    <$t as RingBase>::one() / *self
                }
            }
        )*
    };
}

impl_field_for_primitives!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64
);

/// Complete Field trait with arithmetic
pub trait Field: FieldBase + Ring {
    // Design Decision:
    // Reference arithmetic bounds were removed to avoid recursion cycles.
}

impl<T> Field for T where T: FieldBase + Ring + Add<Output = Self> + Mul<Output = Self> {}

/// Finite Fields
pub mod finite_field;
