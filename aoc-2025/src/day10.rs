use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug)]
pub struct Machine {
    light_diagram: u16,
    button_wirings: Vec<u16>,
    joltage: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct CacheStatus {
    hits: usize,
    refs: usize,
}

/// Does xor to flip lights on/off
fn flip_lights(lights: u16, details: u16) -> u16 {
    lights ^ details
}

fn turn_all_off(
    lights: u16,
    buttons: &[u16],
    last_action: u16,
    depth: u8,
    cache: &mut HashMap<u16, usize>,
    cache_status: &mut CacheStatus,
) -> usize {
    if depth > 7 {
        return u32::MAX as usize;
    }
    cache_status.refs += 1;
    if let Some(score) = cache.get(&lights) {
        cache_status.hits += 1;
        return *score;
    }

    // println!("{:?} {:?}", lights, last_action);
    // if they are all off, we have reached the bottom anyway.
    // If they are all off, number should be 0.
    if lights == 0 {
        return 0;
    }
    // when we just have one flip left.
    if buttons.contains(&lights) {
        return 1;
    }
    let res = buttons
        .iter()
        .filter(|button| **button != last_action)
        .map(|button| {
            let lights = flip_lights(lights, *button);
            turn_all_off(lights, buttons, *button, depth + 1, cache, cache_status) + 1
        })
        .min()
        .unwrap();

    // only cache valid results
    if res < u32::MAX as usize {
        cache.insert(lights, res);
    }
    res
}

fn above_limit(current: &[usize], goal: &[usize]) -> bool {
    current
        .iter()
        .enumerate()
        .any(|(pos, value)| *value > goal[pos])
}

fn byte_to_pos(bytes: &u16) -> Vec<u16> {
    let mut positions: Vec<u16> = vec![];
    for n in 0..15 {
        if bytes >> n & 1 == 1 {
            positions.push(n);
        }
    }
    positions
}

fn joltage_levels(current: Vec<usize>, goal: &[usize], buttons: &[Vec<u16>]) -> usize {
    println!("current: {:?}, goal: {:?}", current, goal);
    if current == goal {
        return 1;
    }
    if above_limit(&current, goal) {
        return u32::MAX as usize;
    }

    buttons
        .iter()
        .map(|button| {
            let mut levels = current.clone();
            println!("{:?}", button);
            button.iter().for_each(|but| levels[*but as usize] += 1);
            println!("{:?}", levels);
            joltage_levels(levels, goal, buttons)
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
            let mut lights = 0b0;
            captures
                .get(1)
                .expect("Failed to get first capture")
                .as_str()
                .chars()
                .enumerate()
                .for_each(|(pos, char)| match char {
                    // '.' => lights &= ,
                    '.' => return,
                    '#' => lights |= 2_u16.pow(pos as u32),
                    _ => panic!("Invalid character in line {:?}", char),
                });

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
                    let mut wirings = 0b0;
                    capture
                        .iter()
                        .skip(1)
                        .next()
                        .unwrap()
                        .unwrap()
                        .as_str()
                        .split(",")
                        .map(|n| n.parse::<u8>().expect("Failed to parse to u8"))
                        .for_each(|p| wirings |= 2_u16.pow(p as u32));
                    wirings
                })
                .collect::<Vec<u16>>();

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
    // input.iter().for_each(|i| println!("{:?}", i));
    // println!(
    //     "input[0]: {:?}",
    //     turn_all_off(&input[0].light_diagram, &input[0].button_wirings, &[], 0)
    // );
    // 0
    input
        .par_iter()
        .map(|i| {
            let mut cache = HashMap::<u16, usize>::new();
            let mut cache_status = CacheStatus::default();
            let res = turn_all_off(
                i.light_diagram,
                &i.button_wirings,
                0,
                0,
                &mut cache,
                &mut cache_status,
            );
            println!(
                "{:?} -> {:?} (hits: {:?}, refs: {:?})",
                i, res, cache_status.hits, cache_status.refs
            );
            res
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> usize {
    input
        // .par_iter()
        .iter()
        .map(|i| {
            let jolts = i
                .button_wirings
                .iter()
                .map(|byte| byte_to_pos(byte))
                .collect_vec();
            let res = joltage_levels(vec![0; i.joltage.len()], &i.joltage, &jolts);
            println!("{:?} -> {:?}", i, res);
            res
        })
        .sum()
}

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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 33);
    }
}
