use std::{collections::HashMap, u8::MAX};

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

    fn get_corners(&self, other: &Positions) -> [Positions; 4] {
        [
            Positions(self.0, self.1),
            Positions(other.0, self.1),
            Positions(other.0, other.1),
            Positions(self.0, other.1),
        ]
    }
    fn corners_shrunk(&self, other: &Positions) -> [Positions; 4] {
        let potential = self.get_corners(other);
        let min_row = potential.iter().map(|p| p.0).min().unwrap();
        let min_col = potential.iter().map(|p| p.1).min().unwrap();
        let max_row = potential.iter().map(|p| p.0).max().unwrap();
        let max_col = potential.iter().map(|p| p.1).max().unwrap();
        [
            Positions(min_row + 1, min_col + 1),
            Positions(max_row + 1, min_col + 1),
            Positions(max_row + 1, max_col - 1),
            Positions(min_row + 1, max_col - 1),
        ]
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

fn min_max(a: isize, b: isize) -> (isize, isize) {
    if a < b { (b, a) } else { (a, b) }
}

fn line_crosses_line(
    line1_start: &Positions,
    line1_end: &Positions,
    line2_start: &Positions,
    line2_end: &Positions,
) -> bool {
    // if it ain't horizontal, it's vertical. Thats just how this works
    let line1_horizontal = line1_start.same_row(&line1_end);
    let line2_horizontal = line2_start.same_row(&line2_end);
    // They can't cross if they are the same horizontal or vertical
    if line1_horizontal && line2_horizontal {
        return false;
    }
    if !line1_horizontal && !line2_horizontal {
        return false;
    }
    println!(
        "l1s {:?}, l1e {:?}, l1h {:?}, l2s {:?}, l2e {:?}, l2h {:?}",
        line1_start, line1_end, line1_horizontal, line2_start, line2_end, line2_horizontal
    );
    if line1_horizontal {
        let line1_col_minmax = min_max(line1_start.1, line1_end.1);
        let line2_row_minmax = min_max(line2_start.0, line2_end.0);

        println!(
            "l1s < l2rm {:?} ({:?} < {:?})",
            line1_start.0 < line2_row_minmax.0,
            line1_start.0,
            line2_row_minmax.0
        );
        println!(
            "l1s > l2rm {:?} ({:?} > {:?})",
            line1_start.0 > line2_row_minmax.1,
            line1_start.0,
            line2_row_minmax.1
        );
        println!(
            "l1s < l2rm {:?} ({:?} < {:?})",
            line2_start.1 < line1_col_minmax.0,
            line2_start.0,
            line1_col_minmax.0
        );
        println!(
            "l1s > l2rm {:?} ({:?} > {:?})",
            line2_start.1 > line1_col_minmax.1,
            line2_start.0,
            line1_col_minmax.1
        );
        return (line1_start.0 < line2_row_minmax.0 && line1_start.0 > line2_row_minmax.1)
            && (line2_start.1 < line1_col_minmax.0 && line2_start.1 > line1_col_minmax.1);
    }

    let line1_row_minmax = min_max(line1_start.0, line1_end.0);
    let line2_col_minmax = min_max(line2_start.1, line2_end.1);

    println!(
        "l1s < l2rm {:?} ({:?} < {:?})",
        line1_start.1 < line2_col_minmax.0,
        line1_start.0,
        line2_col_minmax.0
    );
    println!(
        "l1s > l2rm {:?} ({:?} > {:?})",
        line1_start.1 > line2_col_minmax.1,
        line1_start.0,
        line2_col_minmax.1
    );
    println!(
        "l1s < l2rm {:?} ({:?} < {:?})",
        line2_start.0 < line1_row_minmax.0,
        line2_start.0,
        line1_row_minmax.0
    );
    println!(
        "l1s > l2rm {:?} ({:?} > {:?})",
        line2_start.0 > line1_row_minmax.1,
        line2_start.0,
        line1_row_minmax.0
    );
    return (line1_start.1 < line2_col_minmax.0 && line1_start.1 > line2_col_minmax.1)
        && (line2_start.0 < line1_row_minmax.0 && line2_start.0 > line1_row_minmax.1);
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
    let mut start = input[0];
    let res = input
        .iter()
        .map(|pos| {
            input
                .iter()
                .filter(|pos2| pos.0 != pos2.0 && pos.1 != pos2.1)
                .map(|pos2| {
                    println!("{:?} to {:?}", pos, pos2);
                    println!(
                        "{:?}, {:?}",
                        pos.get_corners(pos2),
                        pos.corners_shrunk(pos2)
                    );
                    let corners = pos.corners_shrunk(pos2);
                    let mut iter = corners.iter().peekable();
                    while let Some(corner) = iter.next() {
                        let goal = iter.peek().unwrap_or(&pos);
                        let mut input_iter = input.iter().peekable();
                        while let Some(start) = input_iter.next() {
                            let end = input_iter.peek().unwrap_or(&start);
                            if line_crosses_line(start, end, corner, goal) {
                                return None;
                            }
                            // println!("no cross");
                        }
                    }
                    println!("{:?} to {:?} ({:?})", pos, pos2, pos.area(pos2));
                    Some(pos.area(pos2))
                })
                .filter_map(|area| area)
                // .inspect(|c| println!("{:?}", c))
                // .map(|corners| corners[0].area(&corners[1]))
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

    // #[test]
    // fn part2_community_1() {
    //     assert_eq!(part2(&parse(COMMUNITY_1)), 30);
    // }

    // #[test]
    // fn part2_community_2() {
    //     assert_eq!(part2(&parse(COMMUNITY_2)), 88);
    // }

    // #[test]
    // fn part2_community_3() {
    //     assert_eq!(part2(&parse(COMMUNITY_3)), 72);
    // }

    #[test]
    fn zero_multiple_2() {
        assert_eq!(0_usize.is_multiple_of(2), true);
    }
}
