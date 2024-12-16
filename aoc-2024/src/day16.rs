use aoc_runner_derive::{aoc, aoc_generator};
use core::fmt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Visited,
    Start,
    End,
    // North,
    // East,
    // South,
    // West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // Useful for quickly going in the opposite direction
    fn inverse(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }

    fn get_score_from_rotation(&self, new: Self) -> u64 {
        match self {
            Direction::North => match new {
                Direction::North => 1,
                Direction::East => 1001,
                Direction::South => 2001,
                Direction::West => 1001,
            },
            Direction::East => match new {
                Direction::North => 1001,
                Direction::East => 1,
                Direction::South => 1001,
                Direction::West => 2001,
            },
            Direction::South => match new {
                Direction::North => 2001,
                Direction::East => 1001,
                Direction::South => 1,
                Direction::West => 1001,
            },
            Direction::West => match new {
                Direction::North => 1001,
                Direction::East => 2001,
                Direction::South => 1001,
                Direction::West => 1,
            },
        }
    }

    // fn get_cell(&self) -> Cell {
    //     match self {
    //         Direction::North => Cell::North,
    //         Direction::East => Cell::East,
    //         Direction::South => Cell::South,
    //         Direction::West => Cell::West,
    //     }
    // }
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

impl Position {
    fn next_pos(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Position(self.0 - 1, self.1),
            Direction::East => Position(self.0, self.1 + 1),
            Direction::South => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0, self.1 - 1),
        }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Cell>>,
    score: Vec<Vec<u64>>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .to_owned()
                .into_par_iter()
                .map(|line| {
                    let mut a = line
                        .into_par_iter()
                        .map(|pos| match pos {
                            Cell::Wall => "#",
                            Cell::Empty => " ",
                            Cell::Start => "S",
                            Cell::End => "E",
                            Cell::Visited => "*",
                            // Cell::North => "^",
                            // Cell::East => ">",
                            // Cell::South => "v",
                            // Cell::West => "<",
                        })
                        .collect::<String>();
                    a.push('\n');
                    a
                })
                .collect::<String>()
        )
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '#' => Cell::Wall,
                        '.' => Cell::Empty,
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        _ => panic!("Invalid character in maze input"),
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        let score = vec![vec![0; grid.first().expect("Failed to get first row").len()]; grid.len()];
        Map { grid, score }
    }

    // fn debug_with_directions(&self, start: Position, directions: Vec<Direction>) {
    //     let mut map = self.to_owned();
    //     let mut pos = start;
    //     for direction in directions {
    //         map.grid[pos.0][pos.1] = direction.get_cell();
    //         pos = pos.next_pos(&direction);
    //     }

    //     println!("{:?}", map);
    // }

    fn visit_cell(&mut self, cell: &Position, score: u64) {
        if self.get_cell(cell) != Cell::Start && self.get_cell(cell) != Cell::End {
            self.grid[cell.0][cell.1] = Cell::Visited;
        }
        self.score[cell.0][cell.1] = score;
    }

    fn has_visited(&mut self, cell: &Position) -> bool {
        self.grid[cell.0][cell.1] == Cell::Visited
    }

    fn find_first_cell_of_type(&self, cell_type: &Cell) -> Position {
        for (line_index, line) in self.grid.iter().enumerate() {
            for (pos_index, pos) in line.iter().enumerate() {
                if pos == cell_type {
                    return Position(line_index, pos_index);
                }
            }
        }

        Position(0, 0)
    }

    fn get_cell(&self, pos: &Position) -> Cell {
        self.grid[pos.0][pos.1]
    }

    fn get_score(&self, pos: &Position) -> u64 {
        self.score[pos.0][pos.1]
    }

    fn can_move(&self, pos: &Position, dir: &Direction) -> bool {
        let destination = pos.next_pos(dir);
        // println!("{:?}", pos);
        // println!("{:?}", destination);
        self.get_cell(&destination) != Cell::Wall
    }

    fn get_possible_directions(&self, pos: &Position, from_dir: &Direction) -> Vec<Direction> {
        // println!("{:?}", self);

        let north = self.can_move(pos, &Direction::North);
        let east = self.can_move(pos, &Direction::East);
        let south = self.can_move(pos, &Direction::South);
        let west = self.can_move(pos, &Direction::West);

        // println!(
        //     "N: {:?}, E: {:?}, S: {:?}, W: {:?}",
        //     north, east, south, west
        // );

        let mut directions: Vec<Direction> = vec![];
        if north && from_dir != &Direction::North {
            directions.push(Direction::North);
        }
        if east && from_dir != &Direction::East {
            directions.push(Direction::East);
        }
        if south && from_dir != &Direction::South {
            directions.push(Direction::South);
        }
        if west && from_dir != &Direction::West {
            directions.push(Direction::West);
        }

        directions
    }
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    pos: Position,
    rot: Direction,
}

impl Reindeer {
    fn move_reindeer(&mut self, map: &mut Map, score: u64) {
        if map.get_cell(&self.pos) == Cell::End {
            // println!("Found end of map!");
            map.visit_cell(&self.pos, score);
            return;
        }

        if map.has_visited(&self.pos) {
            if map.get_score(&self.pos) <= score {
                // println!("Already visited: {:?} at a lower cost.", self.pos);
                return;
            }
        }

        map.visit_cell(&self.pos, score);

        let directions = map.get_possible_directions(&self.pos, &self.rot.inverse());
        // println!("Possible directions to go: {:?}", directions);
        if directions.is_empty() {
            // println!("Dead end reached!");
            return;
        }

        if directions.len() == 1 {
            // println!("Heading {:?} (only option)", directions[0]);
            let cost = self.rot.get_score_from_rotation(directions[0]);
            self.rot = directions[0];
            self.pos = self.pos.next_pos(&self.rot);

            self.move_reindeer(map, score + cost);
            return;
        }

        let original_dir = self.rot;
        let original_pos = self.pos;
        for direction in directions.iter() {
            // println!("Searching: {:?}", direction);

            let cost = original_dir.get_score_from_rotation(*direction);
            self.rot = *direction;
            self.pos = self.pos.next_pos(direction);

            self.move_reindeer(map, score + cost);
            self.pos = original_pos;
            self.rot = original_dir;
        }
    }

    // fn move_reindeer(&mut self, map: &mut Map) -> (u64, Vec<Direction>) {
    //     if map.get_cell(&self.pos) == Cell::End {
    //         println!("Found end of map!");
    //         return (0, vec![self.rot]);
    //     }

    //     if map.has_visited(self.pos) {
    //         println!("Already visited: {:?}. Cancelling route", self.pos);
    //         return (u64::MAX, vec![self.rot]);
    //     }

    //     map.visit_cell(self.pos);

    //     let directions = map.get_possible_directions(&self.pos, &self.rot.inverse());
    //     println!("Directions: {:?}", directions);
    //     if directions.is_empty() {
    //         println!("No directions to go, dead end reached");
    //         return (u64::MAX, vec![self.rot]);
    //     }

    //     if directions.len() == 1 {
    //         println!(
    //             "Only one direction so heading that way ({:?})",
    //             directions[0]
    //         );
    //         let current_dir = self.rot;
    //         self.rot = directions[0];
    //         self.pos = self.pos.next_pos(&directions[0]);
    //         let mut result = self.move_reindeer(map);
    //         result.1.push(directions[0]);
    //         if result.0 == u64::MAX {
    //             return result;
    //         }
    //         return (
    //             result.0 + current_dir.get_score_from_rotation(directions[0]),
    //             result.1,
    //         );
    //     }

    //     let route_score = directions
    //         .iter()
    //         .map(|direction| {
    //             println!("Searching: {:?}", direction);
    //             let mut r = Reindeer {
    //                 pos: self.pos.next_pos(direction),
    //                 rot: *direction,
    //             };
    //             let mut result = r.move_reindeer(map);
    //             result.1.push(*direction);
    //             if result.0 == u64::MAX {
    //                 return result;
    //             }
    //             (
    //                 result.0 + self.rot.get_score_from_rotation(*direction),
    //                 result.1,
    //             )
    //         })
    //         .collect::<Vec<(u64, Vec<Direction>)>>();

    //     let scores = route_score.iter().map(|r| r.0);
    //     let min = scores.min().expect("Failed to get minimum score");
    //     let results = route_score.iter().filter(|score| score.0 == min);
    //     let directions = results.to_owned().map(|r| r.1.len());
    //     let min_dir = directions.min().expect("Failed to get shortest direction");
    //     let result = results
    //         .filter(|r| r.1.len() == min_dir)
    //         .nth(0)
    //         .expect("Failed to get first value")
    //         .to_owned();

    //     println!("Score: {:?}", min);
    //     result
    // }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> (Map, Reindeer) {
    let map = Map::new(input);
    let reindeer_pos = map.find_first_cell_of_type(&Cell::Start);

    (
        map,
        Reindeer {
            pos: reindeer_pos,
            rot: Direction::East,
        },
    )
}

#[aoc(day16, part1)]
fn part1(input: &(Map, Reindeer)) -> u64 {
    let mut map = input.0.to_owned();
    let mut reindeer = input.1.to_owned();

    reindeer.move_reindeer(&mut map, 0);

    let goal = map.find_first_cell_of_type(&Cell::End);
    // println!("{:#?}", map.score);
    // println!("Goal: {:?}", goal);
    map.get_score(&goal)
}

// #[aoc(day16, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 7036);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 11048);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
