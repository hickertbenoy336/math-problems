use std::cmp;

fn main() {
    let mut nums: Vec<i32> = vec![];
    
    loop {
        println!("Enter two numbers:");

        match read_input() {
            Ok((first_num, second_num)) => {
                if first_num > second_num {
                    let temp = first_num;
                    first_num = second_num;
                    second_num = temp
                }

                nums.push(first_num);
                nums.push(second_num);

                println!("Numbers: {:?}", nums);
                continue;
            }
            Err(e) => return,
        }
    }
}

fn read_input() -> Result<(i32, i32), &'static str> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if nums.len() != 2 {
        return Err("Invalid number of inputs".into());
    }

    Ok((nums[0], nums[1]))
}
