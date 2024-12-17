use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Object,
    Guard
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0_isize, 0_isize);
    let mut dir = (-1_isize, 0_isize);

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
            let directions = [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];
            let new_dir = *directions.iter().skip_while(|d| **d != dir).nth(1).unwrap();
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
    let grid = input.lines().map(|line| line.chars().map(|c| {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Object,
            '^' => Tile::Guard,
            _ => panic!()
        }
    }).collect_vec()).collect_vec();

    let mut looper_count = 0;
    // find all the spots the guard normally follows
    // then see if placing an object on one of those spots creates a loop

    let paths =  walk_path(&grid);
    // println!("CHECKING {:?} PATHS", paths.len());

    for (row_idx, col_idx) in paths {
        if grid[row_idx][col_idx] != Tile::Empty { continue }
        let mut test_grid = grid.clone();
        test_grid[row_idx][col_idx] = Tile::Object; // Add a wall and check if it creates a loop

        if does_loop(&test_grid) {
            looper_count += 1;
            // print!("!");
            // let _ = io::stdout().flush();
        } else {
            // print!("x");
            // let _ = io::stdout().flush();
        }
    }

    Some(looper_count)
}

fn count_spaces(grid: Vec<Vec<i32>>) -> usize {
    grid.iter().flat_map(|line: &Vec<_>| line.iter().filter(|v| v > &&0)).count()
}

fn walk_path(grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut pos = grid.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, cell)| {
            if *cell == Tile::Guard { Some((row_idx as isize, col_idx as isize)) } else { None }
        })
    }).unwrap();
    let mut dir = (-1_isize, 0_isize);

    let mut history = HashSet::new();
    history.insert((pos.0 as usize, pos.1 as usize));

    let is_valid_position = |(row_idx, col_idx)| {
        (0..grid.len() as isize).contains(&row_idx) && (0..grid[0].len() as isize).contains(&col_idx)
    };

    loop {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !is_valid_position(new_pos) { break }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == Tile::Object {
            // if it is a blocker, turn 90deg
            let directions = [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];
            let new_dir = *directions.iter().skip_while(|d| **d != dir).nth(1).unwrap();
            dir.0 = new_dir.0;
            dir.1 = new_dir.1;
        } else {
            // else move into the new position and mark it as entered
            history.insert((new_pos.0 as usize, new_pos.1 as usize));
            pos.0 = new_pos.0;
            pos.1 = new_pos.1;
        }
    }

    history.into_iter().collect_vec()
}

fn does_loop(grid: &Vec<Vec<Tile>>) -> bool {
    let mut pos = grid.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, cell)| {
            if *cell == Tile::Guard { Some((row_idx as isize, col_idx as isize)) } else { None }
        })
    }).unwrap();
    let mut dir = (-1_isize, 0_isize);

    let mut history = vec![];
    history.push(((pos.0, pos.1), (-1_isize, 0_isize)));


    let is_valid_position = |(row_idx, col_idx)| {
        (0..grid.len() as isize).contains(&row_idx) && (0..grid[0].len() as isize).contains(&col_idx)
    };

    loop {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !is_valid_position(new_pos) {
            return false;
        }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == Tile::Object {
            // if it is a blocker, turn 90deg
            let directions = [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];
            let new_dir = *directions.iter().skip_while(|d| **d != dir).nth(1).unwrap();

            if history.contains(&(pos, new_dir)) {
                return true;
            }

            dir.0 = new_dir.0;
            dir.1 = new_dir.1;
        } else {
            // else move into the new position and mark it as entered
            if history.contains(&((new_pos.0, new_pos.1), dir)) {
                return true;
            }

            history.push(((new_pos.0, new_pos.1), dir));
            pos.0 = new_pos.0;
            pos.1 = new_pos.1;
        }
    }
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
