use itertools::Itertools;
use std::{collections::HashMap, fmt};

advent_of_code::solution!(12);

struct Plot {
    letter: char,
    region_id: Option<u32>,
}
impl fmt::Debug for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.letter, self.region_id.unwrap_or(0))
    }
}

#[derive(Debug)]
struct Region {
    edges: usize,
    count: usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut plots = input.lines().map(|l| l.chars().map(|c| Plot { letter: c, region_id: None }).collect_vec()).collect_vec();
    let mut regions: HashMap<u32, Region> = HashMap::new();
    let mut next_region_id = 1;

    while let Some(plot) = get_next_unassigned_plot(&plots) {
        plots[plot.0][plot.1].region_id = Some(next_region_id);

        let edge_count = count_edges(&plots, &plot);
        regions.insert(next_region_id, Region { edges: edge_count, count: 1 });
        next_region_id += 1;

        update_neighbors(&mut plots, &plot, &mut regions);
    }

    // println!("{:?}", plots);
    // println!("{:?}", regions);
    let total = regions.into_iter().fold(0, |sum, (_key, region)| {
        sum + (region.edges * region.count)
    });

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_next_unassigned_plot(plots: &Vec<Vec<Plot>>) -> Option<(usize, usize)> {
    for (row_idx, row) in plots.iter().enumerate() {
        for (col_idx, plot) in row.iter().enumerate() {
            if plot.region_id == None {
                // println!("FOUND unassigned plot! ({}, {})", row_idx, col_idx);
                return Some((row_idx, col_idx))
            }
        }
    }
    return None
}

fn update_neighbors(plots: &mut Vec<Vec<Plot>>, plot: &(usize, usize), regions: &mut HashMap<u32, Region>) {
    let grid_height = plots.len();
    let grid_width = plots[0].len();

    let current_region_id = plots[plot.0][plot.1].region_id.unwrap();
    let current_region_letter = plots[plot.0][plot.1].letter;

    // println!("UPDATING plot ({}, {}) - {}:{}", plot.0, plot.1, current_region_letter, current_region_id.unwrap());

    let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for d in directions {
        let new_row_idx = &(plot.0 as isize + d.0);
        let new_col_idx = &(plot.1 as isize + d.1);
        let should_update =
            (0..grid_height as isize).contains(new_row_idx) &&
            (0..grid_width as isize).contains(new_col_idx) &&
            plots[*new_row_idx as usize][*new_col_idx as usize].letter == current_region_letter &&
            plots[*new_row_idx as usize][*new_col_idx as usize].region_id == None;

        if should_update {
            let next_plot: (usize, usize) = (*new_row_idx as usize, *new_col_idx as usize);
            plots[next_plot.0][next_plot.1].region_id = Some(current_region_id);
            let region = regions.get_mut(&current_region_id).unwrap();
            region.count += 1;
            let new_edges = count_edges(plots, &next_plot);
            region.edges += new_edges;

            update_neighbors(plots, &next_plot, regions)
        }
    }
}

fn count_edges(plots: &Vec<Vec<Plot>>, plot: &(usize, usize)) -> usize {
    let grid_height = plots.len();
    let grid_width = plots[0].len();

    vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter(|d| {
        let new_row = plot.0 as isize + &d.0;
        let new_col = plot.1 as isize + &d.1;

        !(0..grid_height as isize).contains(&new_row) ||
        !(0..grid_width as isize).contains(&new_col) ||
        plots[new_row as usize][new_col as usize].letter != plots[plot.0][plot.1].letter
    }).count()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
