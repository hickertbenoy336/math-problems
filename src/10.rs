use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::ops::Add;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 10
    let num1: i32 = rng.gen_range(1, 11);

    // Generate a random number between 1 and 10
    let num2: i32 = rng.gen_range(1, 11);

    println!("What is {} + {}?", num1, num2);

    // Read the answer from the user
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    // Parse the answer as a number
    let answer: i32 = answer.trim().parse().unwrap();

    // Check if the answer is correct
    if answer == num1 + num2 {
        println!("Correct!");
    } else {
        println!("Incorrect, the answer is {}", num1 + num2);
    }
}
