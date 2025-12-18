use algebra::{field::finite_field::Fp, polynomial::Polynomial};

#[test]
fn test_polynomial_subtraction_smaller_minus_larger() {
    type F17 = Fp<17>;

    // p1 = 1 + x
    let p1 = Polynomial::from_coeffs(vec![F17::new(1), F17::new(1)]);
    // p2 = 2 + 2x + 2x^2
    let p2 = Polynomial::from_coeffs(vec![F17::new(2), F17::new(2), F17::new(2)]);

    // p1 - p2 = (1-2) + (1-2)x + (0-2)x^2 = -1 - x - 2x^2
    // modulo 17: 16 + 16x + 15x^2
    // Display: High degree first: 15x^2 + 16x + 16
    let result = &p1 - &p2;

    assert_eq!(
        result,
        Polynomial::from_coeffs(vec![F17::new(15), F17::new(16), F17::new(16)])
    );
}

#[test]
fn test_polynomial_subtraction_larger_minus_smaller() {
    type F17 = Fp<17>;
    // p1 = 2 + 2x + 2x^2
    let p1 = Polynomial::from_coeffs(vec![F17::new(2), F17::new(2), F17::new(2)]);
    // p2 = 1 + x
    let p2 = Polynomial::from_coeffs(vec![F17::new(1), F17::new(1)]);

    // p1 - p2 = 1 + x + 2x^2
    // Display: 2x^2 + 1x + 1
    let result = &p1 - &p2;
    assert_eq!(
        result,
        Polynomial::from_coeffs(vec![F17::new(2), F17::new(1), F17::new(1)])
    );
}
