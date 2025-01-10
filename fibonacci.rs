// fibonacci.rs

// Fibonacci Number Generator

use std::io;

fn main() {
    // Get the quantity of numbers to generate
    println!("Enter the number of terms you want to generate: ");
    let quantity_to_produce: u32 = io::stdin()
        .lines()
        .next()
        .expect("No input received")
        .expect("Failed to read line")
        .parse()
        .expect("Failed to parse number");
    // Generate the Fibonacci numbers
    let sequence: Vec<u32> = Vec::new();

    let generate_next_fibonacci_num(current_num: u32) -> u32 {

        if sequence.len() == 0 {
            sequence.push(0);
        } else if sequence.len() == 1 | sequence.len() == 2 {
            sequence.push(1);
        } else {
            sequence.push(sequence[-1] + sequence[-2]);
        }

        if sequence.len() < quantity_to_produce {
            generate_next_fibonacci_num(sequence[-1]);
        } 
    }

    generate_next_fibonacci_num(quantity_to_produce)

}