use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T
}

#[derive(Debug, Copy, Clone)]
struct Machine<T> {
    a_button: Point<T>,
    b_button: Point<T>,
    prize: Point<T>
}

pub fn part_one(input: &str) -> Option<u32> {
    let r_button = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let r_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let machines = input.lines().chunks(4).into_iter().map(|chunk| {
        let lines = chunk.collect_vec();
        assert!(lines.len() >= 3);

        let c_a = r_button.captures(lines[0]).unwrap();
        let a_button = Point { x: c_a[2].parse::<usize>().unwrap(), y: c_a[3].parse::<usize>().unwrap() };

        let c_b = r_button.captures(lines[1]).unwrap();
        let b_button = Point { x: c_b[2].parse::<usize>().unwrap(), y: c_b[3].parse::<usize>().unwrap() };

        let c_p = r_prize.captures(lines[2]).unwrap();
        let prize = Point { x: c_p[1].parse::<usize>().unwrap(), y: c_p[2].parse::<usize>().unwrap() };

        Machine { a_button, b_button, prize }
    }).collect_vec();

    // println!("MACHINES {:?}", machines);
    // BRUTE FORCE SOLUTION
    let total_tokens: usize = machines.iter().map(|machine| {
        let matches = find_matches(*machine).collect_vec();
        let min = matches.iter().map(|(x, y)| 3 * x + y).min().unwrap_or(0);
        // println!("MATCHES {:?}", matches);
        min
    }).sum();

    Some(total_tokens as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    const OFFSET: isize = 10000000000000;
    let r_button = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let r_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let machines = input.lines().chunks(4).into_iter().map(|chunk| {
        let lines = chunk.collect_vec();
        assert!(lines.len() >= 3);

        let c_a = r_button.captures(lines[0]).unwrap();
        let a_button = Point { x: c_a[2].parse::<isize>().unwrap(), y: c_a[3].parse::<isize>().unwrap() };

        let c_b = r_button.captures(lines[1]).unwrap();
        let b_button = Point { x: c_b[2].parse::<isize>().unwrap(), y: c_b[3].parse::<isize>().unwrap() };

        let c_p = r_prize.captures(lines[2]).unwrap();
        let prize = Point { x: c_p[1].parse::<isize>().unwrap() + OFFSET, y: c_p[2].parse::<isize>().unwrap() + OFFSET };

        Machine { a_button, b_button, prize }
    }).collect_vec();

    // println!("PROCESSING {} MACHINES", machines.len());
    let wins = machines.iter().filter_map(|Machine { a_button, b_button, prize }| {

        let denom = (a_button.x * b_button.y) - (a_button.y * b_button.x );
        let a_numer = (prize.x * b_button.y) - (prize.y * b_button.x);
        let b_numer = (prize.y * a_button.x) - (prize.x * a_button.y);

        if a_numer % denom != 0 || b_numer % denom != 0 {
            // print!("❌");
            return None;
        }

        let a_presses = a_numer / denom;
        let b_presses = b_numer / denom;

        // print!("✔️ ");
        Some(3 * a_presses + b_presses)
        // println!("MATCHES {:?}", matches);
    }).collect_vec();

    // println!();
    // println!("WON {} PRIZES!", wins.len());

    let total_tokens: isize = wins.into_iter().sum();
    // println!("COST {} TOKENS!", total_tokens);
    Some(total_tokens as u64)
}

// TODO figure out how to reference-ify this
fn find_matches(machine: Machine<usize>) -> impl Iterator<Item=(usize, usize)> {
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
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3440247820));
    }
}
