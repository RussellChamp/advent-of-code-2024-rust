use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T
}

#[derive(Debug, Copy, Clone)]
struct Machine {
    a_button: Point<u32>,
    b_button: Point<u32>,
    prize: Point<u32>
}

pub fn part_one(input: &str) -> Option<u32> {
    let r_button = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let r_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let machines = input.lines().chunks(4).into_iter().map(|chunk| {
        let lines = chunk.collect_vec();
        assert!(lines.len() >= 3);

        let c_a = r_button.captures(lines[0]).unwrap();
        let a_button = Point { x: c_a[2].parse::<u32>().unwrap(), y: c_a[3].parse::<u32>().unwrap() };

        let c_b = r_button.captures(lines[1]).unwrap();
        let b_button = Point { x: c_b[2].parse::<u32>().unwrap(), y: c_b[3].parse::<u32>().unwrap() };

        let c_p = r_prize.captures(lines[2]).unwrap();
        let prize = Point { x: c_p[1].parse::<u32>().unwrap(), y: c_p[2].parse::<u32>().unwrap() };

        Machine { a_button, b_button, prize }
    }).collect_vec();

    // println!("MACHINES {:?}", machines);
    // BRUTE FORCE SOLUTION
    let total_tokens: u32 = machines.iter().map(|machine| {
        let matches = find_matches(*machine).collect_vec();
        let min = matches.iter().map(|(x, y)| 3 * x + y).min().unwrap_or(0);
        // println!("MATCHES {:?}", matches);
        min
    }).sum();

    Some(total_tokens)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

// TODO figure out how to reference-ify this
fn find_matches(machine: Machine) -> impl Iterator<Item=(u32, u32)> {
    (0..=100).flat_map(move |a_presses| {
        (0..=100).filter_map(move |b_presses| {
            let pos_x = machine.a_button.x * a_presses + machine.b_button.x * b_presses;
            let pos_y = machine.a_button.y * a_presses + machine.b_button.y * b_presses;
            if pos_x == machine.prize.x && pos_y == machine.prize.y {
                // println!("FOUND POINT! ({}, {})", pos_x, pos_y);
                Some((a_presses, b_presses))
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
