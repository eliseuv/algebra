//! Magma Algebra
//! A magma (S, *) is a set S with a single binary operation *: S x S -> S.
//! https://ncatlab.org/nlab/show/magma

use std::ops::{Add, Sub};

pub trait Operation {}

pub trait Commutative: Operation {}

pub trait Associative: Operation {}

pub struct Addition;

impl Operation for Addition {}
impl Commutative for Addition {}
impl Associative for Addition {}

pub struct Subtraction;

impl Operation for Subtraction {}

/// Magma (S, *)
/// The magma trait is implemented by a type S representing the set and is generic over the operation *.
pub trait Magma<Op: Operation> {
    fn op(self, other: Self) -> Self;
}

impl<T> Magma<Addition> for T
where
    T: Add<Output = T>,
{
    fn op(self, other: Self) -> Self {
        self + other
    }
}

impl<T> Magma<Subtraction> for T
where
    T: Sub<Output = T>,
{
    fn op(self, other: Self) -> Self {
        self - other
    }
}
