use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub struct Positions(isize, isize);
impl Positions {
    fn area(&self, other: &Positions) -> isize {
        let height = (self.0 - other.0).abs() + 1;
        let width = (self.1 - other.1).abs() + 1;
        height * width
    }
}
impl From<&str> for Positions {
    fn from(value: &str) -> Self {
        let mut splits = value.split(",");
        Self(
            splits
                .next()
                .expect("Failed to get first item")
                .parse::<isize>()
                .expect("Failed to parse to isize"),
            splits
                .next()
                .expect("Failed to get second item")
                .parse::<isize>()
                .expect("Failed to parse to isize"),
        )
    }
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

// #[aoc(day9, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
            )),
            50
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
