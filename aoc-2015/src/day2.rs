use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    let regex = Regex::new(r"(?m)(\d*)x(\d*)x(\d*)").unwrap();
    input
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            (
                caps.get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse c1"),
                caps.get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse c3"),
                caps.get(3)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse c3"),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

#[aoc(day2, part1)]
fn part1(input: &[(usize, usize, usize)]) -> usize {
    // 2*l*w + 2*w*h + 2*h*l
    // l w h
    input
        .iter()
        .map(|b| {
            let side_a = b.0 * b.1;
            let side_b = b.0 * b.2;
            let side_c = b.1 * b.2;
            let extra = side_a.min(side_b).min(side_c);

            (2 * side_a) + (2 * side_b) + (2 * side_c) + extra
        })
        .sum::<usize>()
}

#[aoc(day2, part2)]
fn part2(input: &[(usize, usize, usize)]) -> usize {
    input
        .iter()
        .map(|b| {
            let mut min = [b.0, b.1, b.2];
            min.sort();

            (b.0 * b.1 * b.2) + min[0] + min[0] + min[1] + min[1]
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse("2x3x4")), 58);
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse("1x1x10")), 43);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse("2x3x4")), 34);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse("1x1x10")), 14);
    }
}
