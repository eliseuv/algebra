//! Ring Algebra
//! https://ncatlab.org/nlab/show/ring
//! A ring (S, +, *) is a set S with two binary operations + and * that satisfy the following properties:
//! 1. (S, +) is an abelian group
//! 2. (S, *) is a monoid
//! 3. Multiplication is distributive over addition 

use std::ops::{Add, Mul};

/// Base trait for Ring Algebra
/// Used to avoid rewriting HRTBs on every impl
pub trait RingBase: Sized + Clone + PartialEq {
    /// Zero element
    fn zero() -> Self;

    /// Unit element
    fn one() -> Self;
}

macro_rules! impl_ring_for_primitives {
    ($($t:ty),*) => {
        $(
            impl RingBase for $t {
                #[inline(always)]
                fn zero() -> Self { 0 as $t }

                #[inline(always)]
                fn one() -> Self { 1 as $t }
            }
        )*
    };
}

impl_ring_for_primitives!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64);

/// Complete Ring trait with arithmetic
pub trait Ring:
    RingBase 
    // Owned arithmetic
    + Add<Output = Self>
    + Mul<Output = Self>
{
    // Design Decision:
    // Reference arithmetic bounds (for<'a> &'a Self: Add<&'a Self, Output = Self>, etc.)
    // were removed from this trait definition to avoid infinite recursion cycles in the compiler
    // when implementing this trait for types like Polynomial<T> where T: Ring.
    //
    // If we require reference arithmetic here, `impl<T: Ring> Add for &Polynomial<T>` would
    // depend on `T: Ring`, which depends on `&T: Add`, which might depend on `T` being a Polynomial...
    // creating a cycle if Polynomial itself implements Ring.
    //
    // Instead, we implement the reference arithmetic explicitly on types that need it,
    // and keep this trait focused on the core owned arithmetic and identity elements.
}

impl<T> Ring for T
    where
    T: RingBase
    + Add<Output = Self>
    + Mul<Output = Self>,
{}