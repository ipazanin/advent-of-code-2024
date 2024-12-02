use std::{env, fs};

const FILE_PATH: &str = "Projects/personal/advent-of-code-2024/src/historian_hysteria/data.txt";

pub fn get_data_from_file() -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    let binding = env::home_dir().expect("User path is not defined");
    let home_path = binding.as_path();
    let full_filePath = home_path.join(FILE_PATH);

    let contents = fs::read_to_string(full_filePath).expect("File does not exist");

    for line in contents.lines() {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();

        if splitted_line.len() != 2 {
            panic!("There should always be exactly 2 numbers")
        }

        let left_number: i32 = splitted_line[0].parse().expect("Invalid number");
        let right_number: i32 = splitted_line[1].parse().expect("Invalid number");

        left_numbers.push(left_number);
        right_numbers.push(right_number);
    }

    (left_numbers, right_numbers)
}
