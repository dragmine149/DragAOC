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

pub enum InsideType {
    Outside,
    Inside,
    Specified,
    Line,
}
impl InsideType {
    fn next(&self) -> InsideType {
        match self {
            InsideType::Outside => InsideType::Inside,
            InsideType::Inside => InsideType::Outside,
            InsideType::Specified => todo!(),
            InsideType::Line => todo!(),
        }
    }
}

///
/// XXXXX...XXXXX
/// ....X...X...X
/// .O..X.O.X...X
/// ....XXXXX....
///

fn is_inside_perimeter(
    perimeter: &[Positions],
    position: &Positions,
    cache: &mut HashMap<Positions, bool>,
) -> bool {
    if let Some(cached) = cache.get(position) {
        // println!("Returning from cache!");
        // println!();
        return *cached;
    }
    println!();
    println!("Looking at {:?}", position);

    let start = &&perimeter[0].clone();
    let mut iter = perimeter.iter().peekable();
    let mut inside = false;
    while let Some(line_start) = iter.next() {
        let line_end = iter.peek().unwrap_or(&&start);

        // if it's either linestart or lineend it's on the line, hence must be inside.
        if position == line_start || position == *line_end {
            // we start here, so ofc it's true
            inside = true;
        }

        if position.multi_inbetween_col(line_start, line_end)
            && position.multi_inbetween_row(line_start, line_end)
        {
            // well, we are in between 2 lines...
            // i guess, its true...
            inside = true;
        }

        // if position.same_row(line_start) || position.same_row(line_end) {
        //     continue;
        // }

        println!("{:?}, {:?}", line_start, line_end);

        if line_start.same_col(line_end) {
            if position.multi_inbetween_row(line_start, line_end) {
                // println!("+=1");
                inside = !inside;
            }
        }
    }

    println!("{:?} is {:?}", position, inside);
    println!();
    cache.insert(position.to_owned(), inside);

    // if odd, then we passed through 1 wall not 2.
    // count != 0 as passing though zero walls means we are outside.
    // !count.is_multiple_of(2) && count != 0
    inside
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
    let mut cache = HashMap::<Positions, bool>::new();
    let res = input
        .iter()
        .map(|pos| {
            input
                .iter()
                .filter(|pos2| pos.0 != pos2.0 && pos.1 != pos2.1)
                .map(|pos2| pos.get_questionable_corners(pos2))
                .filter(|corners| {
                    let inside = is_inside_perimeter(input, &corners[0], &mut cache)
                        && is_inside_perimeter(input, &corners[1], &mut cache);
                    println!("{:?} inside: {:?}", corners, inside);
                    inside
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
    println!("{:?}", cache);
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

    const CUSTOM_EXAMPLE_1: &str = "11,1
11,6
9,6
9,5
5,5
5,7
2,7
2,3
7,3
7,1
";

    const COMMUNITY_1: &str = "1,0
3,0
3,6
16,6
16,0
18,0
18,9
13,9
13,7
6,7
6,9
1,9";

    const COMMUNITY_2: &str = "1,1
8,1
8,3
3,3
3,4
8,4
8,9
18,9
18,11
5,11
5,9
4,9
4,11
1,11
1,7
6,7
6,6
1,6";

    const COMMUNITY_3: &str = "1,5
3,5
3,8
7,8
7,5
9,5
9,10
11,10
11,3
6,3
6,7
4,7
4,1
13,1
13,12
1,12";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 24);
    }

    #[test]
    fn part2_custom_example_1() {
        assert_eq!(part2(&parse(CUSTOM_EXAMPLE_1)), 24);
    }

    #[test]
    fn part2_community_1() {
        assert_eq!(part2(&parse(COMMUNITY_1)), 30);
    }

    #[test]
    fn part2_community_2() {
        assert_eq!(part2(&parse(COMMUNITY_2)), 88);
    }

    #[test]
    fn part2_community_3() {
        assert_eq!(part2(&parse(COMMUNITY_3)), 72);
    }

    #[test]
    fn zero_multiple_2() {
        assert_eq!(0_usize.is_multiple_of(2), true);
    }
}
