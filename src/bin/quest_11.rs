use itertools::Itertools;
ec::solution!(11);

/*
Notes:
- Lucky I checked the notes for part 3 and saw they were sorted already. 

Part 1:  (2µs)
Part 2:  (386.39975ms)
Part 3:  (5.125µs)
*/

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut columns: Vec<usize> = notes.lines().map(|l| l.parse().unwrap()).collect();
    let num_cols = columns.len();
    let rounds = 10;
    let mut first_phase = true;
    
    for _ in 0..rounds {
        if first_phase {
            let mut changed = false;
            for (a,b) in (0..num_cols).tuple_windows() {
                if columns[a] > columns[b] {
                    columns[a] -= 1;
                    columns[b] += 1;
                    changed = true;
                }            
            }
            
            if !changed {
                first_phase = false;
            } 
        } 

        if !first_phase {
            for (a,b) in (0..num_cols).tuple_windows() {
                if columns[b] > columns[a] {
                    columns[b] -= 1;
                    columns[a] += 1;
                }            
            }
        }
    }
    let checksum = columns.iter().zip(1usize..).map(|(col, idx)| col * idx).sum::<usize>();
    Some(checksum.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut columns: Vec<usize> = notes.lines().map(|l| l.parse().unwrap()).collect();
    let num_cols = columns.len();
    let mut first_phase = true;
    for round in 0.. {
        if first_phase {
            let mut changed = false;
            for (a,b) in (0..num_cols).tuple_windows() {
                if columns[a] > columns[b] {
                    columns[a] -= 1;
                    columns[b] += 1;
                    changed = true;
                }            
            }
            
            if !changed {
                first_phase = false;
            } 
        } 

        if !first_phase {
            let mut changed = false;
            for (a,b) in (0..num_cols).tuple_windows() {
                if columns[b] > columns[a] {
                    columns[b] -= 1;
                    columns[a] += 1;
                    changed = true;
                }            
            }
            if !changed {
                return Some(round.to_string())
            }
        }
    }
    None
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // The notes are sorted smallest to largest. Phase 1 is skipped.
    // Every round, one block is moved from largest halve to smallest halve.
    let columns: Vec<usize> = notes.lines().map(|l| l.parse().unwrap()).collect();
    let num_cols = columns.len();

    // Double check premise:
    let mut check = columns.clone();
    check.sort_unstable();
    assert_eq!(check, columns);

    let avg = columns.iter().sum::<usize>() / num_cols;
    let result: usize = columns.iter().map(|&v| v.saturating_sub(avg)).sum();

    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(11, 1));
        assert_eq!(result, Some(109.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(11, 2));
        assert_eq!(result, Some(1579.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(11, 3));
        assert_eq!(result, None);
    }
}
