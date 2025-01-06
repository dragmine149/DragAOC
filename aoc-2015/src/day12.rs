use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<i64> {
    // regex to get all numbers. We don't really care about the json...
    let r = Regex::new(r"(-)?\d+").unwrap();
    let c = r.captures_iter(input);

    c.map(|cap| cap.get(0).unwrap().as_str().parse().unwrap())
        .collect_vec()
}

#[aoc(day12, part1)]
fn part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

// #[aoc(day12, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1_1() {
        assert_eq!(part1(&parse("[1,2,3]")), 6);
    }
    #[test]
    fn part1_example_1_2() {
        assert_eq!(part1(&parse("{\"a\":2,\"b\":4}")), 6);
    }
    #[test]
    fn part1_example_2_1() {
        assert_eq!(part1(&parse("[[[3]]]")), 3);
    }
    #[test]
    fn part1_example_2_2() {
        assert_eq!(part1(&parse("{\"a\":{\"b\":4},\"c\":-1}")), 3);
    }
    #[test]
    fn part1_example_3_1() {
        assert_eq!(part1(&parse("{\"a\":[-1,1]}")), 0);
    }
    #[test]
    fn part1_example_3_2() {
        assert_eq!(part1(&parse("[-1,{\"a\":1}]")), 0);
    }
    #[test]
    fn part1_example_4_1() {
        assert_eq!(part1(&parse("[]")), 0);
    }
    #[test]
    fn part1_example_4_2() {
        assert_eq!(part1(&parse("{}")), 0);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
