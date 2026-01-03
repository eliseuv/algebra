use algebra::polynomial::lagrange::lagrange_interpolation;
use common::MyF64;

mod common;

#[test]
fn test_lagrange_basic() {
    let points = vec![(MyF64(0.0), MyF64(1.0)), (MyF64(1.0), MyF64(2.0))];
    let poly = lagrange_interpolation(&points);
    let val = poly.evaluate(&MyF64(2.0));
    assert!((val.0 - 3.0).abs() < 1e-6);
}

#[test]
fn test_lagrange_f64() {
    let points = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 5.0)]; // y = x^2 + 1
    let poly = lagrange_interpolation(&points);
    // P(3) = 10.
    let val = poly.evaluate(&3.0);
    assert!((val - 10.0_f64).abs() < 1e-6);
}
