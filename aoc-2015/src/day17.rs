use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect_vec()
}

fn solve_for_liters(containers: &[u32], liters: u32) -> u32 {
    let mut index: u32 = 0b0;
    let mut count: u32 = 0;
    // println!("{:?}", containers.len());
    let max = 2_u32.pow(containers.len() as u32);
    while index < max {
        // println!("{:?}", index);
        let mut sum = 0;
        for (i, v) in containers.iter().enumerate() {
            if (index >> i) & 1 == 1 {
                sum += v;
                if sum > liters {
                    break;
                }
            }
        }
        // println!("{:?}", use_containers);
        count += if sum == liters { 1 } else { 0 };
        index += 0b1;
    }

    count
}

fn solve_for_min_containers(containers: &[u32], liters: u32) -> u32 {
    let mut index: u32 = 0b0;
    // let mut count: u32 = 0;
    let mut options = vec![];
    // println!("{:?}", containers.len());
    while index < 2_u32.pow(containers.len() as u32) {
        // println!("{:?}", index);
        let mut sum = 0;
        let mut count = 0;
        for (i, v) in containers.iter().enumerate() {
            if (index >> i) & 1 == 1 {
                sum += v;
                count += 1;
            }
            if sum > liters {
                break;
            }
        }
        // println!("{:?}", use_containers);
        if sum == liters {
            options.push(count);
        }
        index += 0b1;
    }

    let min = options
        .iter()
        .min()
        .expect("Failed to get minimum amount of containers");
    options.iter().filter(|o| *o == min).count() as u32
}

#[aoc(day17, part1)]
fn part1(input: &[u32]) -> u32 {
    solve_for_liters(input, 150)
}

#[aoc(day17, part2)]
fn part2(input: &[u32]) -> u32 {
    solve_for_min_containers(input, 150)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "20
15
10
5
5";

    #[test]
    fn part1_example() {
        assert_eq!(solve_for_liters(&parse(EXAMPLE_1), 25), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_for_min_containers(&parse(EXAMPLE_1), 25), 3);
    }
}
