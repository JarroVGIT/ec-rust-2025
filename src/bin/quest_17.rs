use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

ec::solution!(17);
/*
Notes:
- Part 1 was straight forward, I felt kinda proud to make the volcano (0,0), so the formula went from
  `(Xv - Xc) * (Xv - Xc) + (Yv - Yc) * (Yv - Yc) <= R * R` to `Xc^2 + Yc^2 <= R^2`. Didn't really 
  matter in the end though, could've done without it I think as well.
- Part 2 was similar in nature, just some looping and additional constraint on the which nodes to count.
- Part 3 was horrible. I made a resolution to not peak on Reddit, but I had to because I had NO idea how
  to solve it. The problem was my reading comprehension; I thought that when you used nodes twice, you 
  only had to count them once. The example (the one with the lasso form) threw me off and I didn't read 
  the specific instruction right below it; you have to count these tiles twice. From there, it was
  simpler; Dijkstra from start to all nodes below volcano, both left and right. The solution worked well
  for the examples given (all three of them) but was wrong on my notes. That took me two hours before I 
  finally gave up, looked on Reddit and realized that I was to strict in my Dijkstra; the optimal path
  for my notes starts with going left, then right around the volcano. My Dijktra only allowed paths on
  the right side of the volcano, so yeah, bummer.

Not particularly fast but okay-ish:

Part 1:  (123.083µs)
Part 2:  (3.025875ms)
Part 3:  (263.214333ms)
*/

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// Parse the grid, ensuring that @ is at 0,0.
fn parse_notes(notes: &str) -> (HashMap<(i32,i32), u32>, (i32,i32)) {
    let mut volcano = (0,0);
    let mut start = (0,0);
    let mut grid = HashMap::new();
    for (y, row) in notes.lines().enumerate() {
        for (x, val) in row.char_indices() {
            if val == '@' {
                volcano = (x as i32, y as i32)
            } else if val == 'S' {
                start = (x as i32, y as i32);
                grid.insert((x as i32, y as i32), 0);
            } 
            else {
                grid.insert((x as i32, y as i32), val.to_digit(10).unwrap());
            }
        }
    }
    grid = grid.iter().map(|(&(x,y), &v)| {
        let new_x = x - volcano.0;
        let new_y = y - volcano.1;
        ((new_x, new_y), v)
    }).collect();
    start = (start.0 - volcano.0, start.1 - volcano.1);
    (grid, start)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (grid, _) = parse_notes(notes);
    let rsqr = 100;
    
    // (Xv - Xc) * (Xv - Xc) + (Yv - Yc) * (Yv - Yc) <= R * R
    // -> Xc^2 + Yc^2 <= R^2 if Xv = 0
    let result = grid.iter().filter_map(|(&(x,y), &v)| {
        if x.pow(2) + y.pow(2) <= rsqr {
            Some(v)
        } else {
            None
        }
    }).sum::<u32>();
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (grid, _) = parse_notes(notes);
    let max_radius = notes.lines().count() / 2;
    let mut damages = vec![0];
    for r in 1..=max_radius {
        let damage = grid.iter().filter_map(|(&(x,y), &v)| {
            let rsqr = r.pow(2) as i32;
            let prev_rsqr = (r-1).pow(2) as i32;
            if x.pow(2) + y.pow(2) <= rsqr && 
            x.pow(2) + y.pow(2) > prev_rsqr {
                Some(v)
            } else {
                None
            }
        }).sum::<u32>();
        damages.push(damage);
    }
    let max = damages.iter().enumerate().max_by_key(|&(_, &dmg)| dmg).map(|(r, &dmg)| r * dmg as usize).unwrap();
    Some(max.to_string())
}

fn is_destroyed(pos:(i32, i32), round: usize) -> bool {
    pos.0.pow(2) + pos.1.pow(2) <= round.pow(2) as i32
}

// Get all distances from S to all cells on either the right side or the left. 
fn dijkstra(start: (i32, i32), grid: &HashMap<(i32, i32), u32>, right_side: bool, round: usize) -> HashMap<(i32, i32), u32> {
    let mut dist: HashMap<(i32, i32), u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    dist.insert(start, 0);
    heap.push(Reverse((0u32, start)));
    
    while let Some(Reverse((cost, pos))) = heap.pop() {
        if cost > *dist.get(&pos).unwrap_or(&u32::MAX) {
            continue;
        }
        for &(dx, dy) in DIRS.iter() {
            let next_pos = (pos.0+dx, pos.1+dy);
            // The path is allowed to wander into the other halve, but only in top half and not in the bottom half.
            // The bottom half is where we want to connect these two paths (right path and left path).
            if ((right_side && !(next_pos.0 > 0 && next_pos.1 > 0)) || (!right_side && !(next_pos.0 < 0 && next_pos.1 > 0))) 
                && let Some(v) = grid.get(&next_pos) 
                && !is_destroyed(next_pos, round) {
                let next_cost = cost + v;
                if next_cost < *dist.get(&next_pos).unwrap_or(&u32::MAX) {
                    dist.insert(next_pos, next_cost);
                    heap.push(Reverse((next_cost, next_pos)));
                }
            }
        }
    }
    dist
}


#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (grid, start) = parse_notes(notes);
    let max_radius = notes.lines().count() / 2;
    let mut loops = vec![];
    for round in 1..max_radius {
        let max_time = 30 * (round as u32 + 1); // Note: path must be smaller than this, not equal.
        let right_paths = dijkstra(start, &grid, true, round);
        let left_paths = dijkstra(start, &grid, false, round);

        for y in (round)..max_radius {
            let connection = (0, y as i32);
            //they both count the connection tile, so subtract once.
            let total_cost = right_paths.get(&connection).unwrap_or(&(u32::MAX / 3)) + 
                left_paths.get(&connection).unwrap_or(&(u32::MAX / 3)) - grid.get(&connection).unwrap_or(&0); 
            if total_cost < max_time {
                loops.push(round * total_cost as usize);
            }
        }
    }
    let result = loops.iter().min().copied().unwrap();
    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(17, 1));
        assert_eq!(result, Some(1573.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(17, 2));
        assert_eq!(result, Some(1090.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(17, 3));
        assert_eq!(result, Some(3180.to_string()));
    }

    #[test]    
    fn test_part_three_2() {
        let result = part_three(&read_example_file(17, 4));
        assert_eq!(result, Some(330.to_string()));
    }

    #[test]    
    fn test_part_three_3() {
        let result = part_three(&read_example_file(17, 5));
        assert_eq!(result, Some(592.to_string()));
    }

}
