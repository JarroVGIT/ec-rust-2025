ec::solution!(8);

/*
Notes:
- Part 1 was easy, part 2 was brute force and part 3 was complete bruteforce. 
- I think there is a more elegant way to determine if two threads cross each other. 
  I just can't think of any right now. 

Part 1: (40.708µs)
Part 2: (2.300209ms)
Part 3: (62.972875ms)
*/


#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let instr: Vec<u8> = notes.split(',').map(|n| n.parse().unwrap()).collect();
    let nails = instr.iter().max().unwrap().to_owned();
    let opposite_diff = nails / 2;
    let mut result = 0;
    for win in instr.windows(2) {
        if win[0].abs_diff(win[1]) == opposite_diff {
            result += 1
        }
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Store all paths, already made (in L,H fashion), then compare current path with each;
    // - if com_L < cur_L < com_H && com_H < cur_H  then crossing, knots + 1.
    let instr: Vec<u32> = notes.split(',').map(|n| n.parse().unwrap()).collect();
    let mut visited: Vec<(u32, u32)> = Vec::new();
    let mut result = 0;
    for win in instr.windows(2) {
        let (cur_l, cur_h) = (win[0].min(win[1]), win[0].max(win[1]));
        for &(com_l, com_h) in visited.iter() {
            if (cur_l > com_l && cur_l < com_h && cur_h > com_h)
                || (cur_l < com_l && cur_h < com_h && cur_h > com_l)
            {
                result += 1
            }
        }
        visited.push((cur_l, cur_h))
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let instr: Vec<u32> = notes.split(',').map(|n| n.parse().unwrap()).collect();
    let nails = instr.iter().max().unwrap().to_owned();
    let mut result = 0;
    for cur_l in 1..nails {
        for cur_h in (cur_l + 1)..=nails {
            let mut current_cuts = 0;
            for win in instr.windows(2) {
                let (com_l, com_h) = (win[0].min(win[1]), win[0].max(win[1]));
                if (cur_l > com_l && cur_l < com_h && cur_h > com_h)
                    || (cur_l < com_l && cur_h < com_h && cur_h > com_l)
                    || (cur_l == com_l && cur_h == com_h)
                {
                    current_cuts += 1
                }
            }
            result = result.max(current_cuts)
        }
    }
    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(8, 1));
        assert_eq!(result, Some(4.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(8, 2));
        assert_eq!(result, Some(21.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(8, 3));
        assert_eq!(result, Some(7.to_string()));
    }
}
