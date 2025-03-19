use std::collections::HashSet;
use rand::Rng;

fn get_random_math_problem(size: usize) -> (String, String) {
    let mut rng = rand::thread_rng();
    let numbers: HashSet<i32> = (0..size).map(|_| rng.gen_range(1, 10)).collect();
    let num1 = *numbers.iter().next().unwrap();
    let num2 = *numbers.iter().skip(1).next().unwrap();

    return (format!("{} + {}", num1, num2), format!("{}", num1 + num2));
}
