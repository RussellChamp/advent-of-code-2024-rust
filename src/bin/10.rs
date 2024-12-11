use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
#[derive(PartialEq)]
struct Path {
    row: isize,
    col: isize,
    elevation: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    // build the grid
    let grid = input.lines().map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap() ).collect_vec()
    ).collect_vec();

    let peaks = grid.iter().enumerate().flat_map(|(row_idx, row)|
        row.iter().enumerate().filter_map(move |(col_idx, c)|
            if *c == 9 { Some(Path { row: row_idx as isize, col: col_idx as isize, elevation: 9 }) } else { None }
        )
    ).collect_vec();

    // find the trailheads
    let trailheads = grid.iter().enumerate().flat_map(|(row_idx, row)|
        row.iter().enumerate().filter_map(move |(col_idx, c)|
            if *c == 0 { Some(Path { row: row_idx as isize, col: col_idx as isize, elevation: 0 }) } else { None }
        )
    ).collect_vec();


    let total_score = trailheads.iter().cartesian_product(&peaks).filter_map(|(trail, peak)| {
        if can_navigate(&grid, trail, peak) { Some(true) } else { None }
    }).count();

    Some(total_score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

const DIRECTIONS: &[(isize, isize); 4] = &[(-1,0), (1,0), (0,-1), (0,1)]; // UP, DOWN, LEFT, RIGHT

fn can_navigate(grid: &Vec<Vec<u32>>, start: &Path, end: &Path) -> bool {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let is_valid = |p: &Path| (0..grid_height as isize).contains(&p.row) && (0..grid_width as isize).contains(&p.col) && grid[p.row as usize][p.col as usize] == p.elevation;

    if start == end { return true }

    DIRECTIONS.iter().any(|d| {
        let next_step = Path { row: &start.row + d.0, col: &start.col + d.1, elevation: start.elevation + 1 };
        // println!("{:?} ==> {:?} ?", start, end);
        is_valid(&next_step) && can_navigate(&grid, &next_step, &end)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
