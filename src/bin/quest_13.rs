ec::solution!(13);

/*
Notes:
- Pretty happy with how I build the wheel for part 1, not so much for part two, seems like
  there must be a faster way to do that. Maybe pre-compute the Vec length based on the ranges
  and extend() for each range? 
Part 1: 605 (3.667µs)
Part 2: 5507 (859µs)
Part 3: 405022 (459.241792ms)
*/

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut numbers = vec![1];
    let notes: Vec<_> = notes.lines().map(|l| l.parse().unwrap()).collect();
    numbers.extend(notes.iter().step_by(2));
    numbers.extend(notes.iter().skip(1).step_by(2).rev());

    let idx = 2025usize.rem_euclid(numbers.len());
    Some(numbers[idx].to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut numbers = vec![vec![1]];
    let notes: Vec<Vec<_>> = notes.lines().map(|l| {
        let (low, high) = l.split_once('-').unwrap();
        let (low, high) = (low.parse().unwrap(), high.parse().unwrap());
        (low..=high).collect()
    }).collect();
    numbers.extend(notes.iter().step_by(2).cloned());
    numbers.extend(notes.into_iter().skip(1).step_by(2).rev().map(|v| v.into_iter().rev().collect()));
    let dial: Vec<_> = numbers.iter().flatten().collect();
    let idx = 20252025usize.rem_euclid(dial.len());
    Some(dial[idx].to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
        let mut numbers = vec![vec![1]];
    let notes: Vec<Vec<_>> = notes.lines().map(|l| {
        let (low, high) = l.split_once('-').unwrap();
        let (low, high) = (low.parse().unwrap(), high.parse().unwrap());
        (low..=high).collect()
    }).collect();
    numbers.extend(notes.iter().step_by(2).cloned());
    numbers.extend(notes.into_iter().skip(1).step_by(2).rev().map(|v| v.into_iter().rev().collect()));
    let dial: Vec<_> = numbers.iter().flatten().collect();
    let idx = 202520252025usize.rem_euclid(dial.len());
    Some(dial[idx].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(13, 1));
        assert_eq!(result, Some(67.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(13, 2));
        assert_eq!(result, Some(30.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(13, 3));
        assert_eq!(result, Some(1.to_string()));
    }
}
