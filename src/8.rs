use std::f32;
use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 100
    let x: f32 = rng.gen_range(1.0..=100.0);
    println!("Random number: {}", x);
}
