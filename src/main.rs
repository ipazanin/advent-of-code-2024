use mull_it_over::{get_data_from_file::get_data, get_uncorrupted_mul_result::get_uncorrupted_mul_result};

mod mull_it_over;

fn main() {
    let data = get_data();
    let result = get_uncorrupted_mul_result(data);

    println!("{}", result);
}
