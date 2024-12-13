use std::ops::Mul;

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = vecify_stones(input);
    step_stones(&mut stones, 25);

    let total_count = stones.len();
    // println!("FINAL STONES: {:?} stones", total_count);

    Some(total_count as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn step_stones(stones: &mut Vec<String>, max_steps: usize) {
    for _step in 0..max_steps {
        // println!("STEP #{step}: {:?} stones", stones.len());
        // stones.clone_from(&process_stones(stones).iter().map(|s| s as &str).collect_vec());
        // stones = next_step.clone().iter().map(|s| s as &str).collect_vec();
        for idx in (0..stones.len()).rev() {
            let stone = &stones[idx];
            let stone_length = stone.len();
            if stone == "0" {
                stones[idx] = String::from("1");
            } else if stone_length % 2 == 0 {
                let mut stone_iter = stone.chars();
                let first_half = stone_iter.by_ref().take(stone_length/2).collect::<String>();
                let second_half = stone_iter.by_ref().take(stone_length/2).collect::<String>().parse::<usize>().unwrap().to_string();

                stones[idx] = first_half;
                stones.insert(idx + 1, second_half);
            } else {
                let new_stone = stone.parse::<usize>().unwrap().mul(2024).to_string();
                stones[idx] = new_stone;
            }
        }
    }
}

fn vecify_stones(input: &str) -> Vec<String> {
    input.strip_suffix("\n").unwrap().split(" ").map(|s| s.to_string()).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_stones() {
        let mut stones= vecify_stones("125 17\n");

        step_stones(&mut stones, 1);
        assert_eq!(stones, vecify_stones("253000 1 7\n"));

        step_stones(&mut stones, 1);
        assert_eq!(stones, vecify_stones("253 0 2024 14168\n"));

        step_stones(&mut stones, 1);
        assert_eq!(stones, vecify_stones("512072 1 20 24 28676032\n"));

        step_stones(&mut stones, 1);
        assert_eq!(stones, vecify_stones("512 72 2024 2 0 2 4 2867 6032\n"));

        step_stones(&mut stones, 1);
        assert_eq!(stones, vecify_stones("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32\n"));

        step_stones(&mut stones, 1);
        assert_eq!(stones, vecify_stones("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2\n"));
    }

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
