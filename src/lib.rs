// Macro to generate reference arithmetic
macro_rules! impl_ref_ops {
    ($trait:ident, $method:ident) => {
        impl<'a, 'b, const P: u64> $trait<&'b Fp<P>> for &'a Fp<P> {
            type Output = Fp<P>;
            fn $method(self, other: &'b Fp<P>) -> Fp<P> {
                (*self).$method(*other)
            }
        }
    };
}

// Macro to generate assign arithmetic
macro_rules! impl_assign_ops {
    ($trait:ident, $assignMethod:ident, $method:ident) => {
        impl<'a, const P: u64> $trait<&'a Fp<P>> for Fp<P> {
            fn $assignMethod(&mut self, other: &'a Fp<P>) {
                *self = (*self).$method(*other)
            }
        }
        impl<const P: u64> $trait<Fp<P>> for Fp<P> {
            fn $assignMethod(&mut self, other: Fp<P>) {
                *self = (*self).$method(other)
            }
        }
    };
}

pub mod number;

pub mod magma;

pub mod ring;

pub mod field;

pub mod polynomial;
