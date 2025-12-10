use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug)]
pub struct Machine {
    light_diagram: Vec<bool>,
    button_wirings: Vec<Vec<u8>>,
    joltage: Vec<usize>,
}

fn flip_lights(lights: &[bool], details: &[u8]) -> Vec<bool> {
    let mut lights = lights.to_owned();
    details
        .iter()
        .for_each(|detail| lights[*detail as usize] = !lights[*detail as usize]);
    lights
}
fn lights_off(lights: &[bool]) -> bool {
    !lights.iter().any(|l| *l)
}
fn lights_on(lights: &[bool]) -> Vec<u8> {
    lights
        .iter()
        .enumerate()
        .filter(|light| *light.1)
        .map(|light| light.0 as u8)
        .collect_vec()
}

fn turn_all_off(lights: &[bool], buttons: &[Vec<u8>], last_action: &[u8], depth: u8) -> usize {
    if depth > 10 {
        return u32::MAX as usize;
    }
    // println!("{:?} {:?}", lights, last_action);
    // if they are all off, we have reached the bottom anyway.
    if lights_off(&lights) {
        return 0;
    }
    // when we just have one flip left.
    if buttons.contains(&lights_on(&lights)) {
        return 1;
    }
    buttons
        .iter()
        .filter(|button| *button != last_action)
        .map(|button| {
            let lights = flip_lights(&lights, button);
            turn_all_off(&lights, buttons, button, depth + 1) + 1
        })
        .min()
        .unwrap()
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    // one regex to capture all the outside one-group only stuff
    let regex = Regex::new(r"\[((?:\.|#)*)\](.*)\{((?:\d|,)*)\}").unwrap();
    // second regex to capture the inside stuff.
    let regex2 = Regex::new(r"(?:\(((?:\d|,)+)\))+").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).expect("Failed to do regex");
            // println!("{:?}", captures);
            let lights = captures
                .get(1)
                .expect("Failed to get first capture")
                .as_str()
                .chars()
                .map(|char| match char {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Invalid character in line {:?}", char),
                })
                .collect::<Vec<bool>>();
            let jolts = captures
                .get(3)
                .expect("Failed to get jolt capture")
                .as_str()
                .split(",")
                .map(|n| n.parse::<usize>().expect("Failed to parse number"))
                .collect::<Vec<usize>>();

            let captures2 = regex2.captures_iter(captures.get(2).unwrap().as_str());
            // println!("{:?}", captures2);
            // println!();
            let wirings = captures2
                .map(|capture| {
                    // println!("{:?}", capture);
                    capture
                        .iter()
                        .skip(1)
                        .next()
                        .unwrap()
                        .unwrap()
                        .as_str()
                        .split(",")
                        .map(|n| n.parse::<u8>().expect("Failed to parse to u8"))
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<Vec<u8>>>();

            Machine {
                light_diagram: lights,
                button_wirings: wirings,
                joltage: jolts,
            }
        })
        .collect::<Vec<Machine>>()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    input.iter().for_each(|i| println!("{:?}", i));
    // println!(
    //     "input[0]: {:?}",
    //     turn_all_off(&input[0].light_diagram, &input[0].button_wirings, &[], 0)
    // );
    // 0
    input
        .par_iter()
        .map(|i| turn_all_off(&i.light_diagram, &i.button_wirings, &[], 0))
        .sum()
}

// #[aoc(day10, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 7);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
