use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Positions(isize, isize);
impl Positions {
    fn area(&self, other: &Positions) -> isize {
        let height = (self.0 - other.0).abs() + 1;
        let width = (self.1 - other.1).abs() + 1;
        height * width
    }

    /// questionable corners are the 2 non-red nodes.
    fn get_questionable_corners(&self, other: &Positions) -> [Positions; 2] {
        // println!("questionable from: {:?}, {:?}", self, other);
        // println!(
        //     "{:?}",
        //     [Positions(other.0, self.1), Positions(self.0, other.1)]
        // );
        [Positions(other.0, self.1), Positions(self.0, other.1)]
    }

    fn multi_same_col(&self, start: &Positions, end: &Positions) -> bool {
        self.1 == start.1 && start.1 == end.1
    }
    fn multi_inbetween_col(&self, start: &Positions, end: &Positions) -> bool {
        (self.1 >= start.1 && self.1 <= end.1) || (self.1 >= end.1 && self.1 <= start.1)
    }
    fn multi_same_row(&self, start: &Positions, end: &Positions) -> bool {
        self.0 == start.0 && start.0 == end.0
    }
    fn multi_inbetween_row(&self, start: &Positions, end: &Positions) -> bool {
        (self.0 >= start.0 && self.0 <= end.0) || (self.0 >= end.0 && self.0 <= start.0)
    }
    fn same_col(&self, pos: &Positions) -> bool {
        self.1 == pos.1
    }
    fn same_row(&self, pos: &Positions) -> bool {
        self.0 == pos.0
    }
}
impl From<&str> for Positions {
    fn from(value: &str) -> Self {
        let mut splits = value.split(",");
        let first = splits
            .next()
            .expect("Failed to get first item")
            .parse::<isize>()
            .expect("Failed to parse to isize");
        let second = splits
            .next()
            .expect("Failed to get second item")
            .parse::<isize>()
            .expect("Failed to parse to isize");
        Self(second, first)
    }
}

///
/// XXXXX...XXXXX
/// ....X...X...X
/// .O..X.O.X...X
/// ....XXXXX....
///

fn is_outside_perimeter(
    perimeter: &[Positions],
    position: &Positions,
    cache: &mut HashMap<Positions, usize>,
) -> bool {
    // println!();
    // println!("Looking at {:?}", position);
    if let Some(cached) = cache.get(position) {
        // println!("Returning from cache!");
        // println!();

        return cached.is_multiple_of(2);
    }

    let start = &&perimeter[0].clone();
    let mut iter = perimeter.iter().peekable();
    let mut count = 0_usize;
    while let Some(line_start) = iter.next() {
        let line_end = iter.peek().unwrap_or(&&start);

        // if it's either linestart or lineend it's on the line, hence must be inside.
        if position == line_start || position == *line_end {
            cache.insert(position.to_owned(), 101);
            // println!("Point is on line, early break");
            return false;
        }

        if position.multi_inbetween_col(line_start, line_end)
            && position.multi_inbetween_row(line_start, line_end)
        {
            // println!("In grid line");
            // count += 1;
            // continue;
            cache.insert(position.to_owned(), 51);
            // println!("In grid line");
            return false;
        }

        if position.same_row(line_start) || position.same_row(line_end) {
            continue;
        }

        // println!("{:?}, {:?}", line_start, line_end);

        if line_start.same_col(line_end) {
            if position.multi_inbetween_row(line_start, line_end) {
                // println!("+=1");
                count += 1;
            }
        }
    }

    // println!("{:?} is {:?}", position, count);
    // println!();
    cache.insert(position.to_owned(), count);

    // if odd, then we passed through 1 wall not 2.
    // count != 0 as passing though zero walls means we are outside.
    // !count.is_multiple_of(2) && count != 0
    count.is_multiple_of(2)
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Positions> {
    input.lines().map(Positions::from).collect_vec()
}

#[aoc(day9, part1)]
fn part1(input: &[Positions]) -> isize {
    input
        .iter()
        .map(|pos| {
            input
                .iter()
                .map(|pos2| pos.area(pos2))
                .max()
                .unwrap_or_default()
        })
        .max()
        .expect("No max found??")
}

#[aoc(day9, part2)]
fn part2(input: &[Positions]) -> isize {
    let mut cache = HashMap::<Positions, usize>::new();
    let res = input
        .iter()
        .map(|pos| {
            input
                .iter()
                .filter(|pos2| pos.0 != pos2.0 && pos.1 != pos2.1)
                .map(|pos2| pos.get_questionable_corners(pos2))
                .filter(|corners| {
                    !is_outside_perimeter(input, &corners[0], &mut cache)
                        && !is_outside_perimeter(input, &corners[1], &mut cache)
                })
                // .inspect(|c| println!("{:?}", c))
                .map(|corners| corners[0].area(&corners[1]))
                // .inspect(|c| println!("{:?}", c))
                .max()
                .unwrap_or_default()
        })
        // .inspect(|c| println!("{:?}", c))
        .max()
        // .inspect(|c| println!("{:?}", c))
        .unwrap();
    // println!("{:?}", cache);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 24);
    }

    #[test]
    fn zero_multiple_2() {
        assert_eq!(0_usize.is_multiple_of(2), true);
    }
}
