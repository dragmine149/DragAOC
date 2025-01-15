use crate::utils::{Incrementer, Length};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

impl Operation {
    fn wrap(&self, part2: bool) -> Self {
        match self {
            Operation::Addition => Operation::Multiplication,
            Operation::Multiplication => {
                if part2 {
                    Operation::Concatenation
                } else {
                    Operation::Addition
                }
            }
            Operation::Concatenation => Operation::Addition,
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let info: Vec<&str> = line.split(':').collect();

            // split input up into calculations and the desired result

            let result = info
                .first()
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

// Using binary, check if we can do a calculation.
fn check_if_calculate(input: &(u64, Vec<u64>)) -> bool {
    let numbers = &input.1;
    let mut operators: u16 = 0b0; // Bit 0: addition, Bit 1: Multiplication

    loop {
        let mut calculation = 0;
        for (index, number) in numbers.iter().enumerate() {
            // get operator
            let desired_operator = (operators >> index) & 1; // get the bit at the specified index
            if desired_operator == 0 {
                calculation += number;
            } else {
                calculation *= number;
            }
        }
        operators += 0b1; // increment by 1, which also wraps the input around

        if calculation == input.0 {
            break true;
        }

        // checks if the last big (biggest bit) is one, aka we have been though everything and found nothing.
        if (operators >> numbers.len()) & 1 == 1 {
            break false;
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    let mut total = 0;

    for calc in input.iter() {
        // println!("Attempting to check: {:?}", calc);
        let calculated = check_if_calculate(calc);
        // println!("Calculated: {:?}", calculated);

        if calculated {
            total += calc.0;
        }
    }

    total
}

// The same as the previous check_if_calculate but uses vector wrapping and support for the third operator
fn check_if_calculate_2(input: &(u64, Vec<u64>), part2: bool) -> bool {
    let numbers = &input.1;
    // let mut operators: Vec<u8> = vec![0; numbers.len() + 2];
    let mut operators: Incrementer<Operation> =
        Incrementer::new(Operation::Addition, numbers.len());

    loop {
        // println!("---");
        let mut calculation = 0;
        for (index, number) in numbers.iter().enumerate() {
            if calculation > input.0 {
                break;
            }

            // get operator
            let desired_operator = operators[index];
            if desired_operator == Operation::Addition {
                // println!("{:?} + {:?}", calculation, number);
                calculation += number;
            } else if desired_operator == Operation::Concatenation {
                // println!("{:?} || {:?}", calculation, number);
                // Does 10^(the ceiling of how many times 10 goes into the right hand side number).
                calculation =
                    calculation * 10_u64.pow(((number + 1) as f64).log10().ceil() as u32) + number;
                // Self::Concatenate => lhs * 10_u64.pow(((rhs + 1) as f64).log10().ceil() as u32) + rhs,
            } else if desired_operator == Operation::Multiplication {
                // println!("{:?} * {:?}", calculation, number);
                calculation *= number;
            } else {
                panic!("AAA");
            }
        }

        let end = operators.wrap(|v: &Operation| v.wrap(part2));

        if calculation == input.0 {
            break true;
        }
        if end == operators.len() {
            break false;
        }
    }
}

#[allow(dead_code)]
fn check_if_calculate_3(goal: u64, numbers: &[u64], part2: bool) -> bool {
    // println!("Goal: {:?}. Numbers: {:?}.", goal, numbers);
    let mut operations: Incrementer<Operation> =
        Incrementer::new(Operation::Addition, numbers.len() - 1);
    let mut cache: HashMap<Vec<Operation>, u64> = HashMap::new();
    let mut count = numbers.len();
    loop {
        let mut calculation = 0;
        let mut iter = numbers.iter().enumerate();
        calculation += iter.next().unwrap().1;
        // iter.next();

        // println!(
        //     "Cache: {:?}",
        //     &operations[0..operations.len().saturating_sub(count)]
        // );
        let cache_value = cache
            .get(&operations[0..operations.len().saturating_sub(count)])
            .unwrap_or(&u64::MAX);
        if *cache_value != u64::MAX {
            // match operations[operations.len().saturating_sub(count)] {
            //     Operation::Addition => calculation = *cache_value,
            //     Operation::Multiplication => calculation = *cache_value,
            //     Operation::Concatenation => {
            //         calculation = cache_value.number_length() as u64 + cache_value
            //     }
            // }
            calculation = *cache_value;

            iter.nth(operations.len().saturating_sub(count) - 1);
        }
        // println!("{:?}", calculation);
        // println!("{:?}", operations);

        for (index, number) in iter {
            // println!("op: {:?}", operations[index - 1]);
            // println!("idx: {:?}, num: {:?}", index, number);
            match operations[index - 1] {
                Operation::Addition => calculation += number,
                Operation::Multiplication => calculation *= number,
                Operation::Concatenation => {
                    calculation = calculation * number.number_length() as u64 + number
                }
            }
            // println!("{:?}", calculation);
            if calculation > goal {
                break;
            }
            if index != operations.len() {
                // prevents having "useless ones" included
                cache.insert(operations[0..index].to_vec(), calculation);
            }
        }

        // println!("{:?}", calculation);
        // println!("{:?}", cache);

        if calculation == goal {
            break true;
        }
        count = operations.wrap(|v| v.wrap(part2));

        if count == operations.len() + 1 {
            break false;
        }
    }
}

#[aoc(day7, part2)]
fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        // .iter()
        .map(|calc| {
            if check_if_calculate_2(calc, true) {
                // if check_if_calculate_3(calc.0, &calc.1, true) {
                calc.0
            } else {
                0
            }
        })
        .sum()
}

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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 11387);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse("190: 10 19")), 190);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse("3267: 81 40 27")), 3267);
    }
    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&parse("83: 17 5")), 0);
    }
    #[test]
    fn part2_example_4() {
        assert_eq!(part2(&parse("156: 15 6")), 156);
    }
    #[test]
    fn part2_example_5() {
        assert_eq!(part2(&parse("7290: 6 8 6 15")), 7290);
    }
    #[test]
    fn part2_example_6() {
        assert_eq!(part2(&parse("161011: 16 10 13")), 0);
    }
    #[test]
    fn part2_example_7() {
        assert_eq!(part2(&parse("192: 17 8 14")), 192);
    }
    #[test]
    fn part2_example_8() {
        assert_eq!(part2(&parse("21037: 9 7 18 13")), 0);
    }
    #[test]
    fn part2_example_9() {
        assert_eq!(part2(&parse("292: 11 6 16 20")), 292);
    }
}
