use std::{env, fs};

const FILE_PATH: &str = "Projects/personal/advent-of-code-2024/src/mull_it_over/data.txt";

pub fn get_data() -> String {

    let binding = env::home_dir().expect("User path is not defined");
    let home_path = binding.as_path();
    let full_file_path = home_path.join(FILE_PATH);

    let contents = fs::read_to_string(full_file_path).expect("File does not exist");

    contents
}
