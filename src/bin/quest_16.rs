ec::solution!(16);
/*
Notes:
- Part 1 was easy, but I still overcomplicated things. You can just divide 90 by each number in the
  spell and add those up (O(n)), while I evaluate the entire spell per column (O(n*m)). I initially
  thought I had to find the LCM and that would be part 2 and 3, but that was a prime example of
  premature optimization.
- Part 2 was interesting, I was pretty proud that I found a way quickly. However, I allocate every
  iteration which I do not like, though it is still pretty fast (30us).
- Part 3 was super iteresting. It took me a second to understand the assignment. First we have to find
  the spell, then the LCM (I figured) of the elements of the spell, because the wall would repeat after
  that length. However, the spell was 30 elements long with a lot > 500, and the LCM was too big to fit in a
  u128, which made me realize this was not the right approach. Then I figured I'd try binary searching the
  solution; trying a wall length in the middle of an upper and lower bound. Calculate the blocks required,
  if that wall lenght require more blocks than the assignment, try again with lower-middle, otherwise try
  again with middle-upper as bounds. This resulted in an extremely painful off-by-one error search that still
  hurts my brain a little. Resorted to Reddit, saw others had the same idea (binary search) and fixed mine
  with how they did it.

Part 1:  (9.958µs)
Part 2:  (36.292µs)
Part 3:  (5.653667ms)
*/

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let numbers: Vec<usize> = notes.split(',').flat_map(|n| n.parse()).collect();
    let cols = 90;
    // let blocks = (1..=cols).map(|c| {
    //     numbers.iter().filter(|&n| c % *n == 0).count()
    // }).sum::<usize>();
    let blocks = numbers.iter().map(|s| 90 / *s).sum::<usize>();
    Some(blocks.to_string())
}

fn get_spell(mut wall: Vec<isize>) -> Vec<usize> {
    let mut spell = vec![];
    loop {
        if wall.iter().sum::<isize>() == 0 {
            break;
        }
        for i in 1isize.. {
            let new_blocks: Vec<isize> = (1..)
                .zip(wall.iter())
                .map(
                    |(col, &blocks)| {
                        if col % i == 0 { blocks - 1 } else { blocks }
                    },
                )
                .collect();
            if new_blocks.iter().all(|n| *n >= 0) {
                wall = new_blocks;
                spell.push(i);
                break;
            }
        }
    }
    spell.iter().map(|c| *c as usize).collect()
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let numbers: Vec<isize> = notes.split(',').flat_map(|n| n.parse()).collect();
    let spell = get_spell(numbers);
    let res: usize = spell.iter().product();
    Some(res.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let numbers: Vec<isize> = notes.split(',').flat_map(|n| n.parse()).collect();
    let spell = get_spell(numbers);

    let mut upper: usize = 202520252025000;
    let mut lower = 0;

    while lower < upper {
        let middle = (lower + upper + 1) / 2;
        let sum: usize = spell.iter().map(|n| middle / n).sum();

        if sum > 202520252025000 {
            upper = middle - 1;
        } else {
            lower = middle;
        }
    }
    Some(lower.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(16, 1));
        assert_eq!(result, Some(193.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(16, 2));
        assert_eq!(result, Some(270.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(16, 3));
        assert_eq!(result, Some(94439495762954usize.to_string()));
    }
}
