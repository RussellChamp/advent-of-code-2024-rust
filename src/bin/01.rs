advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    let mut result = 0;

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let vals: Vec<&str> = line.split("   ").collect();

        let a = vals[0].parse::<u32>().unwrap();
        let b = vals[1].parse::<u32>().unwrap();
        vec_a.push(a);
        vec_b.push(b);
    }
    vec_a.sort();
    vec_b.sort();

    for i in 0..vec_a.len() {
        let diff = vec_a[i].abs_diff(vec_b[i]);
        result += diff;
        // println!("Line {}: abs({} - {}) = {}", i+1, vec_a[i], vec_b[i], diff)
    }
    // println!("Total {result}");
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    let mut result = 0;

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let vals: Vec<&str> = line.split("   ").collect();

        let a = vals[0].parse::<u32>().unwrap();
        let b = vals[1].parse::<u32>().unwrap();
        vec_a.push(a);
        vec_b.push(b);
    }
    vec_a.sort();
    vec_b.sort();

    for item in &vec_a {
        let found = vec_b.iter().filter(|&b| b == item).count() as u32;
        let similarity = found * item;
        result += similarity;

        // println!("Line {}: {} found {} times = {}, total {}", i + 1, needle, found, similarity, result);
    }
    // println!("Total {result}");
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
