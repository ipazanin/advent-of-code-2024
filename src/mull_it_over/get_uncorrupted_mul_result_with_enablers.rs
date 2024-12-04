const FORMAT: [char; 8] = ['m', 'u', 'l', '(', '*', ',', '*', ')'];
const DO: [char; 4] = ['d', 'o', '(', ')'];
const DONOT: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];

pub fn get_uncorrupted_mul_result_with_enablers(data: String) -> i32 {
    println!("{:?}", DONOT[3]);
    let mut result = 0;
    let mut format_pointer = 0;
    let mut do_pointer = 0;
    let mut dont_pointer = 0;
    let mut mul_enabled = true;
    let mut first_number_characters = String::new();
    let mut second_number_characters = String::new();

    for character in data.chars() {
        let mut current_mul_result = 0;

        if mul_enabled {
            (
                current_mul_result,
                format_pointer,
                first_number_characters,
                second_number_characters,
            ) = try_get_mul_result(
                character,
                format_pointer,
                first_number_characters,
                second_number_characters,
            );
        } else {
            format_pointer = 0;
            first_number_characters.clear();
            second_number_characters.clear();
        }

        (mul_enabled, do_pointer, dont_pointer) =
            get_mul_enabled(character, mul_enabled, do_pointer, dont_pointer);

        result += current_mul_result;
    }

    result
}

fn get_mul_enabled(
    character: char,
    mul_enabled: bool,
    mut do_pointer: usize,
    mut dont_pointer: usize,
) -> (bool, usize, usize) {

    let do_character = DO[do_pointer];
    let donot_character = DONOT[dont_pointer];

    if do_pointer == 3 && character == ')' {
        return (true, 0, 0);
    }

    if dont_pointer == 6 && character == ')' {
        return (false, 0, 0);
    }

    if character == do_character {
        do_pointer += 1;
    } else {
        do_pointer = 0;
    };

    if character == donot_character {
        dont_pointer += 1;
    } else {
        dont_pointer = 0;
    };

    return (mul_enabled, do_pointer, dont_pointer);
}

fn try_get_mul_result(
    character: char,
    mut format_pointer: usize,
    mut first_number_characters: String,
    mut second_number_characters: String,
) -> (i32, usize, String, String) {
    let format_character = FORMAT[format_pointer];

    if format_pointer == 4 {
        if character.is_numeric() {
            first_number_characters.push(character);
            return (
                0,
                format_pointer,
                first_number_characters,
                second_number_characters,
            );
        }

        if character == ',' && first_number_characters.len() > 0 {
            format_pointer += 2;
            return (
                0,
                format_pointer,
                first_number_characters,
                second_number_characters,
            );
        }
    }

    if format_pointer == 6 {
        if character.is_numeric() {
            second_number_characters.push(character);
            return (
                0,
                format_pointer,
                first_number_characters,
                second_number_characters,
            );
        }

        if character == ')' && second_number_characters.len() > 0 {
            let first_number: i32 = first_number_characters.parse().expect("Unreachable");
            let second_number: i32 = second_number_characters.parse().expect("Unreachable");

            let result = first_number * second_number;

            format_pointer = 0;
            first_number_characters.clear();
            second_number_characters.clear();

            return (
                result,
                format_pointer,
                first_number_characters,
                second_number_characters,
            );
        }
    }

    if character == format_character {
        format_pointer += 1;
        return (
            0,
            format_pointer,
            first_number_characters,
            second_number_characters,
        );
    }

    format_pointer = 0;
    first_number_characters.clear();
    second_number_characters.clear();

    return (
        0,
        format_pointer,
        first_number_characters,
        second_number_characters,
    );
}
