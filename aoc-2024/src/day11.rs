use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Stone {
    number: usize,
    dead: bool,
}

impl Stone {
    fn increase_stone(&mut self) {
        self.number = 1;
    }
    fn split_stone(&mut self) -> Stone {
        // Thanks day 7, hopefully this doesn't cause any issues...
        let number_size = ((self.number + 1) as f64).log10().ceil() as u32;
        let stone_a = self.number / 10_usize.pow(number_size / 2);
        let stone_b = self.number % 10_usize.pow(number_size / 2);

        self.number = stone_a;
        Stone {
            number: stone_b,
            dead: false,
        }

        // Don't like creating a string all the time but whatever
        // let num_str = self.number.to_string();
        // let (stone_a, stone_b) = num_str.split_at(num_str.len() / 2);

        // self.number = stone_a.parse().expect("Failed to parse number after split");
        // Stone {
        //     number: stone_b.parse().expect("Failed to parse number after split"),
        // }
    }
    fn multiply_stone(&mut self) {
        self.number *= 2024;
    }
    fn process_rules(&mut self) -> Stone {
        let empty_stone = Stone {
            number: 0,
            dead: true,
        };
        if self.number == 0 {
            self.increase_stone();
            return empty_stone;
        }
        let size = ((self.number + 1) as f64).log10().ceil() as u32;
        if size % 2 == 0 {
            return self.split_stone();
        }
        self.multiply_stone();
        empty_stone
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|num| Stone {
            number: num.parse().expect("Failed to parse pre-determind stone"),
            dead: false,
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Stone]) -> usize {
    let mut stones: Vec<Stone> = input.to_vec();
    let mut new_stones: Vec<(usize, Stone)> = vec![];
    for _blink in 0..25 {
        // println!("Blink {}, Stones: {:?}", _blink, stones);
        for stone_index in 0..stones.len() {
            let extra = stones[stone_index].process_rules();
            if extra.dead {
                continue;
            }
            new_stones.push((stone_index + 1, extra));
        }
        for (stone_index, stone) in new_stones.iter() {
            stones.insert(*stone_index, *stone);
        }
        new_stones.clear();
    }

    stones.len()
}

// #[aoc(day11, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 55312);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
