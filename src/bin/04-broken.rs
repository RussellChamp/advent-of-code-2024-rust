advent_of_code::solution!(4);
use std::fs::File;
use std::io::Write;

pub fn part_one(input: &str) -> Option<u32> {
    let result = word_search(input, "XMAS");
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn word_search(input: &str, needle: &str) -> usize {
    // for every character if every line, check in each of 8 directions for a match

    // 1) RIGHT
    let total_right = match_forward(input, needle);
    // println!(" R: Found {} in {:?}", total_right, input);
    File::create("04-data-R.txt").expect("Error creating").write_all(input.as_bytes()).expect("Error writing");

    // Reverse the grid and check it forward
    // 1 2 3        3 2 1
    // 4 5 6  ===>  6 5 4
    // 7 8 9        9 8 7

    // 2) LEFT
    let reverse_input = reverse_string(input);
    let total_left = match_forward(&reverse_input, needle);
    // println!(" L: Found {} in {:?}", total_left, reverse_input);
    File::create("04-data-L.txt").expect("Error creating").write_all(reverse_input.as_bytes()).expect("Error writing");

    // DOWN & UP-RIGHT
    // SKEW the grid to get the DOWN check, TRANSPOSE the SKEWED grid to get the UP-RIGHT check
    // (0,0) (0,1) (0,2) (0,3)       (0,0) (1,0) (2,0) (3,0)   -     -     -          (0,0)   -     -     -
    // (1,0) (1,1) (1,2) (1,3)         -   (0,1) (1,1) (2,1) (3,1)   -     -          (1,0) (0,1)   -     -
    // (2,0) (2,1) (2,2) (2,3)   ==>   -     -   (0,2) (1,2) (2,2) (3,2)   -    ==>   (2,0) (1,1) (0,2)   -
    // (3,0) (3,1) (3,2) (3,3)         -     -     -   (0,3) (1,3) (2,3) (3,3)        (3,0) (2,1) (1,2) (0,3)
    //                                                                                  -   (3,1) (2,2) (1,3)
    //                                                                                  -     -   (3,2) (2,3)
    //                                                                                  -     -     -   (3,3)
    // Eg the "Up-right" check starting from (3,0) and going to (0,3) is now in a row in the skewed grid
    // Eg the "Down" check starting from (0,1) to (3,1)

    let char_vec = vecify_string(&String::from(input));

    // 3) DOWN
    let skewed_vec = skew_vec_bottom(&char_vec);
    let skewed_input = stringify_vec(&skewed_vec);
    let total_down = match_forward(&skewed_input, needle);
    // println!("D : Found {} in {:?}", total_down, skewed_input);
    File::create("04-data-D.txt").expect("Error creating").write_all(skewed_input.as_bytes()).expect("Error writing");

    // 4) UP - Reverse the DOWN vec
    let reversed_skewed_input = reverse_string(&skewed_input);
    let total_up = match_forward(&reversed_skewed_input, needle);
    // println!("U : Found {} in {:?}", total_up, reversed_skewed_input);
    File::create("04-data-U.txt").expect("Error creating").write_all(reversed_skewed_input.as_bytes()).expect("Error writing");

    // 5) UP-RIGHT
    let transposed_skewed_vec = transpose(skewed_vec);
    let transposed_skewed_input = stringify_vec(&transposed_skewed_vec);
    let total_upright = match_forward(&transposed_skewed_input, needle);
    // println!("UR: Found {} in {:?}", total_upright, transposed_skewed_input);
    File::create("04-data-UR.txt").expect("Error creating").write_all(transposed_skewed_input.as_bytes()).expect("Error writing");

    // 6) DOWN-LEFT - Reverse the UP-RIGHT vec
    let reversed_transposed_skewed_vec = reverse_string(&transposed_skewed_input);
    let total_downleft = match_forward(&reversed_transposed_skewed_vec, needle);
    // println!("DL: Found {} in {:?}", total_downleft, reversed_transposed_skewed_vec);
    File::create("04-data-DL.txt").expect("Error creating").write_all(reversed_transposed_skewed_vec.as_bytes()).expect("Error writing");

    // Skew in the reverse direction and transpose it
    // SKEW the grid to get the DOWN check, TRANSPOSE the SKEWED grid to get the UP-RIGHT check
    // (0,0) (0,1) (0,2) (0,3)         -       -     -   (0,0) (1,0) (2,0) (3,0)           -     -     -   (0,3)
    // (1,0) (1,1) (1,2) (1,3)         -       -   (0,1) (1,1) (2,1) (3,1)   -             -     -   (0,2) (1,3)
    // (2,0) (2,1) (2,2) (2,3)   ==>   -     (0,2) (1,2) (2,2) (3,2)   -     -     ==>     -   (0,1) (1,2) (2,3)
    // (3,0) (3,1) (3,2) (3,3)         (0,3) (1,3) (2,3) (3,3)   -     -     -           (0,0) (1,1) (2,2) (3,3)
    //                                                                                   (1,0) (2,1) (3,2)   -
    //                                                                                   (2,0) (3,1)   -     -
    //                                                                                   (3,0)   -     -     -
    // Eg the "DOWN-RIGHT" check starting from (0,0) and going to (3,3) is now in a row in the skewed grid

    // 7) DOWN-RIGHT
    let top_skewed_vec = skew_vec_top(&char_vec);
    let top_skewed_input = stringify_vec(&top_skewed_vec);
    let total_downright = match_forward(&top_skewed_input, needle);
    // println!("DR: Found {} in {:?}", total_downright, top_skewed_input);
    File::create("04-data-DR.txt").expect("Error creating").write_all(top_skewed_input.as_bytes()).expect("Error writing");

    // 7) UP-LEFT - Reverse the DOWN-RIGHT vec
    let reversed_top_skewed_input = reverse_string(&top_skewed_input);
    let total_upleft = match_forward(&reversed_top_skewed_input, needle);
    // println!("UL: Found {} in {:?}", total_upleft, reversed_top_skewed_input);
    File::create("04-data-UL.txt").expect("Error creating").write_all(reversed_top_skewed_input.as_bytes()).expect("Error writing");

    let total = total_right + total_left + total_down + total_up + total_upright + total_downleft + total_downright + total_upleft;

    println!("Right: {total_right}");
    println!("Left: {total_left}");
    println!("Down: {total_down}");
    println!("Up: {total_up}");
    println!("Up-Right: {total_upright}");
    println!("Down-Left: {total_downleft}");
    println!("Down-Right: {total_downright}");
    println!("Up-Left: {total_upleft}");
    println!("Total: {total}");

    total
}


fn match_forward(input: &str, needle: &str) -> usize {
    input.match_indices(needle).count()
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

// shamelessly copied from the internets
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn stringify_vec(grid: &Vec<Vec<char>>) -> String {
    let lines: Vec<String> = grid.iter().map(|l| String::from_iter(l)).collect();
    lines.join("\n")
}

fn vecify_string(str_value: &String) -> Vec<Vec<char>> {
    str_value.split('\n').filter_map(|l| {
        let chars: Vec<char> = l.chars().collect();
        if chars.len() > 0 { Some(chars) } else { None }
    }).collect()
}

fn skew_vec_bottom(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // (0,0) (0,1) (0,2) (0,3)       (0,0) (1,0) (2,0) (3,0)   -     -     -
    // (1,0) (1,1) (1,2) (1,3)         -   (0,1) (1,1) (2,1) (3,1)   -     -
    // (2,0) (2,1) (2,2) (2,3)   ==>   -     -   (0,2) (1,2) (2,2) (3,2)   -
    // (3,0) (3,1) (3,2) (3,3)         -     -     -   (0,3) (1,3) (2,3) (3,3)

    let col_count = grid.len();
    let row_count = grid[0].len();
    let skewed_width = row_count + col_count - 1;

    let mut skewed_vec: Vec<Vec<char>> = vec![vec![' '; skewed_width]; row_count];

    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            // swap and skew
            skewed_vec[row_idx][col_idx+row_idx] = grid[col_idx][row_idx];
        }
    }
    skewed_vec
}

fn skew_vec_top(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // (0,0) (0,1) (0,2) (0,3)         -       -     -   (0,0) (1,0) (2,0) (3,0)
    // (1,0) (1,1) (1,2) (1,3)         -       -   (0,1) (1,1) (2,1) (3,1)   -
    // (2,0) (2,1) (2,2) (2,3)   ==>   -     (0,2) (1,2) (2,2) (3,2)   -     -
    // (3,0) (3,1) (3,2) (3,3)         (0,3) (1,3) (2,3) (3,3)   -     -     -


    let col_count = grid.len();
    let row_count = grid[0].len();
    let skewed_width = row_count + col_count - 1;

    let mut skewed_vec: Vec<Vec<char>> = vec![vec![' '; skewed_width]; row_count];

    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            // swap and skew
            assert!(col_idx + col_count - row_idx - 1 < skewed_width);
            skewed_vec[row_idx][col_idx + col_count - row_idx - 1] = grid[row_idx][col_idx];
        }
    }
    skewed_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skew_vec_bottom() {
        let input_vec = vec![
            vec!['0', '1', '2', '3'],
            vec!['4', '5', '6', '7'],
            vec!['8', '9', 'a', 'b'],
            vec!['c', 'd', 'e', 'f']
        ];

        let skewed_vec = skew_vec_bottom(&input_vec);

        let expected_vec = vec![
            vec!['0', '4', '8', 'c', ' ', ' ', ' '],
            vec![' ', '1', '5', '9', 'd', ' ', ' '],
            vec![' ', ' ', '2', '6', 'a', 'e', ' '],
            vec![' ', ' ', ' ', '3', '7', 'b', 'f']
        ];
        assert_eq!(skewed_vec, expected_vec);
    }

    #[test]
    fn test_skew_vec_top() {
        let input_vec = vec![
            vec!['0', '1', '2', '3'],
            vec!['4', '5', '6', '7'],
            vec!['8', '9', 'a', 'b'],
            vec!['c', 'd', 'e', 'f']
        ];

        let skewed_vec = skew_vec_top(&input_vec);

        let expected_vec = vec![
            vec![' ', ' ', ' ', '0', '1', '2', '3'],
            vec![' ', ' ', '4', '5', '6', '7', ' '],
            vec![' ', '8', '9', 'a', 'b', ' ', ' '],
            vec!['c', 'd', 'e', 'f', ' ', ' ', ' ']
        ];
        assert_eq!(skewed_vec, expected_vec);
    }

    #[test]
    fn test_transpose_square_vec() {
        let input_vec = vec![
            vec!['0', '1', '2', '3'],
            vec!['4', '5', '6', '7'],
            vec!['8', '9', 'a', 'b'],
            vec!['c', 'd', 'e', 'f']
        ];

        let transposed_vec = transpose(input_vec);

        let expected_vec = vec![
            vec!['0', '4', '8', 'c'],
            vec!['1', '5', '9', 'd'],
            vec!['2', '6', 'a', 'e'],
            vec!['3', '7', 'b', 'f']
        ];
        assert_eq!(transposed_vec, expected_vec);
    }

    #[test]
    fn test_transpose_long_vec() {
        let input_vec = vec![
            vec!['0', '1', '2', '3'],
            vec!['4', '5', '6', '7']
        ];

        let transposed_vec = transpose(input_vec);

        let expected_vec = vec![
            vec!['0', '4'],
            vec!['1', '5'],
            vec!['2', '6'],
            vec!['3', '7']
        ];
        assert_eq!(transposed_vec, expected_vec);
    }

    #[test]
    fn test_transpose_tall_vec() {
        let input_vec = vec![
            vec!['0', '1'],
            vec!['2', '3'],
            vec!['4', '5'],
            vec!['6', '7'],
        ];

        let transposed_vec = transpose(input_vec);

        let expected_vec = vec![
            vec!['0', '2', '4', '6'],
            vec!['1', '3', '5', '7']
        ];
        assert_eq!(transposed_vec, expected_vec);
    }

    #[test]
    fn test_match_forward() {
        let input = "abcXMASXMASxyz";
        let count = match_forward(input, "XMAS");

        assert_eq!(count, 2);
    }

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 99));
        assert_eq!(result, Some(18));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
