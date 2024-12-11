use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused_imports)]
use num_format::{Locale, ToFormattedString};
#[warn(unused_imports)]
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Ord, Eq, PartialOrd)]
struct Stone {
    number: usize, // the number of this stone
    count: usize,  // how many of this stone exists
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
        // using maths YAY get the upper and lower half of the numbers
        let number_size = self.get_number_length();
        let stone_a = self.number / 10_usize.pow(number_size / 2);
        let stone_b = self.number % 10_usize.pow(number_size / 2);

        self.number = stone_a;
        // Make the same amount of stones just with the lower number
        Stone {
            number: stone_b,
            count: self.count,
        }
    }
    fn multiply_stone(&mut self) {
        self.number *= 2024;
    }
    fn process_rules(&mut self) -> Option<Stone> {
        // Process all the possible rules for the stones in order.
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
        // Checks if a stone is a stone, could have probably done this somehow using `==` but it works.
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
    let mut new_stones: Vec<Option<Stone>> = vec![]; // list of new empty stones made from the stones

    // Get all the stones to process the rules at once
    stone_list
        .par_iter_mut()
        .map(|stone| stone.process_rules())
        .collect_into_vec(&mut new_stones);

    new_stones
}

// Special count function as `.len` won't return the true value
fn count_stones(stones: &[Stone]) -> usize {
    stones
        .iter()
        .map(|stone| stone.count)
        .reduce(|acc, stn| acc + stn)
        .expect("Failed to reduce stone list")
}

// Allows the main stone list to be kept small. Always combines duplicates when possible
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

fn blink_x_times(input: &[Stone], times: usize) -> usize {
    let mut stones: Vec<Stone> = input.to_vec();
    for _blink in 0..times {
        // println!(
        //     "Blink: {}. Count: {:#?}",
        //     _blink,
        //     count_stones(&stones).to_formatted_string(&Locale::en_GB)
        // );

        let new_stones = blink(&mut stones);
        for stone in new_stones.iter().flatten() {
            stones.push(*stone);
        }
        stones = collapse_stones(&stones);
    }

    // println!("Length: {}", stones.len());
    count_stones(&stones)
}

#[aoc(day11, part1)]
fn part1(input: &[Stone]) -> usize {
    blink_x_times(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &[Stone]) -> usize {
    blink_x_times(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    // only one test and example today
    const EXAMPLE_1: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 65601038650482);
    }
}
