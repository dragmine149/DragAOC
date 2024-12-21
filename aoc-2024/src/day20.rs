use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fmt;

use crate::utils::{Grid, Position};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CellType {
    Wall,
    Empty,
}

impl fmt::Debug for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}",
                match self {
                    Self::Wall => "CellType::Wall",
                    Self::Empty => "CellType::Empty",
                }
            )
        } else {
            write!(
                f,
                "{}",
                match self {
                    Self::Wall => "#",
                    Self::Empty => " ",
                }
            )
        }
    }
}

#[derive(Clone, Copy)]
struct Cell {
    cell_type: CellType,
    score: u64,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Cell {{ cell_type: {:#?}, Score: {:#?} }}",
                self.cell_type, self.score
            )
        } else {
            write!(f, "{:?}", self.cell_type)
        }
    }
}

#[aoc_generator(day20)]
fn parse(input: &str) -> (Grid<Cell>, Position, Position) {
    let size = input
        .lines()
        .next()
        .expect("Failed to get first line")
        .len();
    let mut grid = Grid::new(
        &Position::from(size),
        Cell {
            cell_type: CellType::Empty,
            score: u64::MAX,
        },
    );

    let mut start = Position::empty();
    let mut end = Position::empty();

    for (line_index, line) in input.trim().lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let position = Position(char_index, line_index);
            match char {
                '#' => grid.get_cell(&position).cell_type = CellType::Wall,
                'S' => start = Position(char_index, line_index),
                'E' => end = Position(char_index, line_index),
                '.' => {}
                _ => {
                    println!("{:?}", char);
                    panic!("Invalid cell type");
                }
            }
        }
    }

    grid.get_cell(&start).score = 0;

    (grid, start, end)
}

fn get_default_time(grid: &mut Grid<Cell>, start: Position, end: Position) -> (u64, Vec<Position>) {
    // because map is a single line
    let mut current_pos = start;
    let mut score = 0;
    let mut path: Vec<Position> = vec![current_pos];
    while current_pos != end {
        // println!("Cell: {:?}", current_pos);
        let valid = current_pos.get_valid_positions(&grid.get_size());
        for v in valid {
            let cell = grid.get_cell(&v);
            // println!("{:#?}", cell);
            if cell.cell_type == CellType::Empty && cell.score == u64::MAX {
                current_pos = v;
                score += 1;
                path.push(v);
                cell.score = score;
                break;
            }
        }
    }
    (score, path)
}

fn cheat(grid: &Grid<Cell>, location: &Position) -> Vec<u64> {
    let grid_size: Position = grid.get_size();
    let valid = location.get_valid(&grid_size);
    let local_score = grid.get_unmut_cell(location).score;
    let mut score: Vec<u64> = vec![0, 0, 0, 0];

    for (dir, pos) in valid {
        let cell: Cell = grid.get_unmut_cell(&pos);
        if cell.cell_type == CellType::Empty {
            score[dir.index()] = u32::MAX as u64;
            continue;
        }

        if !pos.get_valid_directions(&grid_size).contains(&dir) {
            score[dir.index()] = u64::MAX;
            continue;
        }

        let next_pos_1 = pos.next_pos(&dir);
        if !next_pos_1.is_valid(&grid_size) {
            score[dir.index()] = u64::MAX;
            continue;
        }
        let next_pos_2 = next_pos_1.next_pos(&dir);
        if !next_pos_2.is_valid(&grid_size) {
            // Assumption: For this not to be valid it must be outside the map,
            score[dir.index()] = u64::MAX;
            continue;
        }

        // now we are 100% in the map
        let next_cell_1: Cell = grid.get_unmut_cell(&next_pos_1); // pos 2
        let next_cell_2: Cell = grid.get_unmut_cell(&next_pos_2); // after pos 2
        if next_cell_2.cell_type == CellType::Wall && next_cell_1.cell_type == CellType::Wall {
            // Segment falut
            score[dir.index()] = u64::MAX;
            continue;
        }

        if next_cell_2.cell_type == CellType::Empty && next_cell_1.cell_type == CellType::Wall {
            score[dir.index()] = next_cell_2
                .score
                .saturating_sub(local_score)
                .saturating_sub(2);
            continue;
        }
        if next_cell_1.cell_type == CellType::Empty {
            // println!(
            //     "Next: {:?}, Local: {:?} -2: -2",
            //     next_cell_1.score, local_score
            // );
            score[dir.index()] = next_cell_1
                .score
                .saturating_sub(local_score)
                .saturating_sub(2);
            continue;
        }
    }

    score
}

fn improved_cheat(grid: &Grid<Cell>, location: &Position, picoseconds: usize) -> u64 {
    let points = location.get_points_in_radius(picoseconds); // get radius, aka all points
    let local_score: u64 = grid.get_unmut_cell(location).score; // score of ourselfs for refrence
    let grid_size: Position = grid.get_size(); // grid size so we don't keep getting it later
    points
        .par_iter()
        // .iter()
        .filter(|point| point.0 < grid_size.0 && point.1 < grid_size.1) // filter out points which are out of the grid
        .map(|loc| {
            // get the Mahhattan distance
            let gap = Position(loc.0.abs_diff(location.0), loc.1.abs_diff(location.1));
            let distance = gap.0 + gap.1;

            // get the cell
            let cell: Cell = grid.get_unmut_cell(loc);
            if cell.cell_type == CellType::Wall {
                // segment error
                u64::MAX
            } else {
                // How much would we get from us to the cell minus the cheat time
                cell.score
                    .saturating_sub(local_score)
                    .saturating_sub(distance as u64)
            }
        })
        // .filter(|option| option != &0)
        // .filter(|option| option >= &50)
        .filter(|option| option >= &100 && *option != u64::MAX) // only top 100 and no segment error
        .count() as u64 // count
}

#[aoc(day20, part1)]
fn part1(input: &(Grid<Cell>, Position, Position)) -> u64 {
    let mut grid = input.0.to_owned();
    let start = input.1;
    let end = input.2;
    // println!("{:?}", grid);
    let (_default_score, path) = get_default_time(&mut grid, start, end);
    // println!("Default path score: {:?}", _default_score);

    path.iter()
        .map(|cell| {
            let options = cheat(&grid, cell);
            // println!("Pos {:?} has {:?}", cell, options);
            options
                .iter()
                .filter(|&&option| {
                    option != u64::MAX && option != u32::MAX as u64 && option != 0 && option >= 100
                })
                .count() as u64
        })
        .sum::<u64>()
}

#[aoc(day20, part2)]
fn part2(input: &(Grid<Cell>, Position, Position)) -> u64 {
    let mut grid = input.0.to_owned();
    let start = input.1;
    let end = input.2;
    // println!("{:?}", grid);
    let (_default_score, path) = get_default_time(&mut grid, start, end);
    // println!("Default path score: {:?}", _default_score);

    path.par_iter()
        // path.iter()
        .map(|cell| improved_cheat(&grid, cell, 20))
        .sum::<u64>()
    // + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 0);
    }
}
