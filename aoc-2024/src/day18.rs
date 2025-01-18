use crate::utils::{Grid, Position};
use aoc_runner_derive::aoc;
use core::fmt;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;

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
        write!(f, "{:?}", self.tpe)
    }
}

impl Grid<Cell> {
    #[allow(dead_code)]
    fn debug_score(&self) {
        let map = self
            .iter()
            .map(|y| {
                let mut yy = y.iter().fold(String::new(), |mut out, x| {
                    let _ = write!(out, "{:>3}", x.score);
                    out
                });
                yy.push('\n');
                yy
            })
            .collect::<String>();
        println!("{}", map);
    }

    fn reset(&mut self, size: &Position) {
        for y in 0..size.1 {
            for x in 0..size.0 {
                let cell = self.get_cell(&Position(x, y));
                cell.score = 0;
                cell.visited = false;
            }
        }
    }
}

fn parse(input: &str, map_size: Position, byte_count: usize) -> Grid<Cell> {
    let mut grid = Grid::new(&map_size, Cell::new(CellType::Empty)); // make a big empty grid
    for (line_index, line) in input.lines().enumerate() {
        // loop though all lines
        if line_index >= byte_count {
            // if we at end of our input, stop
            break;
        }
        // get the location
        let pos = line
            .split(",")
            .map(|v| v.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();

        // set that cell to the memory type
        let position = Position::from(&pos);
        let cell = grid.get_cell(&position);
        cell.tpe = CellType::Memory;
    }

    grid
}

fn pathfinding(grid: &mut Grid<Cell>, size: Position) {
    grid.reset(&size);
    // let mut stack = vec![(Position(0, 0), 0)]; // stack for location finding
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((Position(0, 0), 0)));

    // searching algorith, give each cell the lowest possible score
    while let Some(info) = heap.pop() {
        let pos = info.0 .0;
        let score = info.0 .1;
        // println!("Looking at: {:?}", pos);
        let cell = grid.get_cell(&pos);
        if cell.visited && cell.score <= score {
            continue;
        }
        // don't glitch though walls
        if cell.tpe == CellType::Memory {
            continue;
        }

        cell.score = score;
        cell.visited = true;

        // find neighbours and repeat
        let positions = pos.get_neighbours(&Position(size.0, size.1), false);
        for posit in positions {
            heap.push(Reverse((posit, score + 1)));
        }
    }
}

fn process_part1(input: &str, size: Position, byte_count: usize) -> u64 {
    let mut grid = parse(input, size, byte_count);
    // println!("{:?}", grid);
    pathfinding(&mut grid, size);
    // grid.debug_score();
    // return the end cell location as the score
    // grid.debug_score();
    grid.get_cell(&Position(size.0 - 1, size.1 - 1)).score as u64
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    process_part1(input, Position(71, 71), 1024)
}

#[allow(dead_code)]
fn process_part2(input: &str, size: Position, mut index: usize) -> String {
    // let mut grid = parse(input, size, 1024);
    let mut score = u64::MAX; // set to max so it's not 0
    while score != 0 {
        index += 1; // keep going one iteration at a time
                    // println!("Iteration: {:?}", index);
        score = process_part1(input, size, index);
    }

    println!("{:?}", index);

    input
        .lines()
        .nth(index - 1)
        .expect("Failed to get line at specified index")
        .to_string()
}

fn get_path(grid: &mut Grid<Cell>, size: Position) -> Vec<Position> {
    pathfinding(grid, size);
    // println!("{:#?}", grid);
    // grid.debug_score();

    let end_cell = Position(size.0 - 1, size.1 - 1);
    let mut pos = end_cell;
    // if we have no score at the end, then the path did not computer, hence break.
    if grid.get_cell(&pos).score == 0 {
        return vec![];
    }

    let mut path = vec![pos];

    // start at the end and work to the front
    let mut score = grid.get_cell(&pos).score;
    while score != 0 {
        let options = pos.get_neighbours(&size, false);
        for option in options {
            let cell = grid.get_cell(&option);
            // println!("{:?} opt: {:?}", pos, option);
            // println!(
            //     "cell: {:?} score: {:?}, t: {:?} type: {:?}",
            //     cell.score + 1,
            //     score,
            //     cell.score + 1 != score,
            //     cell.tpe
            // );
            if cell.tpe == CellType::Memory {
                // println!("Memory");
                continue;
            }
            if cell.score != score - 1 {
                // println!("Score ({:?}, {:?})", cell.score, score);
                continue;
            }
            // println!("Found: {:?}", cell);
            // Only care about cells that are one less than us.
            pos = option;
            path.push(pos);
            score = cell.score;
            break;
        }
    }

    path
}

fn speed_part2(input: &str, size: Position, mut byte_count: usize) -> String {
    let mut grid = parse(input, size, byte_count);
    let mut path = get_path(&mut grid, size);

    // println!("e!");
    // println!("{:?}", grid);

    while !path.is_empty() {
        byte_count += 1;
        // println!("Byte: {:?}", byte_count);
        // println!("Path: {:?}", path);

        // make the byte fall
        let pos = Position::from(
            &input
                .lines()
                .nth(byte_count - 1)
                .expect("Failed to get byte count line")
                .split(",")
                .map(|x| x.parse::<usize>().expect("Failed to parse line coord"))
                .collect::<Vec<usize>>(),
        );
        // println!("{:?}", pos);
        grid.get_cell(&pos).tpe = CellType::Memory;

        if path.contains(&pos) {
            // println!("Regenerating path...");
            // If the byte has affected our current path, regenerate it.
            // Possible could only regenerate a short section but this is easier.
            path = get_path(&mut grid, size);
        }
    }

    input
        .lines()
        .nth(byte_count - 1)
        .expect("Failed to get line at speicfied index")
        .to_string()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> String {
    // process_part2(input, Position(71, 71), 1024)
    speed_part2(input, Position(71, 71), 1024)
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
        assert_eq!(process_part1(EXAMPLE_1, Position(7, 7), 12), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(process_part2(EXAMPLE_1, Position(7, 7), 12), "6,1");
    }

    #[test]
    fn part2_example_test() {
        assert_eq!(speed_part2(EXAMPLE_1, Position(7, 7), 12), "6,1");
    }
}
