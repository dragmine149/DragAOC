use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

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
        .unwrap()
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
    path: Vec<u16>,
    goal_id: u16,
    dac: &u16,
    fft: &u16,
    // cache: &mut HashMap<u16, usize>,
) -> usize {
    let previous = path.last().unwrap();
    println!("{:?}", previous);
    let res = nodes
        .iter()
        .find(|device| device.id == *previous)
        .unwrap()
        .outputs
        .iter()
        .map(|out| {
            // if let Some(score) = cache.get(out) {
            //     if score == 2 {}
            //     return *score;
            // }

            println!("{:?} {:?}", path, out);
            if *out == goal_id {
                println!("GOAL!");
                if path.contains(dac) && path.contains(fft) {
                    // cache.insert(*out, 2);
                    // cache.insert(*previous, 1);
                    return 1;
                } else {
                    // cache.insert(*out, 1);
                    return 0;
                }
            }

            let mut pathing = path.clone();
            pathing.push(*out);
            find_path_2(nodes, pathing, goal_id, dac, fft)
        })
        .sum();
    // cache.insert(*previous, res);
    res
}

#[aoc(day11, part1)]
fn part1(input: &(Vec<Device>, Vec<String>)) -> usize {
    println!("{:?}", input.1);
    let you = input.1.iter().position(|id| id == "you").unwrap() as u16;
    let out = input.1.iter().position(|id| id == "out").unwrap() as u16;
    find_path(&input.0, you, out)
}

#[aoc(day11, part2)]
fn part2(input: &(Vec<Device>, Vec<String>)) -> usize {
    println!("{:?}", input.1);
    let svr = input.1.iter().position(|id| id == "svr").unwrap() as u16;
    let out = input.1.iter().position(|id| id == "out").unwrap() as u16;
    let dac = input.1.iter().position(|id| id == "dac").unwrap() as u16;
    let fft = input.1.iter().position(|id| id == "fft").unwrap() as u16;
    // let mut cache = HashMap::<u16, usize>::new();
    let res = find_path_2(&input.0, vec![svr], out, &dac, &fft);
    // println!("{:?}", cache);
    res
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
