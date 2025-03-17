fn main() {
    let num1 = 5;
    let num2 = 3;
    println!("{} + {} = {}", num1, num2, add(num1, num2));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
