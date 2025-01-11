use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<(String, String)>, String) {
    let mut replacements: Vec<(String, String)> = vec![];
    let mut medician: String = String::default();

    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        if line.contains("=>") {
            let mut details = line.split(" => ");
            let first = details.next().unwrap().to_string();
            let second = details.next().unwrap().to_string();
            replacements.push((first, second));
        } else {
            medician = line.to_string();
        }
    });

    (replacements, medician)
}

fn process_string_from(
    medician: &String,
    from: usize,
    replacements: &[(String, String)],
    molecules: &mut HashSet<String>,
) {
    if from > medician.len() || from + 1 > medician.len() {
        return;
    }

    // println!("{:?}", from);
    // println!("{:?}", medician);

    let char = medician.get(from..from + 1).unwrap();
    if char.chars().all(|c| c.is_lowercase()) {
        // skip the check as science says that molecules can't start with a lower case
        process_string_from(medician, from + 1, replacements, molecules);
        return;
    }

    // println!("{:?}", char);
    let possible = replacements.iter().filter(|opt| opt.0 == char);
    let possible_count = possible.clone().count();
    if possible_count > 0 {
        possible.for_each(|opt| {
            let mut med = medician.to_owned();
            med.replace_range(from..from + 1, opt.1.as_str());
            // println!("{:?}", med);
            molecules.insert(med);
        });
    }

    if from + 2 > medician.len() {
        return;
    }

    let char = medician.get(from..from + 2).unwrap();
    // println!("{:?}", char);
    let possible = replacements.iter().filter(|opt| opt.0 == char);
    let possible_count = possible.clone().count();
    if possible_count > 0 {
        possible.for_each(|opt| {
            let mut med = medician.to_owned();
            med.replace_range(from..from + 2, opt.1.as_str());
            // println!("{:?}", med);
            molecules.insert(med);
        });
    }

    process_string_from(medician, from + 1, replacements, molecules);
}

#[aoc(day19, part1)]
fn part1(input: &(Vec<(String, String)>, String)) -> u64 {
    // println!("{:?}", input);
    let mut molecules: HashSet<String> = HashSet::new();

    process_string_from(&input.1, 0, input.0.as_slice(), &mut molecules);
    // println!("{:?}", molecules);
    molecules.len() as u64
}

// #[aoc(day19, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "H => HO
H => OH
O => HH

HOH
";

    const EXAMPLE_2: &str = "H => HO
H => OH
O => HH

HOHOHO
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 7);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
