use std::{env, fs};

const FILE_PATH: &str = "Projects/personal/advent-of-code-2024/src/red_nosed_reactor/data.txt";

pub fn get_data() -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = Vec::new();

    let binding = env::home_dir().expect("User path is not defined");
    let home_path = binding.as_path();
    let full_file_path = home_path.join(FILE_PATH);

    let contents = fs::read_to_string(full_file_path).expect("File does not exist");

    for line in contents.lines() {
        let mut line_vector = Vec::new();

        for number_string in line.split_whitespace() {
            let number: i32 = number_string.parse().expect("Invalid number");

            line_vector.push(number);
        }

        println!("{:?}", line_vector);

        data.push(line_vector);
    }

    data
}
