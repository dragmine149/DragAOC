use std::ops::AddAssign;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let info = line.trim().split_at(1);

            let dir = match info.0 {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction ({:?})", info.0),
            } as i64;

            info.1.parse::<i64>().unwrap() * dir
        })
        .collect::<Vec<i64>>()
}

#[aoc(day1, part1)]
fn part1(input: &[i64]) -> u64 {
    let mut pos = 50;
    let mut zero_count = 0;
    input.iter().for_each(|dir| {
        // println!("Pos: {:?}", pos);
        pos += dir;

        // loop around code.
        if pos < 0 {
            pos = 100 + (pos % 100);
        }
        if pos >= 100 {
            pos = pos % 100;
        }
        // add to 0.
        if pos == 0 {
            zero_count += 1;
        }
    });
    zero_count
}

#[aoc(day1, part2)]
fn part2(input: &[i64]) -> i64 {
    let mut pos = 50;
    let mut zero_count = 0;
    input.iter().for_each(|dir| {
        println!("Pos: {:?}. Zero: {:?}", pos, zero_count);
        let was_zero = pos == 0;
        let pre_zero = zero_count;
        pos += dir;

        // loop w/ div
        if pos < 0 {
            // println!("Pos: {:?}", pos);
            zero_count += (100 + (pos * -1)) / 100;
            if was_zero {
                zero_count -= 1;
            }

            pos = 100 + (pos % 100);
        }
        if pos >= 100 {
            zero_count += pos / 100;
            if was_zero {
                zero_count -= 1;
            }

            pos = pos % 100;
        }
        if pos == 0 && zero_count == pre_zero {
            zero_count += 1;
        }
    });
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            )),
            3
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            )),
            6
        );
    }
}
