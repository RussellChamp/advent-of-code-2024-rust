advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let valid_reports = input.split('\n').fold(0, |total, line| {
        if line.is_empty() { return total };

        let values: Vec<_> = line.split(" ").map(|f| f.parse::<u32>().unwrap()).collect();
        let is_valid = is_valid_report(&values);
        // println!("Report {} {} valid", line, if is_valid { "is" } else { "is NOT" });
        if is_valid { total + 1 } else { total }
    });

    Some(valid_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let valid_reports = input.split('\n').fold(0, |total, line| {
        if line.is_empty() { return total };

        let values: Vec<_> = line.split(" ").map(|f| f.parse::<u32>().unwrap()).collect();

        let is_valid = is_any_valid_report(&values);
        // println!("Report {} {} valid", line, if is_valid { "is" } else { "is NOT" });
        if is_valid { total + 1 } else { total }
    });

    Some(valid_reports)
}

enum Direction {
    Unknown,
    Increasing,
    Decreasing,
}

fn is_valid_report(values: &[u32]) -> bool {
    let mut direction = Direction::Unknown;
    // println!("Checking values {:?}", values);
    let mut last_report = *values.first().unwrap();

    for r in &values[1..] {
        let report = *r;

        if matches!(direction, Direction::Unknown) {
            direction = if report > last_report {
                Direction::Increasing
            } else {
                Direction::Decreasing
            };
        }

        if report == last_report {
            return false;
        };
        if matches!(direction, Direction::Increasing)
            && (report < last_report || report > last_report + 3)
        {
            return false;
        };
        if matches!(direction, Direction::Decreasing)
            && (report > last_report || report + 3 < last_report)
        {
            return false;
        };

        last_report = report;
    }

    true
}

fn is_any_valid_report(values: &[u32]) -> bool {
    if is_valid_report(values) {
        return true;
    }
    let length = values.len();
    for idx in 0..length {
        if is_valid_report(&copy_except(values, idx)) {
            return true;
        }
    }

    false
}

fn copy_except(values: &[u32], exclude_idx: usize) -> Vec<u32> {
    let mut new_values: Vec<u32> = Vec::new();
    for (idx, value) in values.iter().enumerate() {
        if idx != exclude_idx {
            new_values.push(*value);
        }
    }
    // println!("copied new values {:?}", new_values);
    new_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
