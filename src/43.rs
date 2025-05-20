fn main() {
    // Define a function to add two numbers
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Call the function with two integer parameters
    let result = add(5, 7);

    println!("The sum of 5 and 7 is {}", result);
}
