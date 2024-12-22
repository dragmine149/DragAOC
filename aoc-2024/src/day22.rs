use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Secret {
    number: u128,
    prices: Vec<u8>,
    price_dif: Vec<i8>,
}

impl Secret {
    fn mix(&mut self, value: u128) {
        self.number ^= value;
    }

    fn prune(&mut self) {
        self.number %= 16777216;
    }

    fn evolve(&mut self) {
        let r1 = self.number * 64;
        self.mix(r1);
        self.prune();

        let r2 = self.number / 32;
        self.mix(r2);
        self.prune();

        let r3 = self.number * 2048;
        self.mix(r3);
        self.prune();
    }

    fn store_price(&mut self, previous: u128) {
        // println!("{:?} -> {:?}", self.number, self.number % 10);
        let current = (self.number % 10) as i8;
        let prev = (previous % 10) as i8;
        self.prices.push(current as u8);
        self.price_dif.push(current - prev);
    }

    fn evolve_x(&mut self, times: u128) {
        self.store_price(self.number);
        let mut prev = self.number;
        for _ in 0..times {
            self.evolve();
            self.store_price(prev);
            prev = self.number;
        }
        // println!("{:?}", self.prices);
        // println!("{:?}", self.price_dif);
    }

    fn get_sequences(&self) -> Vec<(i8, i8, i8, i8)> {
        let mut sequences = vec![];
        for (index, _) in self.prices.iter().enumerate() {
            if index < 4 {
                continue;
            }
            if index + 5 > self.prices.len() {
                break;
            }

            // let sequence = (
            //     self.prices[index + 1] as i8 - self.prices[index + 0] as i8,
            //     self.prices[index + 2] as i8 - self.prices[index + 1] as i8,
            //     self.prices[index + 3] as i8 - self.prices[index + 2] as i8,
            //     self.prices[index + 4] as i8 - self.prices[index + 3] as i8,
            // );
            let sequence = (
                self.price_dif[index + 0],
                self.price_dif[index + 1],
                self.price_dif[index + 2],
                self.price_dif[index + 3],
            );
            let sum1 = sequence.0 + sequence.1;
            let sum2 = sum1 + sequence.1;
            let sum3 = sum2 + sequence.2;
            if -9 <= sum1 && sum1 <= 9 {
                if -9 <= sum2 && sum2 <= 9 {
                    if -9 <= sum3 && sum3 <= 9 {
                        sequences.push(sequence);
                    }
                }
            }
        }
        sequences
    }

    fn get_sequence_price(&self, sequence: (i8, i8, i8, i8)) -> usize {
        for (index, _) in self.price_dif.iter().enumerate() {
            if index < 4 || index + 5 > self.price_dif.len() {
                continue;
            }

            if self.price_dif[index + 0] != sequence.0 {
                continue;
            }
            if self.price_dif[index + 1] != sequence.1 {
                continue;
            }
            if self.price_dif[index + 2] != sequence.2 {
                continue;
            }
            if self.price_dif[index + 3] != sequence.3 {
                continue;
            }
            return index + 3;
        }
        usize::MAX
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Secret> {
    input
        .lines()
        .map(|line| Secret {
            number: line.trim().parse().expect("Failed to parse number"),
            prices: vec![],
            price_dif: vec![],
        })
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &[Secret]) -> u128 {
    let mut secrets = input.to_vec();
    secrets
        .par_iter_mut()
        .for_each(|secret| secret.evolve_x(2000));
    secrets.par_iter().map(|secret| secret.number).sum::<u128>()
    // + 1
    // 0
}

#[aoc(day22, part2)]
fn part2(input: &[Secret]) -> u128 {
    let mut secrets = input.to_vec();
    secrets
        .par_iter_mut()
        .for_each(|secret| secret.evolve_x(2000));
    let a = secrets
        // .iter()
        .par_iter_mut()
        .map(|secret| secret.get_sequences())
        .collect::<Vec<Vec<(i8, i8, i8, i8)>>>();
    // .map(|sequence| sequence.len())
    // .collect::<Vec<usize>>();
    // println!("{:?}", a);
    let mut unique_count: Vec<((i8, i8, i8, i8), u8)> = vec![];
    for b in a {
        for sequence in b {
            let pos = unique_count.iter().position(|x| x.0 == sequence);
            if pos.is_none() {
                unique_count.push((sequence, 1));
                continue;
            }
            unique_count[pos.unwrap()].1 += 1;
        }
    }
    // let unique_count2 = unique_count
    //     .iter()
    //     .filter(|x| x.1 > 2)
    //     .collect::<Vec<&((i8, i8, i8, i8), u8)>>();
    // println!("{:?}", unique_count2);
    // let m = unique_count2.par_iter().map(|u| u.1 as u64).sum::<u64>();
    // println!("{:?}", m);
    println!("E");

    let total: u128 = unique_count
        .par_iter()
        .filter(|seq| seq.1 > 2)
        .map(|seq| {
            secrets
                .to_vec()
                .par_iter()
                .map(|monkey| {
                    let price = monkey.get_sequence_price(seq.0);
                    if price == usize::MAX {
                        0
                    } else {
                        monkey.prices[price] as u128
                    }
                })
                .sum::<u128>()
        })
        .max()
        .expect("Failed to get max value of sequences");

    total as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1
10
100
2024
";

    const EXAMPLE_2: &str = "1
2
3
2024
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 37327623);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 24);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 23);
    }
}
