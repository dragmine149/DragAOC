use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Secret {
    number: u128,
    prices: Vec<u8>,
}

impl Secret {
    fn get_x_prices(&self, start: u64, end: u64) -> Vec<u8> {
        let mut x = vec![];
        for index in start..end {
            x.push(self.prices[index as usize]);
        }
        x
    }

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

    fn store_price(&mut self) {
        self.prices.push((self.number % 10) as u8);
    }

    fn evolve_x(&mut self, times: u128) -> u128 {
        self.store_price();
        for _ in 0..times {
            self.evolve();
            self.store_price();
        }

        self.number
    }

    fn get_sequence(&self) -> (i8, i8, i8, i8) {
        todo!();
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Secret> {
    input
        .lines()
        .map(|line| Secret {
            number: line.trim().parse().expect("Failed to parse number"),
            prices: vec![],
        })
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &[Secret]) -> u128 {
    let secrets = input.to_vec();
    secrets
        .par_iter()
        .map(|secret| {
            // secret.evolve_x(2000);
            let mut s = secret.to_owned();
            s.evolve_x(2000)
            // println!("{:?}", s);
            // a
        })
        .sum::<u128>()
    // + 1
    // 0
}

#[aoc(day22, part2)]
fn part2(input: &[Secret]) -> u128 {
    let secrets = input.to_vec();
    secrets
        .par_iter()
        .map(|secret| {
            println!("{:?}", secret.number);
            let mut s = secret.to_owned();
            s.evolve_x(2000);
            println!("{:?}", s.get_x_prices(0, 10));
            s
        })
        .map(|secret| secret.get_sequence())
        .collect::<Vec<(i8, i8, i8, i8)>>();

    0
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
