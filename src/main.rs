// use rand::seq::SliceRandom;  // For shuffling or picking random elements from slices
// use rand::thread_rng;         // Provides thread-local random number generator

// fn get_random_number_from_list(list: &[i32]) -> Option<i32> {
//     let mut rng = thread_rng();  // Create a random number generator
//     list.choose(&mut rng).copied()  // Pick a random number and return it, wrapped in Option
// }
use clap::Parser; // Import the Parser trait from clap

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Sum two numbers")]
struct Args {
    num1: i64,
    num2: i64,
}

fn multiply(x: i64, y: i64) -> i64 {
    x * y
}

fn main() {
    let args = Args::parse(); // Parse the command-line arguments

    let result = multiply(args.num1, args.num2);
    println!("The multiplication of num1 and num2: {}", result);
}
