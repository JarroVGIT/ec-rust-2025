use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

ec::solution!(12);

/*
Notes:
- Relatively easy to keep part 1 and 2 short with helper functions.
- I want to speed up part 3, it seems to be still inefficient, I think I do way 
  to many allocations?

Part 1:  (67.333µs)
Part 2:  (1.46725ms)
Part 3:  (796.779959ms)
*/

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse_grid(notes: &str) -> HashMap<(isize, isize), u32> {
    let mut grid: HashMap<(isize, isize), u32> = HashMap::new();
    for (y, row) in notes.lines().enumerate() {
        for (x, val) in row.char_indices() {
            grid.insert((x as isize, y as isize), val.to_digit(10).unwrap());
        }
    }
    grid
}

fn get_exploded_barrels(grid: &HashMap<(isize, isize), u32>, start: &[(isize, isize)]) -> HashSet<(isize, isize)> {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.extend(start);
    let mut queue = VecDeque::new();
    queue.extend(start);
    
    while let Some((x,y)) = queue.pop_front() {
        for (dx, dy) in DIRS {
            let new_x = x + dx;
            let new_y = y + dy;
            if visited.contains(&(new_x, new_y)) {
                continue;
            }
            if let Some(&neighbor_val) = grid.get(&(new_x, new_y)) {
                if neighbor_val <= grid.get(&(x,y)).copied().unwrap() {
                    visited.insert((new_x, new_y));
                    queue.push_back((new_x, new_y));
                }
            }
        }
    }
    visited
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let grid = parse_grid(notes);
    let start = &[(0,0)];
    let barrels = get_exploded_barrels(&grid, start);
    Some(barrels.len().to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let grid = parse_grid(notes);
    let y_max = notes.lines().collect_vec().len() as isize - 1;
    let x_max = notes.lines().last().unwrap().len() as isize - 1;
    let start = &[(0,0), (x_max, y_max)];
    let barrels = get_exploded_barrels(&grid, start);
    Some(barrels.len().to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let grid = parse_grid(notes);
    let y_max = notes.lines().collect_vec().len() as isize - 1;
    let x_max = notes.lines().last().unwrap().len() as isize - 1;
    let mut barrels_to_fire: Vec<(isize, isize)> = Vec::with_capacity(3);
    let mut tmp_grid = grid.clone(); 
    for _ in 0..3 {
        let mut max_barrels = 0;
        let mut max_barrels_coords = HashSet::new();
        let mut starting_barrel = (0,0);
        let mut visisted:  HashSet<(isize, isize)> = HashSet::new();
        for y in 0..=y_max {
            for x in 0..=x_max {
                if visisted.contains(&(x,y)) || !tmp_grid.contains_key(&(x,y)) {
                    continue;
                }
                let barrels = get_exploded_barrels(&tmp_grid, &[(x,y)]);
                visisted.extend(barrels.iter());
                if barrels.len() > max_barrels {
                    max_barrels = barrels.len();
                    max_barrels_coords = barrels.clone();
                    starting_barrel = (x,y);
                }
            }
        }
        barrels_to_fire.push(starting_barrel);
        tmp_grid.retain(|k, _| !max_barrels_coords.contains(k));
    }
    let total = get_exploded_barrels(&grid, &barrels_to_fire);
    Some(total.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(12, 1));
        assert_eq!(result, Some(16.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(12, 2));
        assert_eq!(result, Some(58.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(12, 3));
        assert_eq!(result, Some(136.to_string()));
    }
}
