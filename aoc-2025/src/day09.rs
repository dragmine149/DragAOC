use aoc_runner_derive::{aoc, aoc_generator};
use geo::{Contains, Coord, LineString, Polygon, Rect};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
impl From<Positions> for Coord {
    fn from(value: Positions) -> Self {
        Coord {
            x: value.0 as f64,
            y: value.1 as f64,
        }
    }
}
impl From<&Positions> for Coord {
    fn from(value: &Positions) -> Self {
        Coord {
            x: value.0 as f64,
            y: value.1 as f64,
        }
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

#[aoc(day9, part2)]
fn part2(input: &[Positions]) -> usize {
    let polygon = Polygon::new(
        LineString::from(input.iter().map(Coord::from).collect_vec()),
        vec![],
    );
    // pre calculate as this is really quick
    let input_map = input.iter().tuple_combinations::<(_, _)>().collect_vec();

    // and this is really slow.
    input_map
        .par_iter()
        .map(|corner| Rect::new(corner.0, corner.1))
        // .inspect(|a| println!("{:?}", a))
        // .inspect(|rect| {
        //     println!(
        //         "{:?} contains {:?} (area: {:?}, width: {:?}, height: {:?})",
        //         rect,
        //         polygon.contains(rect),
        //         rect.signed_area(),
        //         rect.width(),
        //         rect.height()
        //     )
        // })
        .map(|rect| {
            if polygon.contains(&rect) {
                ((rect.width() as usize) + 1) * ((rect.height() as usize) + 1)
            } else {
                0
            }
        })
        .max()
        .unwrap()
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
    fn zero_multiple_2() {
        assert_eq!(0_usize.is_multiple_of(2), true);
    }
}
