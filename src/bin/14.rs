use regex::Regex;

advent_of_code::solution!(14);

pub fn get_safety_quotent(input: &str, grid_cols: isize, grid_rows: isize, steps: isize) -> Option<u32> {
    let r = Regex::new(r"p=(?<pos_x>\d+),(?<pos_y>\d+) v=(?<vel_x>-?\d+),(?<vel_y>-?\d+)").unwrap();

    let quadrants = input.lines().enumerate().fold((0,0,0,0,0), |q, (_idx, l)| {
        let c = r.captures(l).unwrap();
        let pos_x = c["pos_x"].parse::<isize>().unwrap();
        let pos_y = c["pos_y"].parse::<isize>().unwrap();

        let vel_x = c["vel_x"].parse::<isize>().unwrap();
        let vel_y = c["vel_y"].parse::<isize>().unwrap();

        let new_x = (pos_x + vel_x * steps).rem_euclid(grid_cols);
        let new_y = (pos_y + vel_y * steps).rem_euclid(grid_rows);

        let middle_x = grid_cols / 2;
        let middle_y = grid_rows / 2;

        if new_x < middle_x && new_y < middle_y {
            // TOP LEFT SIDE
            // println!("R#{idx} START ({pos_x},{pos_y}) VEL ({vel_x},{vel_y}) ==> ({new_x}, {new_y}) TL");
            (q.0 + 1, q.1, q.2, q.3, q.4)
        } else if new_x > middle_x && new_y < middle_y {
            // TOP RIGHT SIDE
            // println!("R#{idx} START ({pos_x},{pos_y}) VEL ({vel_x},{vel_y}) ==> ({new_x}, {new_y}) TR");
            (q.0, q.1 + 1, q.2, q.3, q.4)
        } else if new_x < middle_x && new_y > middle_y {
            // BOTTOM LEFT SIDE
            // println!("R#{idx} START ({pos_x},{pos_y}) VEL ({vel_x},{vel_y}) ==> ({new_x}, {new_y}) BL");
            (q.0, q.1, q.2 + 1, q.3, q.4)
        } else if new_x > middle_x && new_y > middle_y {
            // BOTTOM RIGHT SIDE
            // println!("R#{idx} START ({pos_x},{pos_y}) VEL ({vel_x},{vel_y}) ==> ({new_x}, {new_y}) BR");
            (q.0, q.1, q.2, q.3 + 1, q.4)
        } else {
            // println!("R#{idx} START ({pos_x},{pos_y}) VEL ({vel_x},{vel_y}) ==> ({new_x}, {new_y}) EDGE");
            (q.0, q.1, q.2, q.3, q.4 + 1)
        }
    });

    // println!("QUADRANTS {:?}", quadrants);
    let safety_quotent = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;

    Some(safety_quotent)
}

pub fn part_one(input: &str) -> Option<u32> {
    get_safety_quotent(input, 101, 103, 100)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = get_safety_quotent(&advent_of_code::template::read_file("examples", DAY), 11, 7, 100);
        assert_eq!(result, Some(12));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
