use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

// Vector and manual version of binary wrapping.
fn warp_operators(operators: &mut [u8]) {
    operators[0] += 1;
    for index in 0..(operators.len() - 1) {
        if operators[index] >= 3 {
            operators[index] = 0;
            operators[index + 1] += 1;
        }
    }
}

// The same as the previous check_if_calculate but uses vector wrapping and support for the third operator
fn check_if_calculate_2(input: &(u64, Vec<u64>)) -> bool {
    let numbers = &input.1;
    let mut operators: Vec<u8> = vec![0; numbers.len() + 2];

    loop {
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
                // Does 10^(the ceiling of how many times 10 goes into the right hand side number).
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
    }
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
