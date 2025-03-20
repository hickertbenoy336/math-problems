use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number1 = rng.gen_range(0..10);
    let number2 = rng.gen_range(0..10);
    let sum = number1 + number2;

    println!("What is {} plus {}?", number1, number2);
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).unwrap();

    match guess.trim().parse::<i32>() {
        Ok(num) => {
            if num == sum {
                println!("You got it right!");
            } else {
                println!("Sorry, you lost");
            }
        },
        Err(_) => {
            println!("Invalid input, please try again");
        }
    }
}
