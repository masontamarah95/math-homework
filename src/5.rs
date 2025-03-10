use std::fmt;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(1..=100);
    println!("{}", random_num);
}
