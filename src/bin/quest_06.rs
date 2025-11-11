use std::collections::HashMap;

use itertools::Itertools;

ec::solution!(6);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut positions: HashMap<char, Vec<usize>> = HashMap::new();
    for (idx, ch) in notes.char_indices() {
        positions.entry(ch).or_default().push(idx);
    }

    let experts = positions.get(&'A').cloned().unwrap();
    let novices = positions.get(&'a').cloned().unwrap();

    let mut result = 0;
    for novice in novices.into_iter() {
        result += experts.iter().filter(|&e| *e < novice).count();
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut positions: HashMap<char, Vec<usize>> = HashMap::new();
    for (idx, ch) in notes.char_indices() {
        positions.entry(ch).or_default().push(idx);
    }

    let all_novices = positions
        .keys()
        .cloned()
        .filter(|k| k.is_ascii_lowercase())
        .collect_vec();

    let mut result = 0;
    for novice in all_novices.into_iter() {
        let expert = novice.to_uppercase().next().unwrap();
        let n_positions = positions.get(&novice).cloned().unwrap();
        let e_positions = positions.get(&expert).cloned().unwrap();
        for n_pos in n_positions {
            result += e_positions.iter().filter(|&e_pos| *e_pos < n_pos).count();
        }
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    const REPETITIONS: usize = 1000;
    const MAX_DISTANCE: usize = 1000;

    let pattern_len = notes.len();

    let first_context = format!("{}{}", notes, notes);
    let offset_first = 0;

    let middle_context = format!("{}{}{}", notes, notes, notes);
    let offset_middle = pattern_len;

    let last_context = format!("{}{}", notes, notes);
    let offset_last = pattern_len;

    let first_count = count_pairs_in_context(&first_context, offset_first, pattern_len, MAX_DISTANCE);
    let middle_count = count_pairs_in_context(&middle_context, offset_middle, pattern_len, MAX_DISTANCE);
    let last_count = count_pairs_in_context(&last_context, offset_last, pattern_len, MAX_DISTANCE);

    let result = first_count + middle_count * (REPETITIONS - 2) + last_count;

    Some(result.to_string())
}

fn count_pairs_in_context(
    context: &str,
    novice_start: usize,
    pattern_len: usize,
    max_distance: usize,
) -> usize {
    let mut positions: HashMap<char, Vec<usize>> = HashMap::new();
    for (idx, ch) in context.char_indices() {
        positions.entry(ch).or_default().push(idx);
    }

    let novice_end = novice_start + pattern_len;

    let all_novices: Vec<char> = positions
        .keys()
        .filter(|k| k.is_ascii_lowercase())
        .copied()
        .collect();

    let mut result = 0;

    for novice in all_novices {
        let expert = novice.to_ascii_uppercase();

        let n_positions: Vec<usize> = positions
            .get(&novice)
            .unwrap()
            .iter()
            .filter(|&&pos| pos >= novice_start && pos < novice_end)
            .copied()
            .collect();

        let e_positions = positions.get(&expert).cloned().unwrap();

        for n_pos in n_positions {
            result += e_positions.iter().filter(|&e_pos| e_pos.abs_diff(n_pos) <= max_distance).count();

        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(6, 1));
        assert_eq!(result, Some("5".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(6, 2));
        assert_eq!(result, Some("11".to_string()));
    }
}
