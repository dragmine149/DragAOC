use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<i64>>()
}

#[aoc(day1, part1)]
fn part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

// #[aoc(day1, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_0() {
        assert_eq!(part1(&parse("(())")), 0);
        assert_eq!(part1(&parse("()()")), 0);
    }

    #[test]
    fn part1_example_3() {
        assert_eq!(part1(&parse("(((")), 3);
        assert_eq!(part1(&parse("(()(()(")), 3);
        assert_eq!(part1(&parse("))(((((")), 3);
    }

    #[test]
    fn part1_example_n1() {
        assert_eq!(part1(&parse("())")), -1);
        assert_eq!(part1(&parse("))(")), -1);
    }

    #[test]
    fn part1_example_n3() {
        assert_eq!(part1(&parse(")))")), -3);
        assert_eq!(part1(&parse(")())())")), -3);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
