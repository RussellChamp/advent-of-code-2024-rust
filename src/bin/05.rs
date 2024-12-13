// use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the rules
    let mut reader = input.lines();
    let rules: Vec<_> = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| r.split('|').collect::<Vec<&str>>()).collect();

    // println!("{} lines", rules.len());

    // Parse the books
    let books = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| r.split(','));

    // Check each book against all the rules
    let mut valid_books_count = 0;
    let total = books.fold(0, |t, book| {
        let pages = book.collect::<Vec<&str>>();

        let is_valid = get_is_valid(&rules, &pages);

        if is_valid {
            let middle = pages.len() / 2;
            let middle_val = pages[middle].parse::<u32>().unwrap();
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

#[allow(unreachable_code)]
pub fn part_two(_input: &str) -> Option<u32> {
    // TODO: Fix this
    return None;

    // Parse the rules
    let mut reader = _input.lines();
    let rules: Vec<_> = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| r.split('|').collect::<Vec<&str>>()).collect();

    // println!("{} lines", rules.len());

    // Parse the books
    let books = reader.by_ref().take_while(|l| !l.is_empty()).map(|r| r.split(','));

    // Check each book against all the rules
    let mut valid_books_count = 0;
    let total = books.fold(0, |t, book| {
        let pages = book.collect::<Vec<&str>>();
        let is_valid = get_is_valid(&rules, &pages);

        if is_valid {
            t
        } else {
            // println!("Invalid pages {:?}", pages);

            // build up a book one page at a time (starting with 2 pages) and
            // println!("Checking {} permutations of {:?}", factorial(pages.len()).unwrap(), pages);
            let valid_pages = find_valid_book(&rules, Vec::from([]), pages).unwrap();

            // println!("Valid pages {:?}", valid_pages);

            let middle = valid_pages.len() / 2;
            let middle_val = valid_pages[middle].parse::<u32>().unwrap();
            // println!("{middle_val}");

            valid_books_count += 1;
            t + middle_val
        }
    });

    // println!("Total {}, value {}", valid_books_count, total);
    Some(total)
}

fn get_is_valid(rules: &[Vec<&str>], pages: &[&str]) -> bool {
    for (page_a_idx, page_a) in pages.iter().enumerate() {
        for page_b in &pages[page_a_idx..] {
            // check for a violation of the rules!
            let is_invalid = rules.iter().any(|r| r[0] == *page_b && r[1] == *page_a);
            if is_invalid { return false }
        }
    }
    true
}

fn find_valid_book<'a>(rules: &Vec<Vec<&str>>, pages: Vec<&'a str>, remaining_pages: Vec<&'a str>) -> Option<Vec<&'a str>> {
    // println!("Checking {:?}", pages);

    let is_valid = get_is_valid(rules, &pages);

    if remaining_pages.is_empty()  && is_valid {

        return Some(pages)
    }

    if !is_valid {
        // println!("INVALID! {:?} - Eliminating {} additional permutations", pages, factorial(remaining_pages.len()).unwrap());
        return None
    } // trim this branch

    remaining_pages.iter().find_map(|rp| {
        let mut next_pages = pages.clone();
        next_pages.push(rp);
        let next_remaining_pages = remaining_pages.iter().cloned().filter(|p| p != rp).collect();

        find_valid_book(rules, next_pages, next_remaining_pages)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
