use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Write;

/// stored as (x, y)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize, pub usize);
#[derive(Clone)]
pub struct Grid(pub Vec<Vec<Cell>>);
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Paper,
    Empty,
}
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Grid {
    pub fn get_cell(&self, pos: Position) -> Option<Cell> {
        if let Some(row) = self.0.get(pos.1)
            && let Some(cell) = row.get(pos.0)
        {
            return Some(*cell);
        }
        None
    }

    pub fn clear_cell(&mut self, pos: Position) {
        self.0[pos.1][pos.0] = Cell::Empty;
    }

    pub fn get_access_positions(&self) -> Vec<Position> {
        let grid_size = Position(self.0.len(), self.0.first().unwrap().len());
        self.0
            .iter()
            .enumerate()
            .flat_map(|line| {
                line.1
                    .iter()
                    .enumerate()
                    .filter(|space| *space.1 == Cell::Paper)
                    .filter_map(|space| {
                        let pos = Position(space.0, line.0);

                        let paper = pos
                            .adjacent_eight(grid_size)
                            .iter()
                            .map(|c| match c {
                                Some(pos) => match self.get_cell(*pos) {
                                    Some(cell) => match cell {
                                        Cell::Empty => 0,
                                        Cell::Paper => 1,
                                    },
                                    None => 0,
                                },
                                None => 0,
                            })
                            .sum::<u64>();
                        if paper < 4 { Some(pos) } else { None }
                    })
                    .collect::<Vec<Position>>()
            })
            .collect()
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|y| {
                    let mut yy = y.iter().fold(String::new(), |mut output, x| {
                        // allow each cell to debug how this wish to debug.
                        if f.alternate() {
                            let _ = write!(output, "{:#?}", x);
                            output
                        } else {
                            let _ = write!(output, "{:?}", x);
                            output
                        }
                    });
                    yy.push('\n');
                    yy
                })
                .collect::<String>()
        )
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Paper => write!(f, "@"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl Position {
    fn process_direction(&self, direction: Direction, grid_size: Position) -> Option<Position> {
        match direction {
            Direction::North => {
                if self.1 > 0 {
                    let mut clone = *self;
                    clone.1 -= 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::NorthEast => {
                if self.1 > 0 && self.0 < grid_size.0 {
                    let mut clone = *self;
                    clone.1 -= 1;
                    clone.0 += 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::East => {
                let mut clone = *self;
                if self.0 < grid_size.0 {
                    clone.0 += 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::SouthEast => {
                let mut clone = *self;
                if self.0 < grid_size.0 && self.1 < grid_size.1 {
                    clone.1 += 1;
                    clone.0 += 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::South => {
                let mut clone = *self;
                if self.1 < grid_size.1 {
                    clone.1 += 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::SouthWest => {
                let mut clone = *self;
                if self.0 > 0 && self.1 < grid_size.1 {
                    clone.0 -= 1;
                    clone.1 += 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::West => {
                let mut clone = *self;
                if self.0 > 0 {
                    clone.0 -= 1;
                    Some(clone)
                } else {
                    None
                }
            }
            Direction::NorthWest => {
                let mut clone = *self;
                if self.0 > 0 && self.1 > 0 {
                    clone.0 -= 1;
                    clone.1 -= 1;
                    Some(clone)
                } else {
                    None
                }
            }
        }
    }

    pub fn adjacent_eight(&self, grid_size: Position) -> Vec<Option<Position>> {
        vec![
            self.process_direction(Direction::North, grid_size),
            self.process_direction(Direction::NorthEast, grid_size),
            self.process_direction(Direction::East, grid_size),
            self.process_direction(Direction::SouthEast, grid_size),
            self.process_direction(Direction::South, grid_size),
            self.process_direction(Direction::SouthWest, grid_size),
            self.process_direction(Direction::West, grid_size),
            self.process_direction(Direction::NorthWest, grid_size),
        ]
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    let g = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Cell::Empty,
                    '@' => Cell::Paper,
                    _ => panic!("Invalid character: {:?}", char),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();
    Grid(g)
}

#[aoc(day4, part1)]
fn part1(input: &Grid) -> usize {
    input.get_access_positions().len()
}

#[aoc(day4, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut total_clear = 0;
    let mut to_clear = grid.get_access_positions();
    while !to_clear.is_empty() {
        total_clear += to_clear.len();
        while let Some(pos) = to_clear.pop() {
            grid.clear_cell(pos);
        }
        to_clear = grid.get_access_positions();
    }

    total_clear
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            )),
            13
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            )),
            43
        );
    }
}
