use algebra::{
    field::{finite_field::Fp, Field},
    polynomial::Polynomial,
};

fn sum<T: Field>(a: T, b: T) -> T {
    a + b
}

fn main() {
    // Define the Prime Modulus
    const P: u64 = 17;

    let x = sum(1, 2);
    println!("{}", x);

    // Create convenient alias
    type F17 = Fp<P>;

    let poly1 = Polynomial::from_coeffs(vec![F17::new(11), F17::new(0), F17::new(3)]);
    let poly2 = Polynomial::from_coeffs(vec![F17::new(13), F17::new(5)]);

    println!("{}", &poly1 * &poly2);
}
