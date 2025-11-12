use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

ec::solution!(7);

/*
Notes:
- Immediately created parse_notes() helper, which came in handy in all three parts.
- Initially I created an inline closure for the is_valid name, but making it a
  seperate function made the code cleaner. Also I initially had it consume the String,
  but a Vec<String>.iter() yields &String, so changed the signature to accept &str. Move semantics
  and iterators still throw me off balance sometimes, which becomes quickly apparent when
  the T !Copy.
- I did part three with recursion, but this is not tail recursion. Another idea I had was going
  backwards: tracking for each letter how many names you can make from that position. Something
  like using a HashMap<(char, usize), usize>, where (char,usize) is "from a string of len <usize>
  ending letter <char>, you can make <value> valid names". Start at 11 for all rule sets, e.g.
  ('a', 11) = 0 (because 0 additional names can be made). Then ('r', 10) is the direct next
  possiblities plus hashmap[(c, 10+1)] for each c in rules. Once you have this hashmap (upto
  length of shortest name in names), each name is a O(1) lookup.
  The cool thing is that compared to recursion, you calculate everything once, while with (the
  current) recursion implementation you start from scratch with every name.
- I didn't have the smarts to think about eliminating all names that have substrings in names.
  Meaning, if you have names "abc" and "abcd", you don't have to check "abcd" if abc is valid.
  Because if it is valid, and "abcd" is valid, you will count all possible names anyway. Had to
  get this from Reddit unfortunately :(
- Come to think of it, I really don't need recursion at all, this is a DFS problem so we could
  just create a queue with nodes to process, starting from the last_letter + len. Add to a queue
  and keep track of a count. I can't think of a way to make it tail-recursive, so it might blow
  the stack with bigger inputs. A DFS approach would work well I guess. Though I think I like the
  HashMap approach better.
- Okay couldn't shake it, I tried the Hashmap approach and it was WILDLY faster (30ms vs 50us).
  The DFS approach was faster to whip up but also took 30ms, basically the same as the original.
*/

fn parse_notes(notes: &str) -> (Vec<String>, HashMap<char, Vec<char>>) {
    let mut lines = notes.lines();
    let names = lines
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect_vec();
    lines.next();
    let rules: HashMap<_, _> = lines
        .map(|line| {
            let (ch, rules) = line.split_once(" > ").unwrap();
            let ch = ch.chars().next().unwrap();
            let rules = rules.chars().filter(|c| *c != ',').collect_vec();
            (ch, rules)
        })
        .collect();
    (names, rules)
}

fn is_valid_name(name: &str, rules: &HashMap<char, Vec<char>>) -> bool {
    let chars: Vec<_> = name.chars().collect();
    chars
        .windows(2)
        .all(|w| rules.get(&w[0]).map_or(true, |rule| rule.contains(&w[1])))
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (names, rules) = parse_notes(notes);
    names.into_iter().find(|name| is_valid_name(name, &rules))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (names, rules) = parse_notes(notes);
    let sum: usize = names
        .iter()
        .enumerate()
        .filter(|(_, name)| is_valid_name(name, &rules))
        .map(|(idx, _)| idx + 1)
        .sum();
    Some(sum.to_string())
}

fn get_possible_names(letter: char, len: usize, rules: &HashMap<char, Vec<char>>) -> usize {
    if len >= 11 {
        return 0;
    }
    let Some(rule_letters) = rules.get(&letter) else {
        return 0;
    };

    // Can we make a valid name when we add a letter? Then count all rules. Else 0.
    let mut names = 0;
    for rule in rule_letters {
        names += ((len + 1 >= 7) as usize) + get_possible_names(*rule, len + 1, rules);
    }
    names
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (names, rules) = parse_notes(notes);

    let names = names
        .iter()
        .filter(|name| {
            is_valid_name(name, &rules)
                && names
                    .iter()
                    .all(|other| *name == other || !name.starts_with(other))
        })
        .collect_vec();
    let mut result = 0;
    for name in names {
        let last = name.chars().last().unwrap();
        result += get_possible_names(last, name.len(), &rules);
    }
    Some(result.to_string())
}

fn get_names_count_dfs(letter: char, len: usize, rules: &HashMap<char, Vec<char>>) -> usize {
    let mut queue = VecDeque::new();
    let mut count = 0;
    queue.push_back((letter, len));
    while let Some((ch, len)) = queue.pop_front() {
        if len >= 11 { continue; }
        // nice, let-else, cute.
        let Some(rules) = rules.get(&ch) else { continue; };    
        if len >= 6 { count += rules.len() }      
        for &rule in rules {
            queue.push_back((rule, len + 1));
        }
    }
    count
}

pub fn _part_three_dfs(notes: &str) -> Option<String> {
    let (names, rules) = parse_notes(notes);
    let total: usize = names
        .iter()
        .filter(|name| { // Check for validity and if none of the other names is a prefix.
            is_valid_name(name, &rules)
                && names
                    .iter()
                    .all(|other| *name == other || !name.starts_with(other))
        })
        .map(|name| { // This should now be a O(1) lookup.
            let last_char = name.chars().last().unwrap();
            get_names_count_dfs(last_char, name.len(), &rules)
        })
        .sum();
    Some(total.to_string())
}

pub fn _part_three_hashmap(notes: &str) -> Option<String> {
    let (names, rules) = parse_notes(notes);

    let shortest_name_len = names.iter().map(String::len).min().unwrap();

    let mut counts = HashMap::new();

    // Build the hashmap backwards. 
    for len in (shortest_name_len..=11).rev() {
        for (&ch, next_chars) in rules.iter() {
            let direct = if len >= 6 && len < 11 {
                next_chars.len()
            } else {
                0
            };
            let indirect: usize = next_chars
                .iter()
                .map(|&next| counts.get(&(next, len + 1)).copied().unwrap_or(0))
                .sum();
            counts.insert((ch, len), direct + indirect);
        }
    }

    let total: usize = names
        .iter()
        .filter(|name| { // Check for validity and if none of the other names is a prefix.
            is_valid_name(name, &rules)
                && names
                    .iter()
                    .all(|other| *name == other || !name.starts_with(other))
        })
        .map(|prefix| { // This should now be a O(1) lookup.
            let last_char = prefix.chars().last().unwrap();
            counts.get(&(last_char, prefix.len())).copied().unwrap_or_default()
        })
        .sum();

    Some(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(7, 1));
        assert_eq!(result, Some("Oroneth".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(7, 2));
        assert_eq!(result, Some("23".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(7, 3));
        assert_eq!(result, Some("25".to_string()));
    }

    #[test]
    fn test_part_three_2() {
        let result = part_three(&read_example_file(7, 4));
        assert_eq!(result, Some("1154".to_string()));
    }
}
