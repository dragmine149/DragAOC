use aoc_runner_derive::aoc;
use pcre::Pcre;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;
use regex::bytes::Regex;

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    let double = Regex::new(r"(?m)(?P<double>(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz))").unwrap();
    let fail = Regex::new(r"(?m)(?P<fail>(ab|cd|pq|xy))").unwrap();

    input
        .par_lines()
        .map(|line| {
            let r1 = line
                .chars()
                .filter(|c| c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u')
                .count()
                >= 3;

            let r2 = double.captures(line.as_bytes()).is_some();
            let r3 = fail.captures(line.as_bytes()).is_none();

            // println!("{:?} - {:?}, {:?}, {:?}", line, r1, r2, r3);
            if r1 && r2 && r3 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    let mut p2r1 = Pcre::compile(r"(..).*\1").unwrap();
    let mut p2r2 = Pcre::compile(r"(.).\1").unwrap();

    input
        // .par_lines()
        .lines()
        .map(|line| {
            let r1 = p2r1.exec(line).is_some();
            let r2 = p2r2.exec(line).is_some();

            if r1 && r2 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 2);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
