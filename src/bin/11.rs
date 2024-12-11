use std::ops::{Div, Mul};

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = input.strip_suffix("\n")?.split(" ").collect_vec();

    for step in 0..25 {
        println!("STEP {step}: {:?}", stones);
        // stones.clone_from(&process_stones(stones).iter().map(|s| s as &str).collect_vec());
        // stones = next_step.clone().iter().map(|s| s as &str).collect_vec();
        for idx in (0..stones.len()).rev() {
            let stone = stones[idx];
            let stone_length = stone.len();
            if stone == "0" {
                stones[idx] = "1";
            } else if stone_length % 2 == 0 {
                let stone_iter = stone.chars().into_iter();
                // THIS IS BROKEN AND I"M VERY SAD
                let first_half = stone_iter.by_ref().take(stone_length/2).collect::<String>().as_str();

                let second_half = stone.chars().into_iter().skip(stone_length/2).take(stone_length/2).collect::<String>().as_str();
                stones[idx] = first_half;
                stones.insert(idx + 1, second_half);
            } else {
                stones[idx] = stone.parse::<usize>().unwrap().mul(2024).to_string().as_str();
            }
        }
    }

    println!("FINAL STONES: {:?}", stones);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
