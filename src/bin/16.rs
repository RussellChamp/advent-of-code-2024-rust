use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug)]
#[derive(PartialEq)]
enum TileObject {
    Start,
    End,
    Empty,
    Wall
}

fn get_tile_object(c: char) -> TileObject {
    match c {
        '.' => TileObject::Empty,
        '#' => TileObject::Wall,
        'S' => TileObject::Start,
        'E' => TileObject::End,
        _ => panic!()
    }
}

struct Cost {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|l| {
        l.chars().map(get_tile_object).collect_vec()
    }).collect_vec();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let start_pos = grid.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, t)| if *t == TileObject::Start { Some((row_idx, col_idx)) } else { None })
    }).unwrap();

    let mut score_card: Vec<Vec<Option<u32>>> = vec![vec![None; grid_width]; grid_height];

    // From the starting position facing EAST
    score_card[start_pos.0][start_pos.1] = Some(0);
    explore(&grid, &mut score_card, start_pos, Cost { up: 1001, down: 1001, left: u32::MAX, right: 1 });

    let end_pos = grid.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, t)| if *t == TileObject::End { Some((row_idx, col_idx)) } else { None })
    }).unwrap();

    let end_score = score_card[end_pos.0][end_pos.1].unwrap_or(0);
    Some(end_score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn explore(grid: &Vec<Vec<TileObject>>, score_card: &mut Vec<Vec<Option<u32>>>, pos: (usize, usize), move_cost: Cost) {
    if grid[pos.0][pos.1] == TileObject::Wall { return; }

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let pos_score = score_card[pos.0][pos.1].unwrap();

    let get_step = |row_idx, col_idx| -> Option<(usize, usize)> {
        if !(0..grid_height as isize).contains(&row_idx) || !(0..grid_width as isize).contains(&col_idx) || grid[row_idx as usize][col_idx as usize] == TileObject::Wall {
            None
        } else {
            Some((row_idx as usize, col_idx as usize))
        }
    };

    // UP
    if let Some(step) = get_step(pos.0 as isize - 1, pos.1 as isize) {
        let next_cost = score_card[step.0][step.1];
        if move_cost.up != u32::MAX && (Option::is_none(&next_cost) || next_cost.unwrap() > pos_score + move_cost.up) {
            score_card[step.0][step.1] = Some(pos_score + move_cost.up);
            explore(grid, score_card, step, Cost { up: 1, down: u32::MAX, left: 1001, right: 1001 });
        }
    }

    // DOWN
    if let Some(step) = get_step(pos.0 as isize + 1, pos.1 as isize) {
        let next_cost = score_card[step.0][step.1];
        if move_cost.down != u32::MAX && (Option::is_none(&next_cost) || next_cost.unwrap() > pos_score + move_cost.down) {
            score_card[step.0][step.1] = Some(pos_score + move_cost.down);
            explore(grid, score_card, step, Cost { up: u32::MAX, down: 1, left: 1001, right: 1001 });
        }
    }

    // LEFT
    if let Some(step) = get_step(pos.0 as isize, pos.1 as isize - 1) {
        let next_cost = score_card[step.0][step.1];
        if move_cost.left != u32::MAX && (Option::is_none(&next_cost) || next_cost.unwrap() > pos_score + move_cost.left) {
            score_card[step.0][step.1] = Some(pos_score + move_cost.left);
            explore(grid, score_card, step, Cost { up: 1001, down: 1001, left: 1, right: u32::MAX });
        }
    }

    // RIGHT
    if let Some(step) = get_step(pos.0 as isize, pos.1 as isize + 1) {
        let next_cost = score_card[step.0][step.1];
        if move_cost.right != u32::MAX && (Option::is_none(&next_cost) || next_cost.unwrap() > pos_score + move_cost.right) {
            score_card[step.0][step.1] = Some(pos_score + move_cost.right);
            explore(grid, score_card, step, Cost { up: 1001, down: 1001, left: u32::MAX, right: 1 });
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
