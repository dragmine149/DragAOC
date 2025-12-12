use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Device {
    id: u16,
    outputs: Vec<u16>,
}

fn parse_mappings(id: &str, mappings: &mut Vec<String>) -> u16 {
    if let Some(pos) = mappings.iter().position(|i| i == id) {
        return pos as u16;
    }
    mappings.push(id.to_string());
    (mappings.len() - 1) as u16
}

#[aoc_generator(day11)]
fn parse(input: &str) -> (Vec<Device>, Vec<String>) {
    let mut mappings: Vec<String> = vec![];
    let devices = input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let id = parse_mappings(split.next().unwrap(), &mut mappings);
            let outputs = split
                .next()
                .unwrap()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|out| parse_mappings(out, &mut mappings))
                .collect_vec();

            Device { id, outputs }
        })
        .collect::<Vec<Device>>();
    (devices, mappings)
}

fn find_path(nodes: &[Device], previous_id: u16, goal_id: u16) -> usize {
    // println!("{:?}", previous_id);
    nodes
        .iter()
        .find(|device| device.id == previous_id)
        .unwrap_or_else(|| panic!("Failed to find {:?}", previous_id))
        .outputs
        .iter()
        .map(|out| {
            if *out == goal_id {
                return 1;
            }

            find_path(nodes, *out, goal_id)
        })
        .sum()
}

fn find_path_2(
    nodes: &[Device],
    previous_id: u16,
    goal_id: u16,
    cache: &mut HashMap<u16, usize>,
) -> usize {
    if let Some(score) = cache.get(&previous_id) {
        return *score;
    }

    // println!("{:?}", previous_id);
    let res = nodes
        .iter()
        .find(|device| device.id == previous_id)
        .unwrap_or_else(|| panic!("Failed to find {:?}", previous_id))
        .outputs
        .iter()
        .map(|out| {
            if *out == goal_id {
                return 1;
            }

            find_path_2(nodes, *out, goal_id, cache)
        })
        .sum();
    cache.insert(previous_id, res);
    res
}

#[aoc(day11, part1)]
fn part1(input: &(Vec<Device>, Vec<String>)) -> usize {
    // println!("{:?}", input.1);
    let you = input.1.iter().position(|id| id == "you").unwrap() as u16;
    let out = input.1.iter().position(|id| id == "out").unwrap() as u16;
    find_path(&input.0, you, out)
}

#[aoc(day11, part1, cache)]
fn part1_cache(input: &(Vec<Device>, Vec<String>)) -> usize {
    // println!("{:?}", input.1);
    let you = input.1.iter().position(|id| id == "you").unwrap() as u16;
    let out = input.1.iter().position(|id| id == "out").unwrap() as u16;
    let mut cache = HashMap::<u16, usize>::new();
    find_path_2(&input.0, you, out, &mut cache)
}

// After struggling w/ caching: https://www.reddit.com/r/adventofcode/comments/1pjxwnc/comment/nth5r0h/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
#[aoc(day11, part2, reddit)]
fn part2(input: &(Vec<Device>, Vec<String>)) -> usize {
    let mut devices = input.0.to_owned();

    let svr = input.1.iter().position(|id| id == "svr").unwrap() as u16;
    let out = input.1.iter().position(|id| id == "out").unwrap() as u16;
    let dac = input.1.iter().position(|id| id == "dac").unwrap() as u16;
    let fft = input.1.iter().position(|id| id == "fft").unwrap() as u16;
    devices.push(Device {
        id: out,
        outputs: vec![],
    });

    let mut cache = HashMap::<u16, usize>::new();
    // println!("{:?}", input.0);
    let checkpoint_a = find_path_2(&devices, svr, fft, &mut cache);
    // println!("{:?}", checkpoint_a);
    cache.clear();
    let checkpoint_b = find_path_2(&devices, fft, dac, &mut cache);
    // println!("{:?}", checkpoint_b);
    cache.clear();
    let checkpoint_c = find_path_2(&devices, dac, out, &mut cache);
    // println!("{:?}", checkpoint_c);
    checkpoint_a * checkpoint_b * checkpoint_c
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 2);
    }
}
