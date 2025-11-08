use std::collections::VecDeque;

use itertools::Itertools;

ec::solution!(1);

// Arvaris,Felmardrith,Fyndtyr,Vaelorath,Orahgonn,Brivor,Zraalvor,Bryndin,Varinmarn,Sorphor

// L1,R5,L9,R7,L2,R5,L2,R2,L4,R5,L1

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let lines = notes.lines().collect_vec();
    let names = lines[0].split(',').collect_vec();
    let moves = lines[2]
        .split(',')
        .map(|m| {
            let (dir, steps) = m.split_at(1);
            (dir, steps.parse::<usize>().unwrap())
        })
        .collect_vec();
    let mut pos: usize = 0;
    let max_pos = names.len() - 1;
    for (dir, steps) in moves {
        match dir {
            "L" => pos = pos.saturating_sub(steps),
            _ => pos = max_pos.min(pos + steps),
        }
    }
    Some(names[pos].to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let lines = notes.lines().collect_vec();
    let mut names: VecDeque<_> = lines[0].split(',').collect();
    let moves = lines[2]
        .split(',')
        .map(|m| {
            let (dir, steps) = m.split_at(1);
            (dir, steps.parse::<usize>().unwrap())
        })
        .collect_vec();

    for (dir, steps) in moves {
        match dir {
            "L" => names.rotate_right(steps),
            _ => names.rotate_left(steps),
        }
    }
    Some(names[0].to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let lines = notes.lines().collect_vec();
    let mut names: Vec<_> = lines[0].split(',').collect();
    let moves = lines[2]
        .split(',')
        .map(|m| {
            let (dir, steps) = m.split_at(1);
            (dir, steps.parse::<usize>().unwrap())
        })
        .collect_vec();
    let len = names.len() as i32;
    for (dir, steps) in moves {
        let idx = match dir {
            "L" => {
                // L3 means 3rd from the end
                // Wrap if steps > len
                let wrapped = steps % names.len();
                names.len() - wrapped
            }
            "R" => {
                // R3 means index 3, use mod to wrap around.
                steps % names.len()
            }
            _ => panic!("Invalid direction: {}", dir),
        };

        if idx != 0 {
            // Don't swap with self
            names.swap(0, idx);
        }
    }
    Some(names[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(1, 1));
        assert_eq!(result, Some("Fyrryn".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(1, 2));
        assert_eq!(result, Some("Elarzris".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(1, 3));
        assert_eq!(result, Some("Drakzyph".to_string()));
    }
}
