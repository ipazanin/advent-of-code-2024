mod historian_hysteria;

use historian_hysteria::{get_data_from_file::get_data_from_file, get_distance::get_distance};

fn main() {
    let (left, right) = get_data_from_file();
    let distance = get_distance(left, right);
    println!("{}", distance);
}
