use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

ec::solution!(15);

type Point = (isize, isize);

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn turn(&self, lr: char) -> (Self, Point) {
        match lr {
            'L' => {
                match &self {
                    Self::North => (Self::West, (-1, 0)),
                    Self::East => (Self::North, (0, 1)),
                    Self::South => (Self::East, (1, 0)),
                    Self::West => (Self::South, (0, -1))
                }
            },
            'R' => {
                match &self {
                    Self::North => (Self::East, (1, 0)),
                    Self::East => (Self::South, (0, -1)),
                    Self::South => (Self::West, (-1, 0)),
                    Self::West => (Self::North, (0, 1))
                }
            },
            _ => unreachable!()
        }
    }
}

fn parse_notes(notes: &str) -> Vec<(char, isize)> {
    notes.split(',').map(|v| {
        let c = v.chars().next().unwrap();
        let i = v.chars().skip(1).collect::<String>().parse().unwrap();
        (c, i)
    }).collect()
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let instructions = parse_notes(notes);
    let mut walls = HashSet::new();
    let mut dir = Direction::North;
    let start = (0,0);
    let mut last = (0,0);

    let mut pos= start;
    for (lr, steps) in instructions {
        let (new_dir, delta) = dir.turn(lr);
        dir = new_dir;
        for _ in 0..steps {
            pos = (pos.0 + delta.0, pos.1 + delta.1);
            walls.insert(pos);
            last = pos;
        }
    }

    let min_x = walls.iter().min_by_key(|&&(x,_)| x).unwrap().0 - 1;
    let min_y = walls.iter().min_by_key(|&&(_,y)| y).unwrap().1 - 1;
    let max_x = walls.iter().max_by_key(|&&(x,_)| x).unwrap().0 + 1;
    let max_y = walls.iter().max_by_key(|&&(_,y)| y).unwrap().1 + 1;

    let dirs = [(0,1), (0,-1), (1,0), (-1,0)];
    let mut visited = HashSet::new();
    // Doing Dijkstra, while A* might be better?
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, last)));
    visited.insert(last);
    while let Some(Reverse((steps, cur_pos))) = queue.pop() {
        if cur_pos == start {
            return Some(steps.to_string())
        }
        for dir in dirs {
            let new_pos = (cur_pos.0+dir.0, cur_pos.1+dir.1);
            if !visited.contains(&new_pos) && !walls.contains(&new_pos) 
                && new_pos.0 > min_x && new_pos.0 < max_x 
                && new_pos.1 > min_y && new_pos.1 < max_y {
                visited.insert(new_pos);
                queue.push(Reverse((steps + 1, new_pos)));
            }
        }

    }
    None
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    part_one(notes)
}

fn manhattan_distance(a: Point, b: Point) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_neighbors(p: Point) -> Vec<Point> {
    let mut result = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 || dy != 0 {
                result.push((p.0 + dx, p.1 + dy));
            }
        }
    }
    result
}

// Check if moving from 'from' to 'to' crosses any wall segment
fn valid_move(walls: &Vec<(Point, Point)>, from: Point, to: Point) -> bool {
    let x_min = from.0.min(to.0);
    let x_max = from.0.max(to.0);
    let y_min = from.1.min(to.1);
    let y_max = from.1.max(to.1);

    for &(w1, w2) in walls {
        let wx_min = w1.0.min(w2.0);
        let wx_max = w1.0.max(w2.0);
        let wy_min = w1.1.min(w2.1);
        let wy_max = w1.1.max(w2.1);
        
        // Check if rectangles overlap
        if !(x_min > wx_max || x_max < wx_min || y_min > wy_max || y_max < wy_min) {
            return false;
        }
    }
    true
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let instructions = parse_notes(notes);
    
    // Build walls as line segments and track corners
    let mut walls = vec![];
    let mut corners = vec![];
    let mut dir = Direction::North;
    let start = (0, 0);
    
    let mut pos = start;
    corners.push(pos);
    
    for (lr, steps) in instructions {
        let (new_dir, delta) = dir.turn(lr);
        dir = new_dir;
        
        // Move one step
        pos = (pos.0 + delta.0, pos.1 + delta.1);
        let wall_start = pos;
        
        // Move steps-2 more
        pos = (pos.0 + delta.0 * (steps - 2), pos.1 + delta.1 * (steps - 2));
        let wall_end = pos;
        
        walls.push((wall_start, wall_end));
        
        // Move one more to the corner
        pos = (pos.0 + delta.0, pos.1 + delta.1);
        corners.push(pos);
    }
    
    let end = pos;
    
    // Build points of interest: all neighbors of corners that aren't walls
    let mut points_of_interest = HashSet::new();
    points_of_interest.insert(start);
    points_of_interest.insert(end);
    
    for corner in &corners {
        for neighbor in get_neighbors(*corner) {
            if valid_move(&walls, neighbor, neighbor) {
                points_of_interest.insert(neighbor);
            }
        }
    }
    
    // Now run Dijkstra on this small graph
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();
    
    queue.push(Reverse((0, start)));
    dist.insert(start, 0);
    
    while let Some(Reverse((d, cur))) = queue.pop() {
        if cur == end {
            return Some(d.to_string());
        }
        
        if dist.get(&cur).map_or(false, |&cached| d > cached) {
            continue;
        }
        
        // Check all other points of interest
        for &poi in &points_of_interest {
            if poi != cur && valid_move(&walls, cur, poi) {
                let new_dist = d + manhattan_distance(cur, poi);
                
                if new_dist < *dist.get(&poi).unwrap_or(&isize::MAX) {
                    dist.insert(poi, new_dist);
                    queue.push(Reverse((new_dist, poi)));
                }
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(15, 1));
        assert_eq!(result, Some(6.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(15, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(15, 3));
        assert_eq!(result, None);
    }
}
