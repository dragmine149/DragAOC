use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day12, part1)]
fn part1(input: &str) -> i64 {
    // regex to get all numbers. We don't really care about the json...
    let r = Regex::new(r"(-)?\d+").unwrap();
    let c = r.captures_iter(input);

    c.map(|cap| cap.get(0).unwrap().as_str().parse::<i64>().unwrap())
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1_1() {
        assert_eq!(part1(&"[1,2,3]"), 6);
    }
    #[test]
    fn part1_example_1_2() {
        assert_eq!(part1(&"{\"a\":2,\"b\":4}"), 6);
    }
    #[test]
    fn part1_example_2_1() {
        assert_eq!(part1(&"[[[3]]]"), 3);
    }
    #[test]
    fn part1_example_2_2() {
        assert_eq!(part1(&"{\"a\":{\"b\":4},\"c\":-1}"), 3);
    }
    #[test]
    fn part1_example_3_1() {
        assert_eq!(part1(&"{\"a\":[-1,1]}"), 0);
    }
    #[test]
    fn part1_example_3_2() {
        assert_eq!(part1(&"[-1,{\"a\":1}]"), 0);
    }
    #[test]
    fn part1_example_4_1() {
        assert_eq!(part1(&"[]"), 0);
    }
    #[test]
    fn part1_example_4_2() {
        assert_eq!(part1(&"{}"), 0);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&"[1,2,3]"), 6);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&"[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    }
    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&"{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
    }
    #[test]
    fn part2_example_4() {
        assert_eq!(part2(&"[1,\"red\",5]"), 6);
    }
}
