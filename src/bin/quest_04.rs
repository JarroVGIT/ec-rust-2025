use itertools::Itertools;

ec::solution!(4);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    // Cogs with length a, b, c, d, e will turns as follows with X turns of a:
    // X * (a/b) * (b/c) * (c/d) * (d/e) = X * (a * b * c * d) / (b * c * d * e)
    // = X * (a / e)
    let mut cogs = notes.lines().map(|a| a.parse::<i32>().unwrap());
    let first = cogs.next().unwrap().to_owned();
    let last = cogs.last().unwrap().to_owned();
    let result = (2025 * first) / last;
    Some(format!("{result}"))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // 10000000000000 = X (a / e)
    // X = 10000000000000 * e / a
    let mut cogs = notes.lines().map(|a| a.parse::<usize>().unwrap());
    let first = cogs.next().unwrap().to_owned();
    let last = cogs.last().unwrap().to_owned();
    // We need div_ceil, because if the outcome would be say 20.3, we actually need 21 turns
    // for the first cog to have a minimum amount of last cog rotations.
    let result = (10000000000000 * last).div_ceil(first);
    Some(format!("{result}"))
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // X * (a/b) * (b/c) * (c/d) * (d/e) = X * (a * b * c * d) / (b * c * d * e)
    // 10
    // 7|14
    // 8|16
    // 6
    // a / b1 * b2 / c1 * c2 / d.
    // Looking at the notes, all numbers are integer division. So when calculating the nominator (right side of a|b)
    // we can check if right / left > 0, if so, then take that number (which is 2, 3 or 4, by the looks of it) and
    // if not, then take 1. Rather unsatisfying because we lean on the properties of the input.

    let above: u128 = notes
        .lines()
        .dropping_back(1)
        .map(|line| {
            if line.contains("|") {
                let l = line.split_once('|').unwrap().0.parse::<u128>().unwrap();
                let r = line.split_once('|').unwrap().1.parse::<u128>().unwrap();
                if r / l > 0 { r / l } else { 1 }
            } else {
                line.parse::<u128>().unwrap()
            }
        })
        .product();

    let below: u128 = notes
        .lines()
        .skip(1)
        .map(|line| {
            if line.contains("|") {
                let l = line.split_once('|').unwrap().0.parse::<u128>().unwrap();
                let r = line.split_once('|').unwrap().1.parse::<u128>().unwrap();
                if l / r > 0 { l / r } else { 1 }
            } else {
                line.parse::<u128>().unwrap()
            }
        })
        .product();

    let result = (100 * above) / below;
    Some(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(4, 1));
        assert_eq!(result, Some("15888".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(4, 2));
        assert_eq!(result, Some("1274509803922".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(4, 3));
        assert_eq!(result, Some("6818".to_string()));
    }

    #[test]
    fn test_part_three_2() {
        let result = part_three(&read_example_file(4, 4));
        assert_eq!(result, Some("400".to_string()));
    }
}
