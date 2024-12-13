advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let total: usize = input.lines().filter_map(|line| {
        // println!("{line}");
        let m: Vec<&str> = line.split(": ").collect();
        let total = m[0].parse::<usize>().unwrap();
        let values: Vec<usize> = m[1].split(' ').map(|v| v.parse::<usize>().unwrap()).collect();

        // println!("LINE: {} = {:?}", total, values);

        if can_calc(total, &values) { Some(total) } else { None }
        // Some(total)
    }).sum();
    // println!("TOTAL: {}", total);

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

// allowed operators: + and *
// all possible options OP^(values count - 1)
// eg 190: 10 19 -- has TWO possible paths, 10 + 19 and 10 * 19
// eg 123: 4 5 6 3 -- has 8 possible paths; 4+5+6+3,  4+5+6*3, ... 4*5*6*3

fn can_calc(total: usize, values: &[usize]) -> bool {
    let is_valid = values.iter().fold(vec![0_usize], |subtotals, next_value| {
        process(&subtotals, next_value)
    }).iter().any(|t| *t == total);

    is_valid
}

fn process(sum: &[usize], next_value: &usize) -> Vec<usize> {
    sum.iter().flat_map(|v| {
        vec![v + next_value, v * next_value]
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let sum = [1];
        let next_value = 10;

        let result = process(&sum, &next_value);
        assert_eq!(result, vec![11, 10]);
    }

    #[test]
    fn test_can_calc() {
        let values = [1,2,3,4];
        let total = 10;

        let result = can_calc(total, &values);
        assert_eq!(result, true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
