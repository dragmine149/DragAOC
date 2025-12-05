use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreshRange(usize, usize);

impl FreshRange {
    fn contains(&self, ingredient: &usize) -> bool {
        *ingredient >= self.0 && *ingredient <= self.1
    }

    fn range_count(&self) -> usize {
        (self.1 - self.0) + 1
    }
}

fn parse_ranges<'a>(input: impl Iterator<Item = &'a str>) -> Vec<FreshRange> {
    input
        .take_while(|line| !line.is_empty())
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
        .collect::<Vec<FreshRange>>()
}

#[aoc_generator(day5, part1)]
fn parse(input: &str) -> (Vec<FreshRange>, Vec<usize>) {
    let mut iter = input.lines();
    let ranges = parse_ranges(&mut iter);
    iter.next();
    let available = iter
        .map(|line| line.parse::<usize>().expect("Should be parsable"))
        .collect::<Vec<usize>>();
    (ranges, available)
}

#[aoc_generator(day5, part2)]
fn parse2(input: &str) -> Vec<FreshRange> {
    parse_ranges(input.lines())
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

fn collapse_ranges(ranges: &[FreshRange]) -> Vec<FreshRange> {
    let mut new_ranges: Vec<FreshRange> = vec![];
    for range in ranges {
        // println!("{:?}, {:?}", new_ranges, range);
        if let Some(contains_range) = new_ranges.iter_mut().find(|r| r.contains(&range.0)) {
            contains_range.1 = contains_range.1.max(range.1);
            continue;
        }
        if let Some(contains_range) = new_ranges.iter_mut().find(|r| r.contains(&(range.0 - 1))) {
            contains_range.1 = contains_range.1.max(range.1);
            continue;
        }
        if let Some(contains_range) = new_ranges.iter_mut().find(|r| r.contains(&range.1)) {
            contains_range.0 = contains_range.0.min(range.0);
            continue;
        }
        if let Some(contains_range) = new_ranges.iter_mut().find(|r| r.contains(&(range.1 + 1))) {
            contains_range.0 = contains_range.0.min(range.0);
            continue;
        }
        new_ranges.push(range.clone())
    }
    new_ranges
}

#[aoc(day5, part2)]
fn part2(input: &[FreshRange]) -> usize {
    let mut changed = true;
    let mut ranges = input.to_vec();
    while changed {
        let new_ranges = collapse_ranges(&ranges);
        changed = new_ranges != ranges;
        ranges = new_ranges;
        ranges.sort();
    }

    // ranges.sort();
    // println!("{:#?}", ranges);
    // println!("WARNING +1 output");
    ranges
        .iter()
        .map(|range| range.range_count())
        .sum::<usize>()
    // + 1
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

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse2(
                "3-5
10-14
16-20
12-18
"
            )),
            14
        );
    }
}
