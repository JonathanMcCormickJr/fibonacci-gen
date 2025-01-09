// fibonacci.rs

// Fibonacci Number Generator

use std::io;

fn main() {

    // Get the quantity of numbers to generate
    let mut input_string: String = String::new();
    println!("Enter the number of terms you want to generate: ");
    io::stdin().read_line(&mut input_string).expect("Failed to read input");

    // Convert the input to an integer
    let input: u32 = input_string.trim().parse::<u32>().expect("Failed to convert input to integer");
    




}