use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid_size: (u32, u32) = (input.lines().count() as u32, 0);

    let mut antennas: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    for (row_idx, line) in input.lines().enumerate() {
        if row_idx == 0 { grid_size.1 = line.chars().count() as u32; }

        for (col_idx, c) in line.chars().enumerate(){
            if c != '.' {
                if antennas.contains_key(&c) {
                    antennas.entry(c).and_modify(|v| v.push((row_idx as u32, col_idx as u32)));
                } else {
                    antennas.insert(c, vec![(row_idx as u32, col_idx as u32)]);
                }
            }
        }
    }

    // let locations = HashSet::new();

    let locations = antennas.iter().flat_map(|(_key, values)| {
        // println!("GROUP {} VALUE {:?}", _key, values);

        values.iter().permutations(2).flat_map(|points| {
            assert_eq!(points.len(), 2);
            get_antinodes(points[0], points[1], grid_size)
        }).unique().collect_vec()
    }).unique();

    // println!("ALL LOCATIONS: {:?}", locations.clone().collect_vec());

    Some(locations.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_antinodes(p1: &(u32, u32), p2: &(u32, u32), grid_size: (u32, u32)) -> Vec<(u32, u32)> {
    let diff = (p1.0 as isize - p2.0 as isize, p1.1 as isize - p2.1 as isize);

    let a1 = (p1.0 as isize + diff.0, p1.1 as isize + diff.1);
    let a2 = (p2.0 as isize - diff.0, p2.1 as isize - diff.1);

    let is_valid = |p: (isize, isize)| (0..grid_size.0 as isize).contains(&p.0) && (0..grid_size.1 as isize).contains(&p.1);

    let mut results = vec![];

    if is_valid(a1) { results.push((a1.0 as u32, a1.1 as u32)) };
    if is_valid(a2) { results.push((a2.0 as u32, a2.1 as u32)) };

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let p1 = (5, 6);
        let p2 = (8, 8);
        let grid_size = (12, 12);

        let result = get_antinodes(&p1, &p2, grid_size);
        assert_eq!(result, vec![(2, 4), (11, 10)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
