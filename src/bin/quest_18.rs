ec::solution!(18);

/*
Notes: 
- The puzzle was confusingly worded, took me a while before I understood what was being asked.
- Part 1 and 2 were pretty straightforward, though for part 2 there was this sentence "Consider 
  only those test cases for which the last plant was activated.". That sentence made sense in part 3, 
  but not in part 2, and I mistakenly took it for "don't use testcases that end in 0". 
- One small funny stupid bug: I used i32 which wrapped for part 2, made me wonder for a few seconds how
  for heavens sake I could get a negative number as my answer.
- Part 3, well kinda lame, was thinking about using Z3 but was notified about the specific property
  of the notes. The optimum was easily calculated if you just deactivate all plants that have negative
  branches.

Part 1:  (14.375µs)
Part 2:  (4.014125ms)
Part 3:  (3.780708ms)
*/
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut energies = vec![];
    for block in notes.split("\n\n") {
        if block.contains("free") {
            energies.push(1);
        } else {
            // Plant 10 with thickness 46:
            // - branch to Plant 5 with thickness 39
            let (plant, branches)= block.split_once(':').unwrap();
            let plant_thickness = plant.split_ascii_whitespace().flat_map(str::parse::<i64>).last().unwrap();
            let branches: Vec<_> = branches.split_ascii_whitespace().flat_map(str::parse::<i64>).collect();
            let mut energy = 0;
            for branch in branches.chunks(2) {
                let (branch_to_plant, branch_thickness) = (branch[0], branch[1]);
                energy += energies[branch_to_plant as usize - 1] * branch_thickness;
            }
            if energy >= plant_thickness {
                energies.push(energy);
            } else {
                energies.push(0);
            }
        }
    }

    Some(energies.last().copied().unwrap().to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    
    let (blocks, tests) = notes.split_once("\n\n\n").unwrap();
    let mut total = 0;
    for test in tests.lines() {
        let mut energies: Vec<i64> = test.split_ascii_whitespace().flat_map(str::parse).collect();
        // if energies.last().unwrap() == &0 { continue; }
        for block in blocks.split("\n\n") {
            if block.contains("free") {
                continue;
            }
            let (plant, branches)= block.split_once(':').unwrap();
            let plant_thickness = plant.split_ascii_whitespace().flat_map(str::parse::<i64>).last().unwrap();
            let branches: Vec<_> = branches.split_ascii_whitespace().flat_map(str::parse::<i64>).collect();
            let mut energy = 0;
            for branch in branches.chunks(2) {
                let (branch_to_plant, branch_thickness) = (branch[0], branch[1]);
                energy += energies[branch_to_plant as usize - 1] * branch_thickness;
            }
            if energy >= plant_thickness {
                energies.push(energy);
            } else {
                energies.push(0);
            }
        }
        total += energies.last().copied().unwrap();
    }
    Some(total.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {    
    let (blocks, tests) = notes.split_once("\n\n\n").unwrap();
    let mut total = 0;
    
    let mut optimal_energies = vec![];
    for block in blocks.split("\n\n") {
        if block.contains("free") {
             optimal_energies.push(1);
        } else {
            let (plant, branches)= block.split_once(':').unwrap();
            let plant_thickness = plant.split_ascii_whitespace().flat_map(str::parse::<i64>).last().unwrap();
            let branches: Vec<_> = branches.split_ascii_whitespace().flat_map(str::parse::<i64>).collect();
            let mut energy = 0;
            for branch in branches.chunks(2) {
                let (branch_to_plant, branch_thickness) = (branch[0], branch[1]);
                if branch_to_plant < 82 && branch_thickness < 0 {
                    // Visually confirmed; all start plants either have all positive or all negative thickness. 
                    // If they are all negative, they should not be activated to increase the ultimate score.
                    // Kinda lame, no cool trick, just optics and input analysis.
                    optimal_energies[branch_to_plant as usize - 1] = 0;
                }
                energy += optimal_energies[branch_to_plant as usize - 1] * branch_thickness;
            }
            if energy >= plant_thickness {
                optimal_energies.push(energy);
            } else {
                optimal_energies.push(0);
            }
        }
    }
    let optimal = optimal_energies.last().copied().unwrap();
    
    for test in tests.lines() {
        let mut energies: Vec<i64> = test.split_ascii_whitespace().flat_map(str::parse).collect();
        for block in blocks.split("\n\n") {
            if block.contains("free") {
                continue;
            }
            let (plant, branches)= block.split_once(':').unwrap();
            let plant_thickness = plant.split_ascii_whitespace().flat_map(str::parse::<i64>).last().unwrap();
            let branches: Vec<_> = branches.split_ascii_whitespace().flat_map(str::parse::<i64>).collect();
            let mut energy = 0;
            for branch in branches.chunks(2) {
                let (branch_to_plant, branch_thickness) = (branch[0], branch[1]);
                energy += energies[branch_to_plant as usize - 1] * branch_thickness;
            }
            if energy >= plant_thickness {
                energies.push(energy);
            } else {
                energies.push(0);
            }
        }
        if let Some(last) = energies.last().copied() && last > 0 {
            total += optimal - last;
        }
    }

    Some(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(18, 1));
        assert_eq!(result, Some(774.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(18, 2));
        assert_eq!(result, Some(324.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(18, 3));
        assert_eq!(result, None);
    }
}
