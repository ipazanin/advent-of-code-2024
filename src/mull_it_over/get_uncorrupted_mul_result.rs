const FORMAT: [char; 8] = ['m', 'u', 'l', '(', '*', ',', '*', ')'];

pub fn get_uncorrupted_mul_result(data: String) -> i32 {
    let mut result = 0;
    let mut format_pointer = 0;
    let mut first_number_characters = String::new();
    let mut second_number_characters = String::new();

    for character in data.chars() {
        let format_character = FORMAT[format_pointer];

        if format_pointer == 4 {
            if character.is_numeric() {
                first_number_characters.push(character);
                continue;
            }

            if character == ',' && first_number_characters.len() > 0 {
                format_pointer += 2;
                continue;
            }
        }

        if format_pointer == 6 {
            if character.is_numeric() {
                second_number_characters.push(character);
                continue;
            }

            if character == ')' && second_number_characters.len() > 0 {
                let first_number: i32 = first_number_characters.parse().expect("Unreachable");
                let second_number: i32 = second_number_characters.parse().expect("Unreachable");

                result += first_number * second_number;

                format_pointer = 0;
                first_number_characters.clear();
                second_number_characters.clear();

                continue;
            }
        }

        if character == format_character {
            format_pointer += 1;
            continue;
        }

        format_pointer = 0;
        first_number_characters.clear();
        second_number_characters.clear();
    }

    result
}
