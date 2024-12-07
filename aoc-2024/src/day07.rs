use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

fn warp_operators(operators: &mut Vec<u8>) {
    operators[0] += 1;
    for index in 0..(operators.len() - 1) {
        if operators[index] >= 3 {
            operators[index] = 0;
            operators[index + 1] += 1;
        }
    }
}

fn check_if_calculate_2(input: &(u64, Vec<u64>)) -> bool {
    let numbers = &input.1;
    let mut operators: Vec<u8> = vec![];
    operators.resize(numbers.len() + 2, 0);

    let calculated = loop {
        // println!("---");
        let mut calculation = 0;
        for (index, number) in numbers.iter().enumerate() {
            // get operator
            let desired_operator = operators[index];
            if desired_operator == 0 {
                // println!("{:?} + {:?}", calculation, number);
                calculation += number;
            } else if desired_operator == 1 {
                // println!("{:?} || {:?}", calculation, number);
                calculation =
                    calculation * 10_u64.pow(((number + 1) as f64).log10().ceil() as u32) + number;
                // Self::Concatenate => lhs * 10_u64.pow(((rhs + 1) as f64).log10().ceil() as u32) + rhs,
            } else if desired_operator == 2 {
                // println!("{:?} * {:?}", calculation, number);
                calculation *= number;
            } else {
                panic!("AAA");
            }
        }

        warp_operators(&mut operators);

        // println!("{:?} ?= {:?}", calculation, input.0);
        if calculation == input.0 {
            break true;
        }

        if operators[operators.len() - 1] >= 3 {
            break false;
        }
    };
    calculated
}

#[aoc(day7, part2)]
fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        // .iter()
        .map(|calc| {
            // println!("Attempting to check: {:?}", calc);
            let calculated = check_if_calculate_2(calc);
            // println!("Calculated: {:?}", calculated);
            if calculated {
                return calc.0;
            }

            0
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
}
