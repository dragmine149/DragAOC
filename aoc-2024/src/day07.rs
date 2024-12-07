use std::ops::BitAnd;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let info: Vec<&str> = line.split(':').collect();

            let result = info
                .get(0)
                .expect("Failed to get desired result")
                .parse::<u64>()
                .expect("Failed to parse desired result");
            let numbers = info
                .get(1)
                .expect("Failed to get the input numbers")
                .split_whitespace()
                .map(|num| num.parse::<u64>().expect("Failed to parse number"))
                .collect::<Vec<u64>>();
            (result, numbers)
        })
        .collect::<Vec<(u64, Vec<u64>)>>()
}

fn check_if_calculate(input: &(u64, Vec<u64>)) -> bool {
    let numbers = &input.1;
    let mut operators: u16 = 0b0;

    let calculated = loop {
        let mut calculation = 0;
        for (index, number) in numbers.iter().enumerate() {
            // get operator
            let desired_operator = (operators >> index) & 1;
            if desired_operator == 0 {
                calculation += number;
            } else {
                calculation *= number;
            }
        }
        operators += 0b1;

        if calculation == input.0 {
            break true;
        }

        if (operators >> numbers.len()) & 1 == 1 {
            break false;
        }
    };
    calculated
}

#[aoc(day7, part1)]
fn part1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut total = 0;

    for calc in input.iter() {
        println!("Attempting to check: {:?}", calc);
        let calculated = check_if_calculate(calc);
        println!("Calculated: {:?}", calculated);

        if calculated {
            total += calc.0;
        }
    }

    total
}
// #[aoc(day7, part2)]
// fn part2(input: &(Vec<Vec<u8>>, (u8, u8))) -> u16 {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 3749);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(EXAMPLE_1)), 6);
    // }
}
