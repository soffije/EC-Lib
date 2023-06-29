extern crate p256;
extern crate rand;

use p256::ProjectivePoint;
use rand::RngCore;
use p256::elliptic_curve::generic_array::typenum::{U32, U256};
use p256::elliptic_curve::FieldBytes;
use std::convert::TryInto;

// Structure for representing an EC point
pub struct ECPoint {
    pub point: ProjectivePoint,
}

// Function to generate a random projective point
fn generate_random_point() -> ProjectivePoint {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let scalar_bytes: &[u8; 32] = &bytes;
    let scalar = p256::Scalar::from_bytes_reduced((&scalar_bytes[..]).try_into().unwrap());
    ProjectivePoint::generator() * scalar
}

// Function to add two EC points
fn add_ec_points(a: &ECPoint, b: &ECPoint) -> ECPoint {
    ECPoint {
        point: a.point + b.point,
    }
}

// Function to double an EC point
fn double_ec_point(a: &ECPoint) -> ECPoint {
    ECPoint {
        point: a.point.double(),
    }
}

fn main() {
    let point_a = ECPoint {
        point: generate_random_point(),
    };
    let point_b = ECPoint {
        point: generate_random_point(),
    };

    let sum = add_ec_points(&point_a, &point_b);
    let doubled = double_ec_point(&point_a);

    println!("Sum: \t{:?}", sum.point);
    println!("\nDoubled: \t{:?}", doubled.point);
}
