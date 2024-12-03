use red_nosed_reactor::{
    get_data_from_file::get_data, get_number_of_safe_reports::get_number_of_safe_reports, get_number_of_safe_reports_with_toleration::get_number_of_safe_reports_with_toleration,
};

mod red_nosed_reactor;

fn main() {
    let data = get_data();
    let number_of_safe_reports = get_number_of_safe_reports_with_toleration(data);

    println!("{}", number_of_safe_reports);
}
