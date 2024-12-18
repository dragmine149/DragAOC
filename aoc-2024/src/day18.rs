use crate::utils::{Grid, Position};
use aoc_runner_derive::aoc;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Memory,
}

#[derive(Clone, Copy)]
struct Cell {
    tpe: CellType,
    score: usize,
    visited: bool,
}

impl Cell {
    fn new(tpe: CellType) -> Self {
        Self {
            tpe,
            score: 0,
            visited: false,
        }
    }
}

impl fmt::Debug for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellType::Empty => '.',
                CellType::Memory => '#',
            }
        )
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.tpe))
    }
}

impl Grid<Cell> {
    #[allow(dead_code)]
    fn debug_score(&self) {
        let map = self
            .iter()
            .map(|y| {
                let mut yy = y
                    .iter()
                    .map(|x| format!("{:>3} ", x.score))
                    .collect::<String>();
                yy.push('\n');
                yy
            })
            .collect::<String>();
        println!("{}", map);
    }
}

fn parse(input: &str, map_size: Position, byte_count: usize) -> Grid<Cell> {
    let mut grid = Grid::new(map_size, Cell::new(CellType::Empty));
    for (line_index, line) in input.lines().enumerate() {
        if line_index >= byte_count {
            break;
        }
        let pos = line
            .split(",")
            .map(|v| v.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();

        let position = Position::from(&pos);
        let cell = grid.get_cell(position);
        cell.tpe = CellType::Memory;
    }

    grid
}

fn process_part1(input: &str, size: Position, byte_count: usize) -> u64 {
    let mut grid = parse(input, size, byte_count);
    // println!("{:?}", grid);
    let mut stack = vec![(Position(0, 0), 0)];

    while let Some(info) = stack.pop() {
        let pos = info.0;
        let score = info.1;
        // println!("Looking at: {:?}", pos);
        let cell = grid.get_cell(pos);
        if cell.visited && cell.score <= score {
            continue;
        }
        if cell.tpe == CellType::Memory {
            continue;
        }

        cell.score = score;
        cell.visited = true;

        let positions = pos.get_valid_positions(&Position(size.0 - 1, size.1 - 1));
        for posit in positions {
            stack.push((posit, score + 1));
        }
    }

    // grid.debug_score();
    grid.get_cell(Position(size.0 - 1, size.1 - 1)).score as u64
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    process_part1(input, Position(71, 71), 1024)
}

fn process_part2(input: &str, size: Position) -> String {
    let mut grid = parse(input, size, 1024);

    "0,0".to_string()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> String {
    process_part2(input, Position(71, 71))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn part1_example() {
        assert_eq!(process_part1(&EXAMPLE_1, Position(7, 7), 12), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(process_part2(&EXAMPLE_1, Position(7, 7)), "6,1");
    }
}
