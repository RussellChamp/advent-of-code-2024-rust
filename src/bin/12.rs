use itertools::Itertools;
use std::{collections::HashMap, fmt};

advent_of_code::solution!(12);

#[derive(PartialEq)]
struct Plot {
    letter: char,
    region_id: Option<u32>,
}
impl fmt::Debug for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.letter, self.region_id.unwrap_or(0))
    }
}

type Direction = (isize, isize);
// impl fmt::Debug for Direction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self == &(-1 as isize, 0 as isize) { write!(f, "UP") }
//         else if self == &(1 as isize, 0 as isize) { write!(f, "DOWN") }
//         else if self == &(0 as isize, -1 as isize) { write!(f, "LEFT") }
//         else if self == &(0 as isize, 1 as isize) { write!(f, "RIGHT") }
//         else { write!(f, "({}, {})", self.0, self.1) }
//     }
// }

#[derive(Debug)]
struct Region {
    id: u32,
    edges: usize,
    count: usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut plots = input.lines().map(|l| l.chars().map(|c| Plot { letter: c, region_id: None }).collect_vec()).collect_vec();
    let mut regions: HashMap<u32, Region> = HashMap::new();
    let mut next_region_id = 1;

    while let Some(plot) = find_plot(&plots, |p| p.region_id == None) {
        plots[plot.0][plot.1].region_id = Some(next_region_id);

        let edge_count = count_edges(&plots, &plot);
        regions.insert(next_region_id, Region { id: next_region_id, edges: edge_count, count: 1 });
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
    let mut plots = input.lines().map(|l| l.chars().map(|c| Plot { letter: c, region_id: None }).collect_vec()).collect_vec();
    let mut regions: HashMap<u32, Region> = HashMap::new();
    let mut next_region_id = 1;

    while let Some(plot) = find_plot(&plots, |p| p.region_id == None) {
        plots[plot.0][plot.1].region_id = Some(next_region_id);

        let edge_count = count_edges(&plots, &plot);
        regions.insert(next_region_id, Region { id: next_region_id, edges: edge_count, count: 1 });
        next_region_id += 1;

        update_neighbors(&mut plots, &plot, &mut regions);
    }

    let w = plots.iter().map(|p| p.iter().map(|p| p.letter).collect::<String>()).join("\n");
    println!("{}", w);

    let total = regions.iter().sorted_by_key(|r| r.0).fold(0, |sum, (region_id, region)| {
        let start_plot = find_plot(&plots, |p| p.region_id == Some(*region_id)).unwrap();
        print!("REGION {}:", region.id);
        let edges = traverse_start(&plots, &start_plot);
        let total = sum + (region.count * edges);
        println!(" = {} * {} = {}", region.count, edges, region.count * edges);
        total
    });

    Some(total as u32)
}

fn find_plot<F>(plots: &Vec<Vec<Plot>>, predicate: F) -> Option<(usize, usize)> where
F: Fn(&Plot) -> bool {
    for (row_idx, row) in plots.iter().enumerate() {
        for (col_idx, plot) in row.iter().enumerate() {
            if predicate(plot) {
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

    let directions: Vec<Direction> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
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

fn traverse_start(plots: &Vec<Vec<Plot>>, plot: &(usize, usize)) -> usize {
    let start_direction: Direction = (0, 1);

    let mut prior_plot = plot.clone();
    let mut prior_direction = start_direction.clone();

    println!(" at {:?} going {}", plot, debug_direction(start_direction));

    // count the number of times we turn a corner
    let mut corners = 0;
    while let Some((next_plot, next_direction)) = traverse_step(plots, &prior_plot, prior_direction) {
        if next_direction != prior_direction {
            corners += 1;
        }
        if  next_plot == *plot && next_direction == start_direction { break }
        prior_plot = next_plot;
        prior_direction = next_direction;
        // let (next_plot, next_direction) = traverse_step(plots, &next_plot, next_direction).unwrap();
    }
    corners
}

fn traverse_step(plots: &Vec<Vec<Plot>>, plot: &(usize, usize), direction: Direction) -> Option<((usize, usize), Direction)> {
    let grid_height = plots.len();
    let grid_width = plots[0].len();

    // RIGHT, UP, LEFT, DOWN, RIGHT
    let directions: Vec<Direction> = vec![(0, 1), (-1, 0), (0, -1), (1, 0), (0, 1)];
    let get_at = |row_idx: isize, col_idx: isize| {
        if !(0..grid_height as isize).contains(&row_idx) || !(0..grid_width as isize).contains(&col_idx) { return None; }
        let new_plot = &plots[row_idx as usize][col_idx as usize];
        if new_plot.region_id != plots[plot.0][plot.1].region_id { return None; }
        Some(new_plot)
    };

    let ccw_curve = directions.windows(2).find(|dd| dd[0] == direction).map(|dd| dd[1]).unwrap();
    let ccw_plot = get_at(plot.0 as isize + ccw_curve.0 as isize, plot.1 as isize + ccw_curve.1 as isize);
    let next_plot = get_at(plot.0 as isize + direction.0 as isize, plot.1 as isize + direction.1 as isize);

    // TODO - fix this traversal code
    if ccw_plot != None {
        // if the CCW plot IS in the region, ROTATE CCW
        print!("↺ ");
        return Some((*plot, ccw_curve));
    } else if next_plot != None {
        // if the NEXT plot IS in the region, MOVE NEXT
        print!("{} ", debug_direction(direction));
        return Some((((plot.0 as isize + direction.0 as isize) as usize, (plot.1 as isize + direction.1 as isize) as usize), direction));
    } else {
        // if the NEXT plot is NOT in the region, ROTATE CW
        let cw_curve = directions.iter().rev().collect_vec().windows(2).find(|dd| *dd[0] == direction).map(|dd| *dd[1]).unwrap();
        print!("↻ ");
        return Some((*plot, cw_curve ))
    }

    // When we get back to the FIRST plot and are at the same direction, stop counting
}

fn debug_direction(d: Direction) -> char {
    let arrows = HashMap::from([
        ((-1 as isize, 0 as isize), '↑'),
        ((1 as isize, 0 as isize),  '↓'),
        ((0 as isize, -1 as isize), '←'),
        ((0 as isize, 1 as isize),  '→')
    ]);

    *arrows.get(&d).unwrap_or(&'X')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let values = vec![1,2,3,4,5,6];
        let result = values.windows(2).find(|a| a[0] == 4).map(|a| a[1]);
        assert_eq!(result, Some(5));

        let result = values.iter().rev().collect_vec().windows(2).find(|a| *a[0] == 4).map(|a| *a[1]);
        assert_eq!(result, Some(3));
    }

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
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(1206));
    }
}
