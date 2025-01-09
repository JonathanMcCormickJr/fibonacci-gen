// fibonacci.rs

// Fibonacci Number Generator

use std::io;

fn main() {

    // Get the quantity of numbers to generate
    let mut input_string: String = String::new();
    println!("Enter the number of terms you want to generate: ");
    io::stdin().read_line(&mut input_string).expect("Failed to read input");

    // Convert the input to an integer
    let quantity_to_produce: u32 = input_string.trim().parse::<u32>().expect("Failed to convert input to integer");
    
    // Generate the Fibonacci numbers
    let sequence: Vec<u32> = Vec::new();

    let generate_next_fibonacci_num(current_num: u32) -> u32 {
        while sequence.len() < quantity_to_produce {
            match current_num {
                0 | 1 => 1,
                _ => sequence[-1] + sequence[-2]
    
            }
            sequence.push(generate_next_fibonacci_num(sequence.len()));
        }
        
    }


}