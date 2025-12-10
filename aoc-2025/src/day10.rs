use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug)]
pub struct Machine {
    light_diagram: u16,
    button_wirings: Vec<u16>,
    joltage: u128,
}

fn turn_all_off(
    lights: u16,
    buttons: &[u16],
    last_action: u16,
    depth: u8,
    cache: &mut HashMap<u16, usize>,
) -> usize {
    if depth > 7 {
        return u32::MAX as usize;
    }
    if let Some(score) = cache.get(&lights) {
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
        .map(|button| turn_all_off(lights ^ button, buttons, *button, depth + 1, cache) + 1)
        .min()
        .unwrap();

    // only cache valid results
    if res < u32::MAX as usize {
        cache.insert(lights, res);
    }
    res
}

fn byte_to_pos(bytes: &u16) -> u128 {
    let mut positions = 0_u128;
    for n in 0..15 {
        if bytes >> n & 1 == 1 {
            positions += 2_u128.pow(n * 9);
        }
    }
    positions
}

fn above_limit(current: &u128, goal: &u128) -> bool {
    // println!();
    for n in (1..11).rev() {
        // println!(
        //     "{:9b} {:9b} (n: {:})",
        //     current >> 9 * n & 0b1_1111_1111,
        //     goal >> 9 * n & 0b1_1111_1111,
        //     n
        // );
        if current >> 9 * n & 0b1_1111_1111 > goal >> 9 * n & 0b1_1111_1111 {
            return true;
        }
        // println!("e");
    }

    false
}

fn joltage_levels(
    current: u128,
    goal: u128,
    buttons: &[u128],
    cache: &mut HashMap<u128, usize>,
    worst: u128,
    depth: u128,
) -> usize {
    // println!("current: {:?}, goal: {:?}", current, goal);
    if depth >= worst {
        println!("Depth limit");
        return u32::MAX as usize;
    }
    if current == goal {
        return 0;
    }
    if above_limit(&current, &goal) {
        // println!("{:128b} ({:})", current, current);
        // println!("{:128b} ({:})", goal, goal);
        // println!();
        return u32::MAX as usize;
    }
    if let Some(score) = cache.get(&current) {
        return *score;
    }

    let res = buttons
        .iter()
        .map(|button| joltage_levels(current + button, goal, buttons, cache, worst, depth + 1) + 1)
        .min()
        .unwrap();
    cache.insert(current, res);
    res
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

            let mut jolts = 0_u128;
            captures
                .get(3)
                .expect("Failed to get jolt capture")
                .as_str()
                .split(",")
                .map(|n| n.parse::<u128>().expect("Failed to parse number"))
                .for_each(|num| {
                    jolts <<= 9;
                    jolts += num;
                });
            // .collect::<Vec<u16>>();

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
    input
        .par_iter()
        .map(|i| {
            // this is creating more overhead in some cases than it is worth, but overall it's like a 100x speedup or smth.
            let mut cache = HashMap::<u16, usize>::new();
            let res = turn_all_off(i.light_diagram, &i.button_wirings, 0, 0, &mut cache);
            // println!("{:?} -> {:?}", i, res);
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
            let buttons = i
                .button_wirings
                .iter()
                .map(|byte| byte_to_pos(byte))
                .collect_vec();
            let mut cache = HashMap::<u128, usize>::new();
            // let worst = i.joltage.iter().sum::<u16>();
            let mut worst = 0;
            for n in 0..10 {
                worst += i.joltage >> 9 * n & 0b1_1111_1111;
            }
            // buttons.iter().for_each(|btn| println!("{:128b}", btn));
            // println!();
            let res = joltage_levels(0, i.joltage, &buttons, &mut cache, worst, 0);
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

    #[test]
    fn byte_addition() {
        let mut num = 0b0101_0000_0111_0000_u16;
        num += 0b0001_0000_0001_0000_u16;
        assert_eq!(num, 0b0110_0000_1000_0000);
        num += 0b0001_0000_0001_0000_u16;
        assert_eq!(num, 0b0111_0000_1001_0000);
        num += 0b0001_0000_0001_0000_u16;
        assert_eq!(num, 0b1000_0000_1010_0000);
    }

    #[test]
    fn byte_value() {
        let mut num = 0b0101_0000_0111_0000_u16;
        num += 0b0001_0000_0001_0000_u16;
        num += 0b0001_0000_0001_0000_u16;
        num += 0b0001_0000_0001_0000_u16;
        // println!("{:#0b}", num);
        assert_eq!(num & 0b1111_0000_0000_0000_u16, 0b1000_0000_0000_0000_u16);
        assert!(num & 0b1111_0000_0000_0000_u16 > 0b0111_0000_0000_0000_u16);
    }
}
