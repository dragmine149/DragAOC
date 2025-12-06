use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

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
    raw_numbers: Vec<String>,
    operation: Operation,
    size: usize,
}

impl Calculation {
    pub fn calculate(&self) -> usize {
        match self.operation {
            Operation::Addition => self.numbers.iter().sum(),
            Operation::Multiplication => self.numbers.iter().product(),
        }
    }
}

fn basic_parse<CF>(input: &str, mut func: CF) -> Vec<Calculation>
where
    CF: FnMut(&mut Calculation, &str, usize, usize),
{
    let mut calculations = vec![];
    let mut size_count = 0_usize;
    let mut previous_char: char = char::default();

    input.lines().last().unwrap().chars().for_each(|char| {
        if char.is_ascii_punctuation() {
            if previous_char != char::default() {
                calculations.push(Calculation {
                    numbers: vec![],
                    raw_numbers: vec![],
                    operation: Operation::from(previous_char),
                    size: size_count - 1,
                });
            }
            size_count = 1;
            previous_char = char;
            return;
        }
        size_count += 1;
    });
    calculations.push(Calculation {
        numbers: vec![],
        raw_numbers: vec![],
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
                func(calc, line, pos, size);
                pos += size + 1;
            });
        });
    calculations
}

#[aoc_generator(day6, part1)]
fn parse_p1(input: &str) -> Vec<Calculation> {
    basic_parse(input, |calc, line, pos, size| {
        calc.numbers.push(
            line[pos..pos + size]
                .replace(" ", "")
                .parse::<usize>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to parse number between {:?} and {:?} ({:#?})",
                        pos,
                        pos + size,
                        line[pos..pos + size].to_string()
                    )
                }),
        );
    })
}

#[aoc_generator(day6, part2)]
fn parse_p2(input: &str) -> Vec<Calculation> {
    let mut calculations = basic_parse(input, |calc, line, pos, size| {
        calc.raw_numbers.push(line[pos..pos + size].to_string());
    });

    calculations.iter_mut().for_each(|calc| {
        let length = calc
            .raw_numbers
            .iter()
            .map(|num| num.len())
            .max()
            .expect("No max length?");
        calc.raw_numbers.iter_mut().for_each(|num| {
            while num.len() != length {
                num.push(' ');
            }
        });

        for pos in (0..length).rev() {
            // print!(
            //     "{:?}",
            //     calc.raw_numbers
            //         .iter()
            //         .map(|num| num.get(pos..pos + 1).expect("Should be pos").to_string())
            //         .collect::<Vec<String>>()
            // );
            let nums = calc
                .raw_numbers
                .iter()
                .map(|num| num.get(pos..pos + 1).expect("Should be pos").to_string())
                .join("");
            calc.numbers.push(
                nums.replace(" ", "")
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Should be a num ({:?})", nums)),
            );
        }
    });
    // println!("{:?}", calculations);

    calculations
}

#[aoc(day6, part1)]
fn part1(input: &[Calculation]) -> usize {
    // println!("{:#?}", input);
    input.iter().map(|c| c.calculate()).sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Calculation]) -> usize {
    input.iter().map(|c| c.calculate()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let parsed = parse_p1(
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
        let parsed = parse_p1(
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
        let parsed = parse_p1(
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
        let parsed = parse_p1(
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
            part1(&parse_p1(
                "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
            )),
            4277556
        );
    }

    #[test]
    fn part_2_example_1() {
        let parsed = parse_p2(
            "123
 45
  6
*",
        );

        assert_eq!(parsed.get(0).unwrap().numbers, vec![356, 24, 1]);
        assert_eq!(part1(&parsed), 8544);
    }

    #[test]
    fn part_2_example_2() {
        let parsed = parse_p2(
            "328
64
98
+",
        );
        assert_eq!(parsed.get(0).unwrap().numbers, vec![8, 248, 369]);
        assert_eq!(part1(&parsed), 625);
    }

    #[test]
    fn part_2_example_3() {
        let parsed = parse_p2(
            " 51
387
215
*",
        );
        assert_eq!(parsed.get(0).unwrap().numbers, vec![175, 581, 32]);
        assert_eq!(part1(&parsed), 3253600);
    }

    #[test]
    fn part_2_example_4() {
        let parsed = parse_p2(
            "64
23
314
+",
        );
        assert_eq!(parsed.get(0).unwrap().numbers, vec![4, 431, 623]);
        assert_eq!(part1(&parsed), 1058);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part1(&parse_p2(
                "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
            )),
            3263827
        );
    }
}
