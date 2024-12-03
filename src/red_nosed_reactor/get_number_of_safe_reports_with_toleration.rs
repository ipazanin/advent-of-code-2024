pub fn get_number_of_safe_reports_with_toleration(data: Vec<Vec<i32>>) -> i32 {
    let mut number_of_safe_reports = 0;

    for report in data {
        if is_report_safe_with_toleration(report) {
            number_of_safe_reports += 1;
        }
    }

    number_of_safe_reports
}

fn is_report_safe_with_toleration(report: Vec<i32>) -> bool {
    let is_whole_report_safe = is_report_safe(&report);

    if is_whole_report_safe {
        return true;
    }

    for index in 0..report.len() {
        let left = &report[..index];
        let right = &report[index + 1..];
        let joined = left.iter().chain(right.iter()).cloned().collect();

        println!("{:?}", joined);
        let is_slice_safe = is_report_safe(&joined);

        if is_slice_safe {
            return true;
        }
    }

    false
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut prev_number = report[0];

    let is_ascending = report[1] > prev_number;

    for index in 1..report.len() {
        let number = report[index];

        let is_valid = is_number_valid(prev_number, number, is_ascending);

        if !is_valid {
            return false;
        }

        prev_number = number;
    }

    true
}

fn is_number_valid(prev_number: i32, number: i32, is_ascending: bool) -> bool {
    let diff = if is_ascending {
        number - prev_number
    } else {
        prev_number - number
    };

    let is_valid = validate_step_size(diff);

    return is_valid;
}

fn validate_step_size(step: i32) -> bool {
    match step {
        1 => true,
        2 => true,
        3 => true,
        _ => false,
    }
}
