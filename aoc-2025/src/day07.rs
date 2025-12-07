use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::{Debug, Display},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::Array2;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Cell {
    Empty,
    Split,
    #[default]
    Visited,
    Start,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Position(usize, usize);
#[derive(Debug, PartialEq, Eq)]
pub struct TimedPosition(Position, usize);

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '^' => Cell::Split,
            '|' => Cell::Visited,
            'S' => Cell::Start,
            _ => panic!("Invalid character! ({:?})", value),
        }
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Split => write!(f, "^"),
            Cell::Visited => write!(f, "|"),
            Cell::Start => write!(f, "S"),
        }
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Split => write!(f, "^"),
            Cell::Visited => write!(f, "|"),
            Cell::Start => write!(f, "S"),
        }
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            return Ordering::Less;
        }
        if self.0 < other.0 {
            return Ordering::Greater;
        }
        if self.1 > other.1 {
            return Ordering::Less;
        }
        if self.1 < other.1 {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Position {
    fn to_slice(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}
impl Ord for TimedPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 > other.1 {
            return Ordering::Less;
        }
        if self.1 < other.1 {
            return Ordering::Greater;
        }
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for TimedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl TimedPosition {
    fn to_slice(&self) -> (usize, usize) {
        self.0.to_slice()
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> (Array2<Cell>, usize) {
    let grid = input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect_vec())
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();
    let start = grid[0]
        .iter()
        .position(|c| *c == Cell::Start)
        .expect("Failed to find start position in first row!");
    let array = grid.into_iter().flatten().collect_vec();

    (
        Array2::from_shape_vec((height, width), array).expect("Failed to construct grid"),
        start,
    )
}

#[aoc(day7, part1)]
fn part1(input: &(Array2<Cell>, usize)) -> usize {
    // println!("{:#?}", input);
    let mut array = input.0.to_owned();
    // (row, col)
    let mut search = BinaryHeap::new();
    search.push(Position(1_usize, input.1));
    let mut count = 0;

    while let Some(pos) = search.pop() {
        let cell = array.get(pos.to_slice());
        if cell == None {
            // ignore dead out of bounds cells
            continue;
        }

        match cell.unwrap() {
            Cell::Empty => {
                array[pos.to_slice()] = Cell::Visited;
                search.push(Position(pos.0 + 1, pos.1));
            }
            Cell::Split => {
                let left = (pos.0, pos.1 - 1);
                let right = (pos.0, pos.1 + 1);

                count += (array.get(left) == Some(&Cell::Empty)
                    || array.get(right) == Some(&Cell::Empty)) as usize;
                // println!("Split total: {:?}", count);
                array[left] = Cell::Visited;
                array[right] = Cell::Visited;
                search.push(Position(left.0 + 1, left.1));
                search.push(Position(right.0 + 1, right.1));
                if array.get(left) == Some(&Cell::Empty) {}
                if array.get(right) == Some(&Cell::Empty) {}
            }
            Cell::Visited => continue,
            Cell::Start => panic!("How is there another start cell? ({:?})", pos),
        }

        // println!("{:?}", array);
        // println!("{:?}", search);
    }

    count
}

#[aoc(day7, part2)]
fn part2(input: &(Array2<Cell>, usize)) -> usize {
    // println!("{:#?}", input);
    let mut array = input.0.to_owned();
    // (row, col)
    let mut search = BinaryHeap::new();
    search.push(TimedPosition(Position(1_usize, input.1), 0));
    let mut count = 0;
    let mut splits = 0;

    while let Some(pos) = search.pop() {
        // if pos.0.0 == 16 {
        //     continue;
        // }

        // end of timeline.
        let cell = array.get(pos.to_slice());
        if cell == None {
            // ignore dead out of bounds cells
            count += 1;
            continue;
        }

        match cell.unwrap() {
            Cell::Empty => {
                search.push(TimedPosition(Position(pos.0.0 + 1, pos.0.1), pos.1));
            }
            Cell::Split => {
                let left = (pos.0.0, pos.0.1 - 1);
                let right = (pos.0.0, pos.0.1 + 1);

                splits += 1;
                // println!("Split total: {:?}", count);
                search.push(TimedPosition(Position(left.0 + 1, left.1), pos.1));
                search.push(TimedPosition(
                    Position(right.0 + 1, right.1),
                    pos.1 + splits,
                ));
            }
            Cell::Visited => continue,
            Cell::Start => panic!("How is there another start cell? ({:?})", pos),
        }

        // let s = array[pos.0.to_slice()].clone();
        // array[pos.0.to_slice()] = Cell::Visited;
        // println!("{:?}", array);
        // println!("{:?}", search);
        // array[pos.0.to_slice()] = s;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 40);
    }
}
