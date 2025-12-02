use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Range(u64, u64);
impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self(start, end)
    }
    pub fn find_invalid(&self) -> Vec<u64> {
        let mut invalid = vec![];
        for id in self.0..self.1 + 1 {
            let size = get_num_size(id);
            // ignore all odd width numbers
            if size % 2 == 1 {
                continue;
            }

            // println!("Length of {}: {}", id, size);
            let pow = 10_u64.pow(size / 2); // originally had as `size - 1`
            let half = id / pow;
            let num = half * pow + half;
            // println!("Calculated: {:?}", num);
            if id == num && get_num_size(num) % 2 == 0 {
                invalid.push(id);
            }
        }

        // println!("{:?} has invalid of {:?}", self, invalid);
        invalid
    }

    pub fn find_invalid_str(&self) -> Vec<u64> {
        let mut invalid = vec![];
        for id in self.0..self.1 + 1 {
            let num = id.to_string();
            let length = num.len();
            if length % 2 == 1 {
                continue;
            }
            let data = num.split_at(length / 2);
            if data.0.parse::<u32>() == data.1.parse::<u32>() {
                invalid.push(id);
            }
        }
        invalid
    }

    pub fn find_invalid_extended(&self) -> Vec<u64> {
        let mut invalid = vec![];
        for id in self.0..self.1 + 1 {
            let size = get_num_size(id);
            for i in (size / 2)..size {
                let pow = 10_u64.pow(i);
                let repeat = id / pow;
                let repeat_len = get_num_size(repeat);
                let pow2 = 10_u64.pow(repeat_len);
                // println!(
                //     "Id: {:?}, Pow: {:?}, Repeat: {:?}. Repeat len: {:?}. Pow2: {:?}",
                //     id, pow, repeat, repeat_len, pow2
                // );
                // if repeat_len == 1 {
                //     continue;
                // }

                let mut guess = repeat;
                for x in (0..i).step_by(repeat_len as usize) {
                    // println!(
                    //     "Guess: {:?}, repeat: {:?}, x: {:?}, pow2: {:?}",
                    //     guess, repeat, x, pow2
                    // );
                    guess *= pow2;
                    guess += repeat;
                }
                if guess == repeat {
                    continue;
                }
                if guess == id {
                    // println!("eee");
                    // if !invalid.contains(&guess) {
                    invalid.push(guess);
                    break;
                    // }
                }
            }
        }

        println!("{:?} has invalid of {:?}", self, invalid);
        invalid
    }

    pub fn find_invalid_extended_str(&self) -> Vec<u64> {
        let mut invalid = vec![];
        for id in self.0..self.1 + 1 {
            let num = id.to_string();
            let length = num.len();
            for len in 1..length + 1 {
                if num[0..len].repeat(length / len) == num {
                    invalid.push(id);
                }
            }
        }

        invalid
    }
}
impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut data = value.split("-");
        let start = data
            .next()
            .expect("Meant to be start, nothing.")
            .parse::<u64>()
            .expect("Start is not a number");
        let end = data
            .next()
            .expect("Meant to be End, nothing.")
            .parse::<u64>()
            .expect("End is not a number");
        Self::new(start, end)
    }
}

pub fn get_num_size(num: u64) -> u32 {
    ((num + 1) as f64).log10().ceil() as u32
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Range> {
    input
        .split(",")
        .map(|s| Range::from(s))
        .collect::<Vec<Range>>()
}

#[aoc(day2, part1)]
fn part1(input: &[Range]) -> u64 {
    input.iter().flat_map(|r| r.find_invalid()).sum()
    // input.iter().flat_map(|r| r.find_invalid_str()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Range]) -> u64 {
    input
        .iter()
        .flat_map(|r| r.find_invalid_extended())
        .unique()
        .sum()
    // input
    //     .iter()
    //     .flat_map(|r| r.find_invalid_extended_str())
    //     .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )),
            1227775554
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )),
            4174379265
        );
    }
}
