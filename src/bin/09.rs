use std::ops::Mul;

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    // println!("INPUT:");
    // println!("{}", input);

    let blocks = create_blocks(input.chars().collect_vec());
    // println!("BLOCKS:");
    // let blocks_str = blocks.iter().map(|b| if *b == None { "." } else { "B" }).collect::<String>();
    // println!("{}", blocks_str);

    let compressed_blocks = move_blocks(blocks);
    // println!("COMPRESSED:");
    // let compressed_blocks_str = compressed_blocks.iter().map(|b| if *b == None { "." } else { "B" }).collect::<String>();
    // println!("{}", compressed_blocks_str);

    let checksum = get_checksum(compressed_blocks);

    Some(checksum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn create_blocks(diskmap: Vec<char>) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();

    for (idx, size) in diskmap.iter().filter(|c| **c != '\n').enumerate() {
        let len = *size as usize - '0' as usize;
        if idx % 2 == 0 {
            blocks.extend(vec![Some(idx / 2); len]);
        } else {
            blocks.extend(vec![None; len]);
        }
    }
    blocks
}

fn move_blocks(mut blocks: Vec<Option<usize>>) -> Vec<Option<usize>>{
    let mut head_ptr = 0;
    let mut tail_ptr = blocks.len() - 1;

    while blocks[tail_ptr] == None {
        tail_ptr -= 1;
    }

    while head_ptr <= tail_ptr {
        if blocks[head_ptr] == None {
            blocks.swap(head_ptr, tail_ptr);
        }

        head_ptr += 1;

        while blocks[tail_ptr] == None {
            tail_ptr -= 1;
        }
    }

    blocks
}

fn get_checksum(blocks: Vec<Option<usize>>) -> usize {
    let checksum = blocks.into_iter().enumerate().fold(0 as usize, |sum, (idx, id)| {
        if id == None {
            sum
        } else {
            let total = sum + idx.mul(id.unwrap() as usize);
            total
        }
    });

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_block() {
        let input = "2333133121414131402\n";
        let blocks = create_blocks(input.chars().collect_vec());

        let result_str = blocks.iter().fold("".to_string(), |cur, next| {
            if *next == None {
                cur + "."
            } else {
                cur + &next.unwrap().to_string()
            }
        });

        assert_eq!(result_str, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_move_blocks() {
        let blocks_str = "00...111...2...333.44.5555.6666.777.888899";
        let blocks = blocks_str.chars().map(|c| if c == '.' { None } else { Some(c as usize - '0' as usize) }).collect_vec();

        let result = move_blocks(blocks);
        let result_str = result.iter().map(|b| if *b == None { '.' } else { ('0' as u8 + b.unwrap() as u8) as char }).collect::<String>();

        assert_eq!(result_str, "0099811188827773336446555566..............");
    }

    #[test]
    fn test_get_checksum() {
        let blocks_str = "0099811188827773336446555566..............";
        let blocks= blocks_str.chars().map(|c| if c == '.' { None } else { Some(c as usize - '0' as usize) }).collect_vec();
        let result = get_checksum(blocks);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
