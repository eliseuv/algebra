//! Shamir Secret Sharing
//! Shamir's Secret Sharing is a method for secret sharing based on polynomial interpolation.

use std::fmt::Display;

use algebra::{
    field::finite_field::Fp,
    polynomial::{lagrange::lagrange_interpolation, Polynomial},
};
use rand::{seq::IteratorRandom, Rng, SeedableRng};

/// Share
#[derive(Debug, Clone, Copy)]
pub struct Share<T> {
    pub x: T,
    pub y: T,
}

impl<const P: u64> Display for Share<Fp<P>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}

impl<T> Share<T> {
    pub fn into_pair(self) -> (T, T) {
        (self.x, self.y)
    }
}

/// Split a secret into shares
pub fn split_secret<const P: u64, R: Rng + ?Sized>(
    secret: Fp<P>,
    share_threshold: usize,
    number_of_shares: usize,
    rng: &mut R,
) -> Vec<Share<Fp<P>>> {
    if share_threshold > number_of_shares {
        panic!("Share threshold must be less than the number of shares");
    }

    // Construct a random polynomial
    let poly = loop {
        let poly = Polynomial::from_coeffs(
            [secret]
                .into_iter()
                .chain((0..share_threshold - 1).map(|_| Fp::<P>::new(rng.random::<u64>())))
                .collect(),
        );
        if poly
            .degree()
            .expect("Since the secret is non-zero, the degree should be defined")
            == share_threshold - 1
        {
            break poly;
        }
    };

    // Generate shares
    let shares_abscissa = (1..=number_of_shares as u64).map(Fp::<P>::new);
    shares_abscissa
        .map(|x| Share {
            x,
            y: poly.evaluate(&x),
        })
        .collect()
}

/// Reconstruct a secret from shares
pub fn reconstruct_secret<const P: u64>(shares: &[Share<Fp<P>>]) -> Fp<P> {
    let poly = lagrange_interpolation(
        &shares
            .into_iter()
            .map(|share| share.into_pair())
            .collect::<Vec<_>>(),
    );
    *poly
        .constant_term()
        .expect("Since the degree is defined, the constant term should be defined")
}

fn main() {
    // Field order
    const PRIME: u64 = 2147483647;
    type Field = Fp<PRIME>;

    // Secret
    let secret = Field::new(123456789);
    println!("Secret: {secret}");

    // Set up RNG
    let mut rng = rand::rngs::StdRng::seed_from_u64(99);

    // Split secret
    let share_threshold = 3;
    let number_of_shares = 5;
    let shares = split_secret(secret, share_threshold, number_of_shares, &mut rng);
    println!("Share threshold: {share_threshold}");
    println!("Number of shares: {number_of_shares}");
    println!("Shares:");
    for share in shares.iter() {
        println!("{share}");
    }

    // Attempt to reconstruct the secret with too few shares
    let n_too_few = share_threshold - 1;
    println!("\nAttempting to reconstruct the secret with {n_too_few} < {share_threshold} shares. (Too few shares)");
    let shares_too_few = shares
        .iter()
        .choose_multiple(&mut rng, n_too_few)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    println!("Selected shares:");
    for share in shares_too_few.iter() {
        println!("{share}");
    }
    let secret_reconstructed = reconstruct_secret(&shares_too_few);
    println!("Reconstructed secret: {secret_reconstructed}");
    assert_ne!(secret, secret_reconstructed);

    // Select enough shares
    println!("\nAttempting to reconstruct the secret with {share_threshold} shares.");
    let shares = shares
        .iter()
        .choose_multiple(&mut rng, share_threshold)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    println!("Selected shares:");
    for share in shares.iter() {
        println!("{share}");
    }

    // Reconstruct the secret
    let poly = lagrange_interpolation(
        &shares
            .into_iter()
            .map(|share| share.into_pair())
            .collect::<Vec<_>>(),
    );
    let secret_reconstructed = poly.evaluate(&Field::new(0));
    println!("Reconstructed secret: {secret_reconstructed}");
    assert_eq!(secret, secret_reconstructed);
}
