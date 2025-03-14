fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=10);
    let b = rng.gen_range(1..=10);
    println!("The sum of {} and {} is {}", a, b, a + b);
}
