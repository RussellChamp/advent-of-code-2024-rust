// use itertools::Itertools;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the rules
    let mut reader = input.lines();
    let rules = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| {
        let pages = r.split('|').collect_vec();
        (pages[0].parse::<u32>().unwrap(), pages[1].parse::<u32>().unwrap())
    }).collect_vec();

    // println!("{} lines", rules.len());

    // Parse the books
    let books = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| r.split(','));

    // Check each book against all the rules
    let mut valid_books_count = 0;
    let total = books.fold(0, |t, book| {
        let pages = book.map(|s| s.parse::<u32>().unwrap()).collect_vec();

        let is_valid = get_is_valid(&rules, &pages);

        if is_valid {
            let middle = pages.len() / 2;
            let middle_val = pages[middle];
            // println!("{middle_val}");

            valid_books_count += 1;
            t + middle_val
        } else {
            t
        }
    });

    // println!("Total {}, value {}", valid_books_count, total);
    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {

    // Parse the rules
    let mut reader = _input.lines();
    let rules = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| {
        let pages = r.split('|').collect_vec();
        (pages[0].parse::<u32>().unwrap(), pages[1].parse::<u32>().unwrap())
    }).collect_vec();

    // println!("{} lines", rules.len());

    // Parse the books
    let books = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| r.split(','));

    // Check each book against all the rules
    let mut valid_books_count = 0;
    let total = books.fold(0, |t, book| {
        let pages = book.map(|s| s.parse::<u32>().unwrap()).collect_vec();
        let is_valid = get_is_valid(&rules, &pages);

        if is_valid {
            t
        } else {
            // println!("Invalid pages {:?}", pages);

            // build up a book one page at a time (starting with 2 pages) and
            // println!("Checking {} permutations of {:?}", factorial(pages.len()).unwrap(), pages);
            let valid_pages = order_book(&rules, pages);

            // println!("Valid pages {:?}", valid_pages);

            let middle = valid_pages.len() / 2;
            let middle_val = valid_pages[middle];
            // println!("{middle_val}");

            valid_books_count += 1;
            t + middle_val
        }
    });

    // println!("Total {}, value {}", valid_books_count, total);
    Some(total)
}

fn get_is_valid(rules: &[(u32, u32)], pages: &[u32]) -> bool {
    pages.iter().combinations(2).all(|pages| {
            // check for a violation of the rules!
            !rules.iter().any(|r| r.0 == *pages[1] && r.1 == *pages[0])
    })
}

fn order_book(rules: &[(u32, u32)], pages: Vec<u32>) -> Vec<u32> {
    let mut ordered_pages = vec![];

    'page: for p in pages {
        for idx in 0..=ordered_pages.len() {
            let mut new_pages = ordered_pages.clone();
            new_pages.insert(idx, p);
            if get_is_valid(rules, new_pages.as_slice()) {
                ordered_pages.insert(idx, p);
                continue 'page;
            } else if idx == ordered_pages.len() {
                panic!();
            }
        }
    }

    ordered_pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let result = (1..=4).combinations(2);
        itertools::assert_equal(result, vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
