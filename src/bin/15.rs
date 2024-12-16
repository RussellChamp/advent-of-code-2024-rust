use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug)]
#[derive(PartialEq)]
enum TileObject {
    Empty,
    Wall,
    Robot,
    Box
}

fn get_tile_object(c: char) -> TileObject {
    match c {
        '.' => TileObject::Empty,
        '#' => TileObject::Wall,
        '@' => TileObject::Robot,
        'O' => TileObject::Box,
        _ => panic!()
    }
}

fn get_direction(c: char) -> (isize, isize) {
    match c {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut i = input.lines();
    let mut grid = i.by_ref().take_while(|l| !l.is_empty()).map(|l| l.chars().map(get_tile_object).collect_vec()).collect_vec();

    let moves = i.by_ref().flat_map(|l| {
        l.chars().map(get_direction).collect_vec()
    });

    let mut robot_pos = grid.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, t)| if *t == TileObject::Robot { Some((row_idx, col_idx)) } else { None })
    }).unwrap();

    grid[robot_pos.0][robot_pos.1] = TileObject::Empty;

    // println!("GRID ({}, {})", grid_height, grid_width);
    // println!("ROBOT ({}, {})", robot_pos.0, robot_pos.1);

    for m in moves {
        let new_row = (robot_pos.0 as isize + m.0) as usize;
        let new_col = (robot_pos.1 as isize + m.1) as usize;
        let next_pos = &grid[new_row][new_col];

        match next_pos {
            TileObject::Empty => {
                robot_pos = (new_row, new_col);
                // print!("M");
            },
            TileObject:: Box => {
                if let Some(next_open_space) = get_next_open_space(&grid, robot_pos, m) {
                    grid[new_row][new_col] = TileObject::Empty;
                    grid[next_open_space.0][next_open_space.1] = TileObject::Box;

                    robot_pos = (new_row, new_col);
                    // print!("~");
                } else {
                    // print!("!");
                }
            },
            _ => {
                // print!("!");
            }
        }
    }

    let calc_score = |row_idx, col_idx| 100 * row_idx + col_idx;

    let gps_score = grid.iter().enumerate().fold(0, |total, (row_idx, row)| {
        total + row.iter().enumerate().fold(0, |sub_total, (col_idx, tile)| {
            sub_total + if *tile == TileObject::Box { calc_score(row_idx, col_idx) } else { 0 }
        })
    });


    Some(gps_score as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn get_next_open_space(grid: &[Vec<TileObject>], pos: (usize, usize), m: (isize, isize)) -> Option<(usize, usize)> {
    let new_row = (pos.0 as isize + m.0) as usize;
    let new_col = (pos.1 as isize + m.1) as usize;
    let tile = &grid[new_row][new_col];

    match tile {
        TileObject::Empty => Some((new_row, new_col)),
        TileObject::Box => get_next_open_space(grid, (new_row, new_col), m),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2028));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
