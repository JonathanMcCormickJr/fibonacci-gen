// src/main.rs

#![forbid(unsafe_code)]

fn main() {
    println!("{:?}", generate_nums(10));
}


pub fn generate_nums(quantity: u32) -> Vec<u32> {
    let mut additional_iterations_needed = quantity;
    let mut sequence: Vec<u32> = Vec::new();

    loop {
        if additional_iterations_needed < 1 {
            break
        }

        if sequence.len() == 0 {
            sequence.push(0);
        } else if sequence[sequence.len() - 1] == 0 || sequence[sequence.len() - 1] == 1 && sequence[sequence.len() - 2] == 0 {
            sequence.push(1);
        } else {
            sequence.push(sequence[sequence.len() - 1] + sequence[sequence.len() - 2])
        }

        additional_iterations_needed -= 1;
    };

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_nums() {
        let example0 = generate_nums(0);
        assert_eq!(example0, Vec::<u32>::new())
    }
}