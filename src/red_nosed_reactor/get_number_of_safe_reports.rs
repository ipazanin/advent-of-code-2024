pub fn get_number_of_safe_reports(data: Vec<Vec<i32>>) -> i32 {
    let mut number_of_safe_reports = 0;

    for report in data {
        if is_report_safe(report) {
            number_of_safe_reports += 1;
        }
    }

    number_of_safe_reports
}

fn is_report_safe(report: Vec<i32>) -> bool {
    let mut prev_number = report[0];

    let is_ascending = report[1] > prev_number;

    for index in 1..report.len() {
        let number = report[index];

        if prev_number == number {
            return false;
        }

        let diff = if is_ascending {
            number - prev_number
        } else {
            prev_number - number
        };

        let is_valid = validate_step_size(diff);

        if !is_valid {
            return false;
        }

        prev_number = number;
    }

    true
}

fn validate_step_size(step: i32) -> bool {
    match step {
        1 => true,
        2 => true,
        3 => true,
        _ => false,
    }
}
