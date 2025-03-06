fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=10);
    println!("Guess the number:");
    loop {
        let guess = rng.gen_range(1..=10);
        if guess == secret_number {
            println!("Correct!");
            break;
        } else if guess > secret_number {
            println!("Too high.");
        } else {
            println!("Too low.");
        }
    }
}
