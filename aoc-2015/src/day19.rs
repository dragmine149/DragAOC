use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::BinaryHeap;
use std::{collections::HashSet, ops::Range};

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

fn get_molecules(medician: &str) -> Vec<(String, Range<usize>)> {
    let mut molecules: Vec<(String, Range<usize>)> = vec![];
    let mol = medician.chars().enumerate();
    for (index, char) in mol {
        if char.is_lowercase() && char != 'e' {
            continue;
        }

        if let Some(next) = medician.get(index + 1..index + 2) {
            if next.chars().next().unwrap().is_lowercase() {
                molecules.push((char.to_string() + next, index..index + 2));
            } else {
                molecules.push((char.to_string(), index..index + 1));
            }
        } else {
            molecules.push((char.to_string(), index..index + 1));
        }
    }

    molecules
}

fn generate_medician_while(start: &str, replacements: &[(String, String)], target: &String) -> u64 {
    // let mut currents: Vec<(String, u64)> = vec![];
    let mut heap = BinaryHeap::new();
    heap.push((start.to_string(), 0));
    while let Some(v) = heap.pop() {
        let score = v.1;
        let options = get_molecules(&v.0)
            .par_iter()
            .map(|(molecule, range)| {
                let options = replacements.par_iter().filter(|opt| opt.0 == *molecule);
                // let options = replacements.iter().filter(|opt| opt.0 == *molecule);
                if options.clone().count() == 0 {
                    // println!("{:?} has no options", molecule);
                    return vec![None];
                }
                options
                    .map(|opt| {
                        // println!("Looking at: {:?}", opt);
                        // println!("current: {:?}", current);
                        let mut c = v.0.to_string();
                        c.replace_range(range.start..range.end, &opt.1);
                        // println!("new: {:?}", c);

                        if &c == target {
                            // println!("Found!");
                            return Some(String::new());
                        }
                        if c.len() >= target.len() {
                            // println!("Too big...");
                            return None;
                        }
                        // println!("Again!");
                        // println!("{:?} -> {:?}", c, target);
                        Some(c)
                    })
                    .filter(|opt| opt.is_some())
                    .collect::<Vec<Option<String>>>()
            })
            .flatten()
            .map(|opt| (opt, score + 1))
            .collect::<Vec<(Option<String>, u64)>>();

        let found = options
            .iter()
            .filter(|opt| opt.0.to_owned().unwrap().is_empty())
            .map(|opt| opt.1)
            .min()
            .unwrap_or(u64::MAX);
        if found != u64::MAX {
            return found;
        }

        for opt in options {
            heap.push((opt.0.unwrap(), opt.1));
        }
    }

    0
}

fn generate_medician(
    current: String,
    replacements: &[(String, String)],
    target: &String,
    depth: usize,
) -> u64 {
    if depth > target.len() {
        println!("Exit due to target being big");
        return 0;
    }

    // println!("{:?}", get_molecules(&current));
    // println!("---");
    // println!("{:?}", current);
    if &current == target {
        println!("Exit due to target met");
        return 1;
    }

    1 + get_molecules(&current)
        .par_iter()
        // .iter()
        .map(|(molecule, range)| {
            let options = replacements.par_iter().filter(|opt| opt.0 == *molecule);
            // let options = replacements.iter().filter(|opt| opt.0 == *molecule);
            if options.clone().count() == 0 {
                // println!("{:?} has no options", molecule);
                return 0;
            }
            options
                .map(|opt| {
                    // println!("Looking at: {:?}", opt);
                    // println!("current: {:?}", current);
                    let mut c = current.to_string();
                    c.replace_range(range.start..range.end, &opt.1);
                    // println!("new: {:?}", c);

                    if &c == target {
                        // println!("Found!");
                        return 1;
                    }
                    if c.len() >= target.len() {
                        println!("Too big...");
                        return 0;
                    }
                    // println!("Again!");
                    generate_medician(c, replacements, target, depth + 1)
                })
                .min()
                .unwrap_or(0)
        })
        .min()
        .unwrap_or(0)
}

#[aoc(day19, part2)]
fn part2(input: &(Vec<(String, String)>, String)) -> u64 {
    // println!("{:?}", get_molecules(&input.1));
    generate_medician_while("e", input.0.as_slice(), &input.1)
}

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

    const EXAMPLE_3: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOH
";

    const EXAMPLE_4: &str = "e => H
e => O
H => HO
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_3)), 3);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE_4)), 6);
    }
}
