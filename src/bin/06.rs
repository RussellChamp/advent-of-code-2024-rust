use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0 as isize, 0 as isize);
    let mut dir = (-1 as isize, 0 as isize);

    let grid = input.lines().enumerate().map(|(line_idx, line)| {
        let chars = line.chars().enumerate().map(|(char_idx, char)| {
            if char == '^' {
                pos.0 = line_idx as isize;
                pos.1 = char_idx as isize;
            }
            char
        });
        // println!("{:?}", chars);
        chars.collect_vec()
    }).collect_vec();

    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    visited[pos.0 as usize][pos.1 as usize] = 1;


    let is_valid_position = |(row_idx, col_idx)| {
        (0..grid.len() as isize).contains(&row_idx) && (0..grid[0].len() as isize).contains(&col_idx)
    };

    loop {
        // println!("At {:?} going {:?}", pos, dir);

        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !is_valid_position(new_pos) { break }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            // if it is a blocker, turn 90deg
            let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];
            let new_dir = *directions.iter().skip_while(|d| **d != dir).skip(1).next().unwrap();
            dir.0 = new_dir.0;
            dir.1 = new_dir.1;
        } else {
            // else move into the new position and mark it as entered
            visited[new_pos.0 as usize][new_pos.1 as usize] += 1;
            pos.0 = new_pos.0;
            pos.1 = new_pos.1;
        }
    }

    // println!("Made it out!");

    Some(count_spaces(visited) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn count_spaces(grid: Vec<Vec<i32>>) -> usize {
    grid.iter().flat_map(|line: &Vec<_>| line.iter().filter(|v| v > &&0)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_spaces() {
        let grid = vec![
            vec![1,2,3],
            vec![0,0,0],
            vec![1,0,0]
        ];
        let result = count_spaces(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
