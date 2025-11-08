use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

ec::solution!(3);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let result: HashSet<i32> = notes
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let result = result.into_iter().sum::<i32>();
    Some(format!("{result}"))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let result: BTreeSet<i32> = notes
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let result = result.into_iter().take(20).sum::<i32>();
    Some(format!("{result}"))
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let crates: HashMap<i32, usize> = notes
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .counts();
    let result = crates.into_values().max().unwrap();
    Some(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(3, 1));
        assert_eq!(result, Some("29".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(3, 2));
        assert_eq!(result, Some("781".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(3, 3));
        assert_eq!(result, Some("3".to_string()));
    }
}
