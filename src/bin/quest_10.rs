use std::collections::{HashMap, HashSet};

use itertools::Itertools;

ec::solution!(10);


/*
Well that was difficult. Couldn't have done it without peeking on Reddit :(
Cleaned up the code a lot, moved knight_moves to const, get_next_position was a for-loop
but now a filter_map (just because I like filter_maps now).

The recursion bit was tough, definitely was inspired by other work there (from Reddit). 
*/

type Coord = (i32, i32);
type Sheep = HashSet<Coord>;
type Hides = HashSet<Coord>;

const KNIGHT_MOVES: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (1, -2),
    (2, -1),
    (2, 1),
    (-1, 2),
    (1, 2),
];

fn get_next_positions((cur_x, cur_y): Coord, max_x: i32, max_y: i32) -> Vec<Coord> {
    KNIGHT_MOVES.iter().filter_map(|&(dx, dy)| {
        if cur_x + dx >= 0 && cur_x + dx <= max_x && cur_y + dy >= 0 && cur_y + dy <= max_y {
            Some((cur_x + dx, cur_y + dy))
        } else {
            None
        }
    }).collect()
}


fn parse_grid(grid: &str) -> (Coord, Sheep, Hides, Coord) {
    let grid = grid
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;
    let mut sheep: HashSet<(i32, i32)> = HashSet::new();
    let mut hides: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            match val {
                'S' => {
                    sheep.insert((x as i32, y as i32));
                }
                'D' => {
                    start = (x as i32, y as i32);
                }
                '#' => {
                    hides.insert((x as i32, y as i32));
                }
                _ => (),
            }
        }
    }
    (start, sheep, hides, (max_x, max_y))
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (start, sheep, _, (max_x, max_y)) = parse_grid(notes);
    let steps = if cfg!(test) { 3 } else { 4 };

    let mut visisted = HashSet::new();
    let mut from = vec![start];
    visisted.insert(start);

    for _ in 0..steps {
        let mut next_positions = HashSet::new();
        for (cur_x, cur_y) in from {
            for next_pos in get_next_positions((cur_x, cur_y), max_x, max_y) {
                if !visisted.contains(&next_pos) {
                    next_positions.insert(next_pos);
                }
            }
        }
        visisted.extend(next_positions.iter());
        from = next_positions.into_iter().collect()
    }

    let result = sheep.intersection(&visisted).count();
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (start, mut sheep, hides, (max_x, max_y)) = parse_grid(notes);
    let steps = if cfg!(test) { 3 } else { 20 };

    let mut eaten_sheep = 0;
    let mut dragon_positions = vec![start];
    for _ in 0..steps {
        // create next positions
        let mut next_positions = HashSet::new();
        for &(cur_x, cur_y) in dragon_positions.iter() {
            next_positions.extend(get_next_positions((cur_x, cur_y), max_x, max_y));
        }

        // eat the sheep currently in those positions unless on hides
        let overlap: HashSet<Coord> = next_positions
            .intersection(&sheep)
            .cloned()
            .collect::<HashSet<Coord>>()
            .difference(&hides)
            .cloned()
            .collect();
        eaten_sheep += overlap.len();
        sheep = sheep.difference(&overlap).cloned().collect();

        // move the sheep forward.
        sheep = sheep
            .iter()
            .filter_map(|&(x, y)| if y < max_y { Some((x, y + 1)) } else { None })
            .collect();

        let overlap: HashSet<Coord> = next_positions
            .intersection(&sheep)
            .cloned()
            .collect::<HashSet<Coord>>()
            .difference(&hides)
            .cloned()
            .collect();
        eaten_sheep += overlap.len();
        sheep = sheep.difference(&overlap).cloned().collect();

        dragon_positions = next_positions.iter().cloned().collect();
    }
    Some(eaten_sheep.to_string())
}

fn count_wins(
    dragon: Coord,
    mut sheep: Vec<Coord>,
    dragons_turn: bool,
    cache: &mut HashMap<(Coord, Vec<Coord>, bool), usize>,
    hides: &HashSet<Coord>,
    max_x: i32,
    max_y: i32,
) -> usize {
    sheep.sort_unstable();

    let key = (dragon, sheep.clone(), dragons_turn);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let mut total = 0;

    if dragons_turn {
        for next_pos in get_next_positions(dragon, max_x, max_y) {
            let mut new_sheep = Vec::new();
            for &s in &sheep {
                if s != next_pos || hides.contains(&s) {
                    new_sheep.push(s);
                }
            }

            if new_sheep.is_empty() {
                total += 1;
            } else {
                total += count_wins(next_pos, new_sheep, false, cache, hides, max_x, max_y);
            }
        }
    } else {
        let mut has_used_turn = false;

        for i in 0..sheep.len() {
            let new_pos = (sheep[i].0, sheep[i].1 + 1);

            // sheep escaped, not a win
            if new_pos.1 > max_y {
                has_used_turn = true;
                continue;
            }

            // invalid to move into dragon
            if new_pos == dragon && !hides.contains(&new_pos) {
                continue;
            }

            has_used_turn = true;
            let mut new_sheep = sheep.clone();
            new_sheep[i] = new_pos;
            total += count_wins(dragon, new_sheep, true, cache, hides, max_x, max_y);
        }

        if !has_used_turn {
            // no sheep had a move, dragon goes again
            total += count_wins(dragon, sheep, true, cache, hides, max_x, max_y);
        }
    }

    cache.insert(key, total);
    total
}

pub fn part_three(notes: &str) -> Option<String> {
    let (dragon, sheep, hides, (max_x, max_y)) = parse_grid(notes);
    let sheep = sheep.into_iter().collect();
    let mut cache = HashMap::new();
    let result = count_wins(dragon, sheep, false, &mut cache, &hides, max_x, max_y);
    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(10, 1));
        assert_eq!(result, Some(27.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(10, 2));
        assert_eq!(result, Some(27.to_string()));
    }

    #[test]
    fn test_part_three_3() {
        let result = part_three(&read_example_file(10, 3));
        assert_eq!(result, Some(15.to_string()));
    }

    #[test]
    fn test_part_three_4() {
        let result = part_three(&read_example_file(10, 4));
        assert_eq!(result, Some(8.to_string()));
    }

    #[test]
    fn test_part_three_5() {
        let result = part_three(&read_example_file(10, 5));
        assert_eq!(result, Some(4406.to_string()));
    }
}
