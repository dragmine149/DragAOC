use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
use num_format::{Locale, ToFormattedString};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Ord, Eq, PartialOrd)]
struct Stone {
    number: usize,
    count: usize,
}

impl Stone {
    fn get_number_length(&self) -> u32 {
        // Thanks day 7, hopefully this doesn't cause any issues...
        ((self.number + 1) as f64).log10().ceil() as u32
    }
    fn increase_stone(&mut self) {
        self.number = 1;
    }
    fn split_stone(&mut self) -> Stone {
        let number_size = self.get_number_length();
        let stone_a = self.number / 10_usize.pow(number_size / 2);
        let stone_b = self.number % 10_usize.pow(number_size / 2);

        self.number = stone_a;
        Stone {
            number: stone_b,
            count: self.count,
        }
    }
    fn multiply_stone(&mut self) {
        self.number *= 2024;
    }
    fn process_rules(&mut self) -> Option<Stone> {
        if self.number == 0 {
            self.increase_stone();
            return None;
        }
        if self.get_number_length() % 2 == 0 {
            return Some(self.split_stone());
        }
        self.multiply_stone();
        None
    }
    fn is_stone(&self, other: Stone) -> bool {
        self.number == other.number
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|num| Stone {
            number: num.parse().expect("Failed to parse pre-determind stone"),
            count: 1,
        })
        .collect()
}

fn blink(stone_list: &mut [Stone]) -> Vec<Option<Stone>> {
    let mut new_stones: Vec<Option<Stone>> = vec![];

    stone_list
        .par_iter_mut()
        .map(|stone| stone.process_rules())
        .collect_into_vec(&mut new_stones);

    new_stones
}

fn count_stones(stones: &[Stone]) -> usize {
    stones
        .iter()
        .map(|stone| stone.count)
        .reduce(|acc, stn| acc + stn)
        .expect("Failed to reduce stone list")
}

fn collapse_stones(stones: &[Stone]) -> Vec<Stone> {
    let mut new_stones: Vec<Stone> = vec![];

    for stone in stones.iter() {
        let found = new_stones.iter().position(|stn| stone.is_stone(*stn));
        match found {
            Some(pos) => {
                new_stones[pos].count += stone.count;
            }
            None => {
                new_stones.push(*stone);
            }
        }
    }

    new_stones.sort();
    new_stones
}

#[aoc(day11, part1)]
fn part1(input: &[Stone]) -> usize {
    let mut stones: Vec<Stone> = input.to_vec();
    for _blink in 0..25 {
        // println!(
        //     "Blink: {}. Count: {:#?}",
        //     _blink,
        //     count_stones(&stones).to_formatted_string(&Locale::en_GB)
        // );

        let new_stones = blink(&mut stones);
        for stone in new_stones.iter() {
            if let Some(result) = stone {
                stones.push(*result);
            }
        }
        stones = collapse_stones(&stones);
    }

    // println!("Length: {}", stones.len());
    count_stones(&stones)
}

#[aoc(day11, part2)]
fn part2(input: &[Stone]) -> usize {
    let mut stones: Vec<Stone> = input.to_vec();
    for _blink in 0..75 {
        println!(
            "Blink: {}. Count: {:#?}",
            _blink,
            count_stones(&stones).to_formatted_string(&Locale::en_GB)
        );

        let new_stones = blink(&mut stones);
        for stone in new_stones.iter() {
            if let Some(result) = stone {
                stones.push(*result);
            }
        }
        stones = collapse_stones(&stones);
    }

    println!("Length: {}", stones.len());
    count_stones(&stones)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 55312);
    }
}
