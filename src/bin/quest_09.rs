use std::collections::{HashMap, HashSet, VecDeque};

use itertools::{Itertools, izip};

/*
Well, that was a tough one today. Part 3 had me scratching my head for a while, finally decided on bruteforce.
For part 1, I determined by hand which one was the child. For part 2 I made the are_parents_child() helper 
that I could've used for part 1 as well, to make it more generic. 
*/

ec::solution!(9);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let seqs = notes.lines().map(|line| line.split_once(':').unwrap().1.chars().collect_vec()).collect_vec();
    let p1 = seqs[0].iter().zip(seqs[2].iter()).filter(|(a,b)| a==b).count();
    let p2 = seqs[1].iter().zip(seqs[2].iter()).filter(|(a,b)| a==b).count();

    Some((p1*p2).to_string())
}

fn are_parents_child(p1: &[char], p2: &[char], ch: &[char]) -> bool {
    for (a,b,c) in izip!(p1, p2, ch) { // yay for Itertools again.
        if !((a==b && a==c) || (a!=b && (a==c || b==c))) {
            return false
        }
    }
    true
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Test all parents- child combinations, assuming there is only one way to fit everybody on a tree. 
    let seqs = notes.lines().map(|line| line.split_once(':').unwrap().1.chars().collect_vec()).collect_vec();
    let mut in_familiy_tree = HashSet::new();
    let mut family_trees: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    'p1: for p1 in 0..seqs.len() {
        for p2 in (p1+1)..seqs.len() {
             for ch in 0..seqs.len() {
                if p2 != ch && p1 != ch && are_parents_child(&seqs[p1], &seqs[p2], &seqs[ch]) {
                    in_familiy_tree.insert(p1); 
                    in_familiy_tree.insert(p2);
                    in_familiy_tree.insert(ch);
                    family_trees.entry((p1, p2)).or_default().push(ch);
                    if in_familiy_tree.len() == seqs.len() {
                        break 'p1;
                    }
                }
            }
        }
    }
    let mut result = 0;

    for ((p1, p2), children) in family_trees {
        for ch in children {
            let s1 = seqs[p1].iter().zip(seqs[ch].iter()).filter(|(a,b)| a==b).count();
            let s2 = seqs[p2].iter().zip(seqs[ch].iter()).filter(|(a,b)| a==b).count();
            result += s1*s2;
        }
    }
    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // Create adjency matrix, for each node just check all the neighbours. Keep global set of checked nodes; if
    // node already present in checked nodes, then it is already part of a checked cluster. 
    let seqs = notes.lines().map(|line| line.split_once(':').unwrap().1.chars().collect_vec()).collect_vec();
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for p1 in 0..seqs.len() {
        for p2 in (p1+1)..seqs.len() {
             for ch in 0..seqs.len() {
                if p2 != ch && p1 != ch && are_parents_child(&seqs[p1], &seqs[p2], &seqs[ch]) {
                    adj.entry(p1).or_default().push(ch);
                    adj.entry(p2).or_default().push(ch);
                    adj.entry(ch).or_default().push(p1);
                    adj.entry(ch).or_default().push(p2);
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut largest_cluster = HashSet::new();

    for &start in adj.keys() {
        if visited.contains(&start) {
            continue;
        }
        
        let mut cur_cluster = HashSet::new();
        let mut queue = VecDeque::from([start]);
        visited.insert(start);
        cur_cluster.insert(start);
        
        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = adj.get(&node) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                        cur_cluster.insert(neighbor);
                    }
                }
            }
        }
        
        if cur_cluster.len() > largest_cluster.len() {
            largest_cluster = cur_cluster;
        }
    }
    // note we were dealing with indexes (0-based), not ID's (1-based)
    let result: usize = largest_cluster.iter().sum::<usize>() + largest_cluster.len();

    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(9, 1));
        assert_eq!(result, Some(414.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(9, 2));
        assert_eq!(result, Some(1245.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(9, 3));
        assert_eq!(result, Some(36.to_string()));
    }
}
