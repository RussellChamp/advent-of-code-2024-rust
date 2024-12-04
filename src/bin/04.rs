advent_of_code::solution!(4);
use itertools::Itertools;
use tap::Pipe;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let row_size = grid.len() as isize;
    let col_size = grid[0].len() as isize;
    let is_valid = |row_idx, col_idx| (0..row_size).contains(&row_idx) && (0..col_size).contains(&col_idx);
    let get_value = |row_idx, col_idx| is_valid(row_idx, col_idx).then(|| grid[row_idx as usize][col_idx as usize]);

    let total: i32 = Itertools::cartesian_product(0..row_size, 0..col_size)
    .flat_map(|(row_idx, col_idx)| {
        [(0, -1), (-1, 0), (-1, -1), (-1, 1)]
        .into_iter()
        .map(move |(row_direction, col_direction)| {
            (0..=3)
            .filter_map(
                |magnitude| get_value(row_idx + row_direction * magnitude, col_idx + col_direction * magnitude)
            )
            .collect::<String>()
            .pipe(|s| ["XMAS", "SAMX"].contains(&s.as_ref()) as i32)
        })
    }).sum();

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let row_size = grid.len() as isize;
    let col_size = grid[0].len() as isize;
    let is_valid = |row_idx, col_idx| (0..row_size).contains(&row_idx) && (0..col_size).contains(&col_idx);
    let get_value = |row_idx, col_idx| is_valid(row_idx, col_idx).then(|| grid[row_idx as usize][col_idx as usize]);

    let total= Itertools::cartesian_product(0..row_size, 0..col_size)
    .filter_map(|(row_idx, col_idx)| {
        if get_value(row_idx, col_idx) != Some('A') { return None }

        let first = [(row_idx - 1, col_idx - 1), (row_idx, col_idx), (row_idx+1, col_idx+1)].iter().filter_map(|(row, col)| get_value(*row, *col)).collect::<String>();
        if !["MAS", "SAM"].contains(&first.as_ref()) { return None }

        let second = [(row_idx - 1, col_idx + 1), (row_idx, col_idx), (row_idx+1, col_idx-1)].iter().filter_map(|(row, col)| get_value(*row, *col)).collect::<String>();
        if !["MAS", "SAM"].contains(&second.as_ref()) { return None }

        Some(true)
    }).count();

    Some(total as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
