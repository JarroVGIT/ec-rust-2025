use std::cmp;

use itertools::{EitherOrBoth, Itertools};

ec::solution!(5);

const LEFT: usize = 0;
const MID: usize = 1;
const RIGHT: usize = 2;

fn calculate_quality(line: &str) -> (String, Vec<[Option<usize>; 3]>) {
    let mut numbers = line
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(|n| n.parse::<usize>().unwrap());
    let mut sword: Vec<[Option<usize>; 3]> = vec![[None, numbers.next(), None]];

    for number in numbers {
        let num_segments = sword.len();
        let mut placed = false;
        for seg in 0..num_segments {
            if sword[seg][LEFT].is_none() && number < sword[seg][MID].unwrap() {
                sword[seg][LEFT] = Some(number);
                placed = true;
                break;
            } else if sword[seg][RIGHT].is_none() && number > sword[seg][MID].unwrap() {
                sword[seg][RIGHT] = Some(number);
                placed = true;
                break;
            }
        }
        if !placed {
            // push new segment with this as middle
            let segment = [None, Some(number), None];
            sword.push(segment);
        }
    }
    let quality = sword
        .iter()
        .map(|&seg| seg[MID].unwrap().to_string())
        .collect();
    (quality, sword)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let result = calculate_quality(notes).0;
    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for line in notes.lines() {
        let quality = calculate_quality(line).0.parse::<usize>().unwrap();
        min = min.min(quality);
        max = max.max(quality);
    }
    Some(format!("{}", max - min))
}

fn array_to_number(arr: &[Option<usize>; 3]) -> usize {
    // Cool, a slice of Option's can be flattened, because Option (and Result, for that matter)
    // implements IntoIterator
    arr.iter().flatten().fold(0, |acc, &digit| acc * 10 + digit)
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // (id, qual, sword)
    let mut swords = notes
        .lines()
        .map(|line| {
            let (id, _) = line.split_once(':').unwrap();
            let id: usize = id.parse().unwrap();
            let (quality, sword) = calculate_quality(line);
            (id, quality.parse::<usize>().unwrap(), sword)
        })
        .collect_vec();
    swords.sort_by(|a, b| {
        // Compare quality first
        a.1.cmp(&b.1)
            .then_with(|| {
                // then_with, really happy that I found this, very nice to chain comparisons.
                // Compare segments level by level
                for pair in a.2.iter().zip_longest(b.2.iter()) {
                    let ordering = match pair {
                        EitherOrBoth::Both(seg_a, seg_b) => {
                            array_to_number(seg_a).cmp(&array_to_number(seg_b))
                        }
                        EitherOrBoth::Left(_) => cmp::Ordering::Greater,
                        EitherOrBoth::Right(_) => cmp::Ordering::Less,
                    };
                    if ordering != cmp::Ordering::Equal {
                        return ordering;
                    }
                }
                cmp::Ordering::Equal
            })
            .then_with(|| a.0.cmp(&b.0)) // ID comparison as tiebreaker
    });

    let result = swords
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, (id, _, _))| (idx + 1) * id)
        .sum::<usize>();

    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(5, 1));
        assert_eq!(result, Some("581078".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(5, 2));
        assert_eq!(result, Some("77053".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(5, 3));
        assert_eq!(result, Some("260".to_string()));
    }

    #[test]
    fn test_part_three_2() {
        let result = part_three(&read_example_file(5, 4));
        assert_eq!(result, Some("4".to_string()));
    }
}
