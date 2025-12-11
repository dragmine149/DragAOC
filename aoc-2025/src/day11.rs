use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
    previous_id: u16,
    goal_id: u16,
    dac: &u16,
    fft: &u16,
    hit_dac: bool,
    hit_fft: bool,
    cache: &mut HashMap<(u16, bool, bool), usize>,
    dac_first: &mut Option<bool>,
) -> usize {
    println!("{:?}", previous_id);
    // let hit_dac = previous_id == *dac;
    // let hit_fft = previous_id == *fft;

    // hopefully assumptions are correct.
    if dac_first.is_none() {
        if previous_id == *dac {
            *dac_first = Some(true);
        } else if previous_id == *fft {
            *dac_first = Some(false);
        }
    }

    if let Some(first) = dac_first {
        if *first {
            if previous_id == *fft && !hit_dac {
                return 0;
            }
        } else {
            if previous_id == *dac && !hit_fft {
                return 0;
            }
        }
    }

    // if previous_id == *fft && !hit_dac {
    //     return 0;
    // }

    // let previous = path.last().unwrap();
    // println!("{:?}", previous);
    let res = nodes
        .iter()
        .find(|device| device.id == previous_id)
        .unwrap()
        .outputs
        .iter()
        .map(|out| {
            let hd = hit_dac || out == dac;
            let hf = hit_fft || out == fft;
            if let Some(score) = cache.get(&(*out, hd, hf)) {
                println!("{:?} {:?} {:?} {:?}", out, hd, hf, score);
                // if hit_dac && hit_fft {
                // return (score.0 + hit_dac as u32, score.1 + hit_fft as u32);
                return *score;
                // }
            }

            if *out == goal_id {
                panic!("end of line 1");
                if let Some(first) = dac_first {
                    if *first && hit_fft {
                        return 1;
                    } else if !*first && hit_dac {
                        return 1;
                    }
                }
                return 0;
            }
            let res = find_path_2(nodes, *out, goal_id, dac, fft, hd, hf, cache, dac_first);
            // println!("out: {:0>2}/{:?}", out, res);
            cache.insert((*out, hd, hf), res);
            // cache.insert(*out, (res, hit_dac || out == dac, hit_fft || out == fft));
            res
        })
        // .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        .sum();
    // cache.insert(*previous, res);
    // (res.0 + hit_dac as u32, res.1 + hit_fft as u32)
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
    println!(
        "[  0  ,   1  ,   2  ,   3  ,   4  ,   5  ,   6  ,   7  ,   8  ,   9  ,  1 0 ,  1 1 ,  1 2 ,  1 3 ]"
    );
    println!("{:?}", input.1);
    let svr = input.1.iter().position(|id| id == "svr").unwrap() as u16;
    let out = input.1.iter().position(|id| id == "out").unwrap() as u16;
    let dac = input.1.iter().position(|id| id == "dac").unwrap() as u16;
    let fft = input.1.iter().position(|id| id == "fft").unwrap() as u16;
    println!(
        "svr: {:?}, out: {:?}, dac: {:?}, fft: {:?}",
        svr, out, dac, fft
    );
    let mut cache = HashMap::<(u16, bool, bool), usize>::new();
    // https://www.reddit.com/r/adventofcode/comments/1pjsol8/2025_day_11_part_2_i_feel_betrayed/
    let mut dac_first: Option<bool> = None;
    let res = find_path_2(
        &input.0,
        svr,
        out,
        &dac,
        &fft,
        false,
        false,
        &mut cache,
        &mut dac_first,
    );
    println!("{:?}", cache);
    println!("{:?}", dac_first);
    println!("{:?}", res);
    res
    // 0
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
