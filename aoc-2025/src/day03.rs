use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct BatterBank(pub Vec<u8>);
impl From<Vec<u8>> for BatterBank {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl BatterBank {
    fn get_largest(&self) -> u8 {
        let mut ten = 0_u8;
        let mut one = 0_u8;
        let mut iter = self.0.iter().peekable();
        while let Some(num) = iter.next() {
            let not_end = iter.peek().is_some();
            if *num > ten && not_end {
                ten = *num;
                one = 0;
                continue;
            }
            if *num > one {
                one = *num;
                continue;
            }
        }
        // println!("{:?} -> {:?}", self, ten * 10 + one);
        ten * 10 + one
    }

    fn multi_jolt(&self, jolt_size: usize) -> u64 {
        let mut batteries = Vec::with_capacity(jolt_size);
        for _ in 0..jolt_size {
            batteries.push(0);
        }
        // let mut max = 0;
        for x in 0..self.0.len() {
            println!("{:?}", batteries);
            let value = self.0.get(x).unwrap();
            // if let Some(pos) = next_empty {
            //     batteries[pos] = *value;
            // }
            println!("value: {:?}", value);
            let max_left = self.0.len() - jolt_size;
            println!("{:?} {:?}", max_left, x);
            if x < max_left {
                for pos in 0..(x + 1) {
                    println!("{:?} '{:?}'", pos, batteries[pos]);
                    if *value > batteries[pos] {
                        batteries[pos] = *value;
                        for cell in (pos + 1)..jolt_size {
                            batteries[cell] = 0;
                        }
                        break;
                    }
                }
            } else {
                let next_empty = batteries.iter().position(|x| *x == 0);

                if self.0.len() - x > jolt_size - next_empty.unwrap() {
                    continue;
                }

                if let Some(pos) = next_empty {
                    batteries[pos] = *value;
                } else {
                    break;
                }
            }
        }

        // for x in next_empty.unwrap()..batteries.len() {
        //     println!(
        //         "{:?}/{:?}/{:?}/{:?}/{:?}/{:?}",
        //         batteries.len(),
        //         next_empty.unwrap(),
        //         x,
        //         batteries,
        //         (batteries.len() - next_empty.unwrap()),
        //         self.0.get(self.0.len() - x)
        //     );
        //     // let pos = batteries.len() - next_empty.unwrap() - x;
        //     batteries[x] = *self.0.get(self.0.len() - x - 1).unwrap();
        // }
        // for x in 0..self.0.len() - (batteries.len() - next_empty.unwrap()) {
        //     let value = *self.0.get(x).unwrap();
        //     let pos = (batteries.len() - next_empty.unwrap()) - x;
        //     batteries[pos] = value;
        // }

        println!("{:?} -> {:?}", self, batteries);
        batteries.iter().fold(0, |acc, x| acc * 10 + (*x as u64))
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<BatterBank> {
    input
        .lines()
        .map(|line| {
            BatterBank::from(
                line.chars()
                    .map(|c| c.to_digit(10).expect("Failed to convert digit to number!") as u8)
                    .collect::<Vec<u8>>(),
            )
        })
        .collect::<Vec<BatterBank>>()
}

#[aoc(day3, part1)]
fn part1(input: &[BatterBank]) -> u64 {
    input.iter().map(|bank| bank.get_largest() as u64).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[BatterBank]) -> u64 {
    input.iter().map(|bank| bank.multi_jolt(12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
            )),
            357
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
            )),
            3121910778619
        );
    }
}
