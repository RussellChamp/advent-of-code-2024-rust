advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    let result = mul_regex.captures_iter(input).fold(0, |total, cap| {
        let (_, [a_str, b_str]) = cap.extract();

        let a = a_str.parse::<u32>().unwrap();
        let b = b_str.parse::<u32>().unwrap();
        let product = a * b;

        total + product
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"(do)\(\)()()|(don\'t)\(\)()()|(mul)\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    let (_, result) = mul_regex.captures_iter(input).fold((true, 0), |(enabled, total), cap| {
        let (_, [command, a_str, b_str]) = cap.extract();
        // println!("Captures: {command} ({a_str}, {b_str})");

        match command {
            "do" => (true, total),
            "don't" => (false, total),
            "mul" => {
                if !enabled { return (enabled, total) };

                let a = a_str.parse::<u32>().unwrap();
                let b = b_str.parse::<u32>().unwrap();
                let product = a * b;
                (enabled, total + product)
            },
            _ => panic!("Could not match command {command}"),
        }
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
