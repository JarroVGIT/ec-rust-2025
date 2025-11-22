ec::solution!(14);

/*
Notes:
- Due to time constraints, I already had looked at solutions on Reddit. This is heavily inspired by that, 
  but the most important piece was the bitwise XOR-ing. a^b^C^d^e is 1 if an odd number is 1, else 0. In
  case of this assignment, if C = 1 (tile is active), and 1 or 3 neighbours are 1, we need one. But with
  normal XOR this will yield 0, so we have to NOT it. 
- The cycle finding was interesting to do, 
*/

fn parse_grid(notes: &str) -> Vec<u64> {
    let mut grid = vec![];
    for line in notes.lines() {
        let mut row = 0u64;
        for (x, val) in line.char_indices() {
            if val == '#' {
                row |= 1 << x;
            }
        }
        grid.push(row);
    }
    grid
}

fn step(grid: &Vec<u64>, tmp: &mut Vec<u64>, width: usize) {
    let mask = (1u64 << width) - 1;
    for y in 0..grid.len() {
        let prev = if y == 0 { 0 } else { grid[y - 1] };
        let next = if y == grid.len() - 1 { 0 } else { grid[y + 1] };
        let new_row = !(prev << 1 ^ prev >> 1 ^ grid[y] ^ next << 1 ^ next >> 1) & mask;
        tmp[y] = new_row;
    }
}
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut grid = parse_grid(notes);
    let mut tmp = vec![0u64; grid.len()];
    let width = notes.lines().next().unwrap().len();
    let mut result = 0;
    for _ in 0..10 {
        step(&grid, &mut tmp, width);
        (grid, tmp) = (tmp, grid);
        result += grid.iter().map(|v| v.count_ones() ).sum::<u32>();
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut grid = parse_grid(notes);
    let mut tmp = vec![0u64; grid.len()];
    let width = notes.lines().next()?.len();
    let mut result = 0;
    for _ in 0..2025 {
        step(&grid, &mut tmp, width);
        (grid, tmp) = (tmp, grid);
        result += grid.iter().map(|v| v.count_ones() ).sum::<u32>();
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let small_grid = parse_grid(notes);
    let small_width = notes.lines().next().unwrap().len();
    let small_height = small_grid.len();
    
    // Create empty 34x34 grid
    let mut grid = vec![0u64; 34];
    let mut tmp = vec![0u64; 34];
    let width = 34;
    
    let mut total_matched = 0usize;
    let mut last_match = 0usize;
    let mut matches: Vec<(usize, usize)> = Vec::new();

    let row_offset = (34 - small_height) / 2;
    let col_offset = (34 - small_width) / 2;
    let mask = (1u64 << small_width) - 1;

    for round in 0..1_000_000_000usize {
        step(&grid, &mut tmp, width);
        std::mem::swap(&mut grid, &mut tmp);
        
        let live_count: u32 = grid.iter().map(|row| row.count_ones()).sum();

        // Check if center matches the input pattern
        let mut match_found = true;
        for r in 0..small_height {
            let grid_row_bits = (grid[r + row_offset] >> col_offset) & mask;
            if grid_row_bits != small_grid[r] {
                match_found = false;
                break;
            }
        }

        if match_found {
            total_matched += live_count as usize;
            matches.push((round - last_match, live_count as usize));
            last_match = round;
            
            for i in 0..matches.len() - 1 {
                if matches[i] == matches[matches.len() - 1] {
                    let cycle_len: usize = matches[i + 1..].iter().map(|x| x.0).sum();
                    let cycle_sum: usize = matches[i + 1..].iter().map(|x| x.1).sum();
                    let cycles = (1_000_000_000 - round - 1) / cycle_len;
                    total_matched += cycles * cycle_sum;
                    let mut remaining = (1_000_000_000 - round - 1) % cycle_len;
                    for j in i + 1..matches.len() {
                        if remaining < matches[j].0 {
                            break;
                        }
                        total_matched += matches[j].1;
                        remaining -= matches[j].0;
                    }
                    return Some(total_matched.to_string());
                }
            }
        }
    }
    
    Some(total_matched.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(14, 1));
        assert_eq!(result, Some(200.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(14, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(14, 3));
        assert_eq!(result, Some(278388552.to_string()));
    }
}
