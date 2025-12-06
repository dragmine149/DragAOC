use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Operation {
    Addition,
    Multiplication,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Addition,
            '*' => Self::Multiplication,
            _ => panic!("Invalid operation!"),
        }
    }
}

#[derive(Debug)]
pub struct Calculation {
    numbers: Vec<usize>,
    operation: Operation,
    size: usize,
}

impl Calculation {
    pub fn calculate_p1(&self) -> usize {
        match self.operation {
            Operation::Addition => self.numbers.iter().fold(0, |acc, x| acc + x),
            Operation::Multiplication => self.numbers.iter().fold(1, |acc, x| acc * x),
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Calculation> {
    let mut calculations = vec![];
    let mut size_count = 0_usize;
    let mut previous_char: char = char::default();
    // For now, we are just going to ignore the weird spacing. However i fell like its going to come back to bite me.
    input.lines().last().unwrap().chars().for_each(|char| {
        if char.is_ascii_punctuation() {
            if previous_char != char::default() {
                calculations.push(Calculation {
                    numbers: vec![],
                    operation: Operation::from(previous_char),
                    size: size_count - 1,
                });
            }
            size_count = 1;
            previous_char = char;
            return;
        }
        if char.is_whitespace() {
            size_count += 1;
            return;
        }
    });
    calculations.push(Calculation {
        numbers: vec![],
        operation: Operation::from(previous_char),
        size: size_count,
    });

    input
        .lines()
        .enumerate()
        .take_while(|(count, _)| *count != input.lines().count() - 1)
        .for_each(|(_, line)| {
            // println!("{:?}", line);
            let mut pos = 0;
            calculations.iter_mut().for_each(|calc| {
                let size = if calc.size == 1 {
                    line.len() - pos
                } else {
                    calc.size
                };
                calc.numbers.push(
                    line[pos..pos + size]
                        .replace(" ", "")
                        .parse::<usize>()
                        .expect(&format!(
                            "Failed to parse number between {:?} and {:?} ({:#?})",
                            pos,
                            pos + size,
                            line[pos..pos + size].to_string()
                        )),
                );
                pos += size + 1;
            });
        });

    calculations
}

#[aoc(day6, part1)]
fn part1(input: &[Calculation]) -> usize {
    // println!("{:#?}", input);
    input.iter().map(|c| c.calculate_p1()).sum()
}

// #[aoc(day6, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let parsed = parse(
            "123
45
6
*",
        );

        assert_eq!(parsed.get(0).unwrap().numbers, vec![123, 45, 6]);
        assert_eq!(part1(&parsed), 33210);
    }

    #[test]
    fn part_1_example_2() {
        let parsed = parse(
            "328
64
98
+",
        );
        assert_eq!(parsed.get(0).unwrap().numbers, vec![328, 64, 98]);
        assert_eq!(part1(&parsed), 490);
    }

    #[test]
    fn part_1_example_3() {
        let parsed = parse(
            " 51
387
215
*",
        );
        assert_eq!(parsed.get(0).unwrap().numbers, vec![51, 387, 215]);
        assert_eq!(part1(&parsed), 4243455);
    }

    #[test]
    fn part_1_example_4() {
        let parsed = parse(
            "64
23
314
+",
        );
        assert_eq!(parsed.get(0).unwrap().numbers, vec![64, 23, 314]);
        assert_eq!(part1(&parsed), 401);
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
            )),
            4277556
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
