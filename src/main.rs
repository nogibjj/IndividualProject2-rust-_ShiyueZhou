use rand::seq::SliceRandom;  // For shuffling or picking random elements from slices
use rand::thread_rng;         // Provides thread-local random number generator

fn get_random_number_from_list(list: &[i32]) -> Option<i32> {
    let mut rng = thread_rng();  // Create a random number generator
    list.choose(&mut rng).copied()  // Pick a random number and return it, wrapped in Option
}

fn sum(x: i64, y: i64) -> i64 {
    x + y
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];  // Example list of numbers
    if let Some(random_number) = get_random_number_from_list(&numbers) {
        if let Some(random_number2) = get_random_number_from_list(&numbers) {
            let result = sum(random_number as i64, random_number2 as i64);
            println!("The sum of x and y is: {}", result);
        }
    } else {
        println!("The list is empty!");
    }
}
