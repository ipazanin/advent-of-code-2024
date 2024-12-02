mod historian_hysteria;

use historian_hysteria::{get_data_from_file::get_data_from_file, get_distance::get_distance, get_similarity_score::get_similarity_score};

fn main() {
    let (left, right) = get_data_from_file();
    let similarity_score = get_similarity_score(&left, &right);
    println!("{}", similarity_score);
    
    let distance = get_distance(left, right);
    println!("{}", distance);

}
