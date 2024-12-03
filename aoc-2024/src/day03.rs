use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

fn parse_regex(input: &str, skip: bool) -> Vec<Vec<Vec<u64>>> {
    // Regex used: https://regex101.com/r/PHC6Mr/2
    let regex = Regex::new(r"mul\((?P<num1>\d*),(?P<num2>\d*)\)").unwrap();

    let result = input
        .lines()
        .map(|x| {
            let reg_result = regex.captures_iter(x);
            let mut calculations: Vec<Vec<u64>> = vec![];
            for mat in reg_result {
                let num_1 = mat
                    .name("num1")
                    .expect("Failed to find first number in match")
                    .as_str()
                    .parse::<u64>()
                    .expect("Failed to parse number from regex num1 match");
                let num_2 = mat
                    .name("num2")
                    .expect("Failed to find second number in match")
                    .as_str()
                    .parse::<u64>()
                    .expect("Failed to parse number from regex num2 match");

                calculations.push(vec![num_1, num_2]);
            }
            calculations
        })
        .collect();
    // println!("{:#?}", result);

    result
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let regex_result = parse_regex(input, false);
    let result = regex_result
        .iter()
        .map(|line| {
            line.iter()
                .map(|calc| {
                    calc.get(0).expect("Failed to get num 1")
                        * calc.get(1).expect("Failed to get num 2")
                })
                .sum::<u64>()
        })
        .sum::<u64>();
    // println!("{:#?}", result);

    result
}

// #[aoc(day3, part2)]
// fn part2(input: &Vec<Vec<u32>>) -> u32 {
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 161);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(EXAMPLE_2)), 48);
    // }
}
