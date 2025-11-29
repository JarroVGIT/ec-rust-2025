use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

ec::solution!(20);

/*
Notes:
- Part 1 was just summing over windows, part 2 was BFS.
- Part 3 was very hard, I was unable to create the rotation logic, so I borrowed that from p88h.
  After that, it was part 2 largely but tracking which rotation we are in.

Part 1:  (20.042µs)
Part 2:  (960.667µs)
Part 3:  (1.231625ms)
*/

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    // Every T next to one another is jumpable.
    // Every T from an even row in uneven column with a corresponding T in the next row.
    // Every T from an uneven row in even column with a corresponding T in the next row.
    let lines = notes
        .lines()
        .map(|l| l.char_indices().collect_vec())
        .collect_vec();
    let hor_neighbours = lines
        .iter()
        .map(|line| {
            line.windows(2)
                .filter(|&w| w[0].1 == 'T' && w[1].1 == 'T')
                .count()
        })
        .sum::<usize>();

    let mut vert_neighbours = 0;

    for (r, line) in lines.iter().enumerate() {
        if r == lines.len() - 1 {
            continue;
        }
        let offset = if r % 2 == 0 { 1 } else { 0 };
        for &(c, val) in line.iter().skip(offset).step_by(2) {
            if val == 'T' && lines[r + 1][c].1 == 'T' {
                vert_neighbours += 1;
            }
        }
    }
    Some((hor_neighbours + vert_neighbours).to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Dijkstra again? Actually, all steps are equal, so BFS.
    let mut start = (0, 0);
    let mut target = (0, 0);
    let lines = notes.lines().map(|l| l.chars().collect_vec()).collect_vec();
    for (y, row) in notes.lines().enumerate() {
        for (x, val) in row.char_indices() {
            match val {
                'S' => start = (x, y),
                'E' => target = (x, y),
                _ => (),
            }
        }
    }
    let width = lines[0].len();
    let height = lines.len();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut result = 0;
    seen.insert(start);
    queue.push_back((start, 0));
    while let Some(((x, y), jumps)) = queue.pop_front()
        && result == 0
    {
        let neighbors = {
            // neighbors are left and right, and
            // up if r.is_even and !c.is_even || !r.is_even and c.is_even else down.
            if (y.is_multiple_of(2) && !x.is_multiple_of(2))
                || (!y.is_multiple_of(2) && x.is_multiple_of(2))
            {
                [(-1, 0), (1, 0), (0, 1)]
            } else {
                [(-1, 0), (1, 0), (0, -1)]
            }
        };
        for (dx, dy) in neighbors {
            let (n_x, n_y) = (x as isize + dx, y as isize + dy);
            if n_x < 0 || n_y < 0 || n_x >= width as isize || n_y >= height as isize {
                continue;
            }
            let (n_x, n_y) = (n_x as usize, n_y as usize);
            if target == (n_x, n_y) {
                result = jumps + 1;
                break;
            }
            if lines[n_y][n_x] == 'T' && seen.insert((n_x, n_y)) {
                queue.push_back(((n_x, n_y), jumps + 1));
            }
        }
    }
    Some(result.to_string())
}

// Borrowed from p88h (https://github.com/p88h/everybody.codes/blob/main/src/e2025/q20.rs)
fn rotate_triangle_120(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    // grid contains triangle pointing down
    // after rotation the top edge becomes the right edge
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid1 = vec![vec!['.'; width]; height];
    let mut new_grid2 = vec![vec!['.'; width]; height];
    for y in 0..height {
        // the row spans from y to width-y, but first & last y cols are already rotated
        let mut e1 = vec![];
        for x in y * 3..width - y * 3 {
            e1.push((x, y));
            // this takes care of center cells really
            new_grid1[y][x] = grid[y][x];
            new_grid2[y][x] = grid[y][x];
        }
        if e1.len() < 2 {
            break;
        }
        let mut e2 = vec![];
        // and last two cells are the right edge
        e2.push(e1.pop().unwrap());
        e2.push(e1.pop().unwrap());
        while e2.len() <= e1.len() + 1 {
            let j = e2.len() - 2;
            let (ex, ey) = e2[j];
            let (fx, fy) = e2[j + 1];
            // walk diagonally down-left
            e2.push((ex - 1, ey + 1));
            e2.push((fx - 1, fy + 1));
        }
        e2.pop(); // last one is extra
        let mut e3 = vec![];
        // and these last two are actually left edge
        e3.push(e2.pop().unwrap());
        e3.push(e2.pop().unwrap());
        while e3.len() < e1.len() {
            let j = e3.len() - 2;
            let (ex, ey) = e3[j];
            let (fx, fy) = e3[j + 1];
            // walk diagonally up-left
            e3.push((ex - 1, ey - 1));
            e3.push((fx - 1, fy - 1));
        }
        e3.pop(); // last one is extra
        // now rotate grid contents between e1,e2,e3
        for i in 0..e1.len() {
            let (x1, y1) = e1[i];
            let (x2, y2) = e2[i];
            let (x3, y3) = e3[i];
            (new_grid1[y2][x2], new_grid1[y3][x3], new_grid1[y1][x1]) =
                (grid[y1][x1], grid[y2][x2], grid[y3][x3]);
            (new_grid2[y3][x3], new_grid2[y1][x1], new_grid2[y2][x2]) =
                (grid[y1][x1], grid[y2][x2], grid[y3][x3]);
        }
    }
    (new_grid1, new_grid2)
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let grid = notes.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = grid[0].len();
    let height = grid.len();

    let extra = rotate_triangle_120(&grid);
    let grids = vec![grid, extra.0, extra.1];
    let mut targets = vec![];
    let mut starts = vec![];

    for (gix, grid) in grids.iter().enumerate() {
        for (y, row) in grid.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                match *val {
                    'S' => starts.push((x, y)),
                    'E' => targets.push((x, y)),
                    _ => (),
                }
            }
        }
    }
    let start: (usize, usize, usize) = (starts[0].0, starts[0].1, 0);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut result = 0;
    seen.insert(start);
    queue.push_back((start, 0));
    while let Some(((x, y, rot), jumps)) = queue.pop_front()
        && result == 0
    {
        let neighbors = {
            // neighbors are left and right, and
            // up if r.is_even and !c.is_even || !r.is_even and c.is_even else down.
            if (y.is_multiple_of(2) && !x.is_multiple_of(2))
                || (!y.is_multiple_of(2) && x.is_multiple_of(2))
            {
                [(-1, 0), (1, 0), (0, 1), (0, 0)]
            } else {
                [(-1, 0), (1, 0), (0, -1), (0, 0)]
            }
        };
        let next_rot = (rot + 1) % 3;
        for (dx, dy) in neighbors {
            let (n_x, n_y) = (x as isize + dx, y as isize + dy);
            if n_x < 0 || n_y < 0 || n_x >= width as isize || n_y >= height as isize {
                continue;
            }
            let (n_x, n_y) = (n_x as usize, n_y as usize);
            if targets[next_rot] == (n_x, n_y) {
                result = jumps + 1;
                break;
            }
            if grids[next_rot][n_y][n_x] == 'T' && seen.insert((n_x, n_y, next_rot)) {
                queue.push_back(((n_x, n_y, next_rot), jumps + 1));
            }
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
        let result = part_one(&read_example_file(20, 1));
        assert_eq!(result, Some(7.to_string()));
    }

    #[test]
    fn test_part_one_4() {
        let result = part_one(&read_example_file(20, 4));
        assert_eq!(result, Some(0.to_string()));
    }

    #[test]
    fn test_part_one_5() {
        let result = part_one(&read_example_file(20, 5));
        assert_eq!(result, Some(0.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(20, 2));
        assert_eq!(result, Some(32.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(20, 3));
        assert_eq!(result, Some(23.to_string()));
    }
}
