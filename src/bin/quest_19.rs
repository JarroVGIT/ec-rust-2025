use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

/*
Notes:
- Started with simple Dijkstra, most time was spent on one-off-errors in flap-cost-calculations.
- For part 2, I split part 1 in multiple helpers (parse, dijkstra, build adjacency list, etc), 
  was pretty proud of finding the `partition_point()` trick. In hindsight I guess I could've
  grouped all openings in a HashMap keyed by their x value. 
- Part 3 runs in under a second, so I am pretty sure this is a very ineffective way to solve
  this problem (using Dijkstra), but overall happy.
- Writing this down makes me realize that I think you can just take the min/max Y of each opening, 
  and not all of them (like I did). 

Part 1:  (37.084µs)
Part 2:  (276.084µs)
Part 3:  (986.390917ms)
*/


ec::solution!(19);

fn parse_notes(notes: &str) -> Vec<(i32, i32, i32)> {
    notes
        .lines()
        .map(|l| {
            let mut nums = l.split(',');
            let x = nums.next().unwrap().parse::<i32>().unwrap();
            let y = nums.next().unwrap().parse::<i32>().unwrap();
            let len = nums.next().unwrap().parse::<i32>().unwrap();
            (x, y, len)
        })
        .collect_vec()
}

// Build an adjency list with cost, then Dijkstra it.
// Accessible tiles are if col+row is even, and if horizontal => vertical distance (you
// can't increase more than you can move to the right).
// Cost is (hori_dis / 2 + adjustment) + (vert_dis / 2).
// Adjustment is 1 if target is higher, else 0 (see Excel).
// Returns point -> vec<(point, cost)>
fn build_adjacency_list(openings: &[(i32, i32, i32)]) -> HashMap<(i32, i32), Vec<(i32, i32, i32)>> {
    let mut adjacency: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    let mut openings = openings.to_vec();
    openings.insert(0, (0,0,1));
    
    for opening_idx in 0..openings.len() - 1 {
        let (open_x, open_y, open_len) = openings[opening_idx];
        let current_value = openings[opening_idx].0;
    
        // Find where the current value's run ends
        let next_start = openings[opening_idx..].partition_point(|x| x.0 == current_value) + opening_idx;
        
        if next_start >= openings.len() {
            continue; // there are no walls after this one anymore
        }
        let next_value = openings[next_start].0;
        let next_end = openings[next_start..].partition_point(|x| x.0 == next_value) + next_start;
        let slice = &openings[next_start..next_end];      
        for &(next_x, next_y, next_len) in slice {
            for cur_y in open_y..open_y + open_len {
                if (open_x + cur_y) % 2 != 0 {
                    continue; // this tile is unreachable, skip.
                }
                for n_y in next_y..next_y + next_len {
                    if (next_x + n_y) % 2 != 0 {
                        continue; // next tile is unreachable, skip.
                    }
                    if (next_x - open_x).abs() >= (n_y - cur_y).abs() {
                        let adjust = if n_y < cur_y { 0 } else { 1 }; 
                        let cost = ((next_x - open_x + adjust) / 2) + ((n_y - cur_y) / 2);
                        adjacency
                            .entry((open_x, cur_y))
                            .or_default()
                            .push((next_x, n_y, cost));
                    }
                }
            }
        }
        
    }
    adjacency
}

// Returns all shortest paths
fn dijkstra(adjacency: &HashMap<(i32, i32), Vec<(i32, i32, i32)>>) -> HashMap<(i32, i32), i32> {
    let start = (0,0);
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(Reverse((0i32, start)));

    while let Some(Reverse((cur_flaps, (cur_x, cur_y)))) = heap.pop() {
        if cur_flaps > *dist.get(&(cur_x, cur_y)).unwrap_or(&i32::MAX) {
            continue;
        }
        if adjacency.contains_key(&(cur_x, cur_y)) {
            for &(n_x, n_y, flaps) in adjacency[&(cur_x, cur_y)].iter() {
                let next_flaps = cur_flaps + flaps;
                if next_flaps < *dist.get(&(n_x, n_y)).unwrap_or(&i32::MAX) {
                    dist.insert((n_x, n_y), next_flaps);
                    heap.push(Reverse((next_flaps, (n_x, n_y))));
                }
            }
        }
    }
    dist
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let start = (0, 0);
    let openings = parse_notes(notes);
    let adjacency = build_adjacency_list(&openings);
    let dist = dijkstra(&adjacency);
    let last_x = openings.last().unwrap().0;
    let result = dist
        .iter()
        .filter_map(|((x, y), v)| if *x == last_x { Some(v) } else { None })
        .min()
        .copied()
        .unwrap();
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    part_one(notes)
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    part_one(notes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(19, 1));
        assert_eq!(result, Some(24.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(19, 2));
        assert_eq!(result, Some(22.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(19, 3));
        assert_eq!(result, None);
    }
}
