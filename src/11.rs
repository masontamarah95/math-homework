use std::cmp::{max, min};
use std::f64;

fn main() {
    let x = 2.0;
    let y = -3.0;
    println!("The maximum of {} and {} is {}", x, y, max(x, y));
    println!("The minimum of {} and {} is {}", x, y, min(x, y));
}
