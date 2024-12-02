use red_nosed_reactor::{
    get_data_from_file::get_data, get_number_of_safe_reports::get_number_of_safe_reports,
};

mod red_nosed_reactor;

fn main() {
    let data = get_data();
    let number_of_safe_reports = get_number_of_safe_reports(data);

    println!("{}", number_of_safe_reports);
}
