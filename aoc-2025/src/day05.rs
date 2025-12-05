use aoc_runner_derive::{aoc, aoc_generator};

pub struct FreshRange(usize, usize);

impl FreshRange {
    fn contains(&self, ingredient: &usize) -> bool {
        *ingredient >= self.0 && *ingredient <= self.1
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<FreshRange>, Vec<usize>) {
    let mut data = input.split("\n\n");
    let ranges = data
        .next()
        .expect("Should be ranges")
        .lines()
        .map(|line| {
            let mut ld = line.split("-");
            let start = ld
                .next()
                .expect("Should be a num")
                .parse::<usize>()
                .expect("Should be parsable");
            let end = ld
                .next()
                .expect("Should be a num")
                .parse::<usize>()
                .expect("Should be parsable");
            FreshRange(start, end)
        })
        .collect::<Vec<FreshRange>>();
    let available = data
        .next()
        .expect("Should be numbers")
        .lines()
        .map(|line| line.parse::<usize>().expect("Should be parsable"))
        .collect::<Vec<usize>>();
    (ranges, available)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<FreshRange>, Vec<usize>)) -> usize {
    input
        .1
        .iter()
        .map(|ingredient| {
            if input.0.iter().any(|range| range.contains(ingredient)) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<FreshRange>, Vec<usize>)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
            )),
            3
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
