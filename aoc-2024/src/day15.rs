use std::cmp::Ordering;

use aoc_runner_derive::aoc;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }

    fn is_lr(&self) -> bool {
        self == &Direction::East || self == &Direction::West
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position(usize, usize);

impl Position {
    fn next_pos(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.0 -= 1,
            Direction::East => self.1 += 1,
            Direction::South => self.0 += 1,
            Direction::West => self.1 -= 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Location {
    cell: CellType,
}

impl Location {
    fn printable_type(&self) -> char {
        match self.cell {
            CellType::Box => 'O',
            CellType::Empty => '.',
            CellType::Robot => '@',
            CellType::Wall => '#',
            CellType::BoxLeft => '[',
            CellType::BoxRight => ']',
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Position,
    debug: bool,
}

impl Robot {
    fn find_empty_cell(self, grid: &[Vec<Location>], direction: Direction) -> Option<Position> {
        let mut search_pos = self.pos;
        let mut cell = get_cell(grid, self.pos);
        while cell != CellType::Empty {
            search_pos.next_pos(direction);
            cell = get_cell(grid, search_pos);
            // println!("{:?} -> {:?}", search_pos, cell);

            if cell == CellType::Wall {
                // hit a wall, no more options aka stop.
                return None;
            }
        }
        match cell {
            CellType::Empty => Some(search_pos),
            _ => None,
        }
    }

    fn find_boxes(self, grid: &[Vec<Location>], direction: Direction) -> Option<Vec<Position>> {
        let mut search_pos = self.pos;
        search_pos.next_pos(direction);
        let mut boxes = vec![];

        let mut stack: Vec<Position> = vec![search_pos];
        while let Some(pos) = stack.pop() {
            let cell = get_cell(grid, pos);
            match cell {
                CellType::Wall => return None,
                CellType::Robot => panic!("How did we run into the robot?"),
                CellType::Box => panic!("How is there a 1x1 box here?"),
                CellType::BoxLeft => {
                    let mut forward = Position(pos.0, pos.1);
                    forward.next_pos(direction);
                    let mut right_forward = Position(pos.0, pos.1);
                    right_forward.next_pos(Direction::East);
                    right_forward.next_pos(direction);

                    stack.push(forward);
                    stack.push(right_forward);

                    if !boxes.contains(&forward) {
                        boxes.push(forward);
                    }
                    if !boxes.contains(&right_forward) {
                        boxes.push(right_forward);
                    }
                }
                CellType::BoxRight => {
                    let mut forward = Position(pos.0, pos.1);
                    forward.next_pos(direction);
                    let mut left_forward = Position(pos.0, pos.1);
                    left_forward.next_pos(Direction::West);
                    left_forward.next_pos(direction);

                    stack.push(forward);
                    stack.push(left_forward);

                    if !boxes.contains(&forward) {
                        boxes.push(forward);
                    }
                    if !boxes.contains(&left_forward) {
                        boxes.push(left_forward);
                    }
                }
                CellType::Empty => {}
            }
        }
        boxes.sort_by(|a, b| {
            if direction == Direction::North {
                if a.0 < b.0 {
                    return Ordering::Equal;
                }
                if a.0 > b.0 {
                    return Ordering::Less;
                }
            } else if direction == Direction::South {
                if a.0 > b.0 {
                    return Ordering::Equal;
                }
                if a.0 < b.0 {
                    return Ordering::Less;
                }
            }
            if a.0 > b.0 {
                return Ordering::Greater;
            }
            if a.0 == b.0 {
                return Ordering::Equal;
            }
            if a.0 < b.0 {
                return Ordering::Less;
            }

            Ordering::Equal
        });

        boxes.sort_by(|a, b| {
            if a.0 > b.0 {
                return if direction == Direction::South {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
            if a.1 < b.1 {
                return if direction == Direction::South {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }

            if direction == Direction::South {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        Some(boxes)
    }

    fn next_cell_empty(self, grid: &[Vec<Location>], direction: Direction) -> bool {
        let mut cpos = self.pos;
        cpos.next_pos(direction);
        get_cell(grid, cpos) == CellType::Empty
    }

    fn move_cells(
        &mut self,
        grid: &mut [Vec<Location>],
        empty_cell: Position,
        direction: Direction,
    ) {
        // println!(
        //     "Attempting to move from {:?} ({:?}) to {:?}",
        //     self.pos, direction, empty_cell
        // );
        let mut current_pos = empty_cell;
        let mut next_pos = empty_cell;
        while current_pos != self.pos {
            next_pos.next_pos(direction.inverse());
            grid[current_pos.0][current_pos.1].cell = get_cell(grid, next_pos);
            current_pos = next_pos;
        }
        grid[self.pos.0][self.pos.1].cell = CellType::Empty;
        self.pos.next_pos(direction);
    }

    fn improved_vertical_movement(
        &mut self,
        grid: &mut Vec<Vec<Location>>,
        boxes: Vec<Position>,
        direction: Direction,
    ) {
        let mut boxes = boxes.to_owned();
        for box_part in boxes.iter_mut() {
            box_part.next_pos(direction.inverse());
            if self.debug {
                println!("{:?}", box_part);
            }

            let original_type = get_cell(grid, *box_part);
            grid[box_part.0][box_part.1].cell = CellType::Empty;
            if self.debug {
                debug_map(grid);
            }

            box_part.next_pos(direction);
            grid[box_part.0][box_part.1].cell = original_type;

            if self.debug {
                println!("{:?}", box_part);
                debug_map(grid);
            }
        }

        self.move_if_empty(grid, direction);
    }

    fn move_if_empty(&mut self, grid: &mut Vec<Vec<Location>>, direction: Direction) {
        grid[self.pos.0][self.pos.1].cell = CellType::Empty;
        self.pos.next_pos(direction);
        grid[self.pos.0][self.pos.1].cell = CellType::Robot;
    }

    fn try_move(&mut self, grid: &mut Vec<Vec<Location>>, direction: Direction) -> bool {
        if self.next_cell_empty(grid, direction) {
            self.move_if_empty(grid, direction);
            return true;
        }

        let empty = self.find_empty_cell(grid, direction);
        if empty.is_none() {
            return false;
        }

        self.move_cells(grid, empty.expect("Failed to get cell somehow?"), direction);

        true
    }

    fn try_move_2(&mut self, grid: &mut Vec<Vec<Location>>, direction: Direction) -> bool {
        if self.next_cell_empty(grid, direction) {
            self.move_if_empty(grid, direction);
            return true;
        }

        if direction.is_lr() {
            return self.try_move(grid, direction);
        }

        let boxes = self.find_boxes(grid, direction);
        if boxes.is_none() {
            return false;
        }

        if self.debug {
            debug_map(grid);
            println!("{:?}", boxes);
        }
        self.improved_vertical_movement(grid, boxes.expect("Boxes are empty?"), direction);

        true
    }
}

fn get_cell(grid: &[Vec<Location>], pos: Position) -> CellType {
    grid[pos.0][pos.1].cell
}

#[allow(dead_code)]
fn debug_map(grid: &[Vec<Location>]) {
    let map = grid
        .par_iter()
        .map(|line| {
            let mut a = line
                .par_iter()
                .map(|cell| cell.printable_type())
                .collect::<String>();
            a.push('\n');
            a
        })
        .collect::<String>();
    println!("{}", map);
}

#[allow(dead_code)]
fn check_grid(grid: &[Vec<Location>], direction_count: usize) {
    // Par iter speeds this up slightly consedering how often we check it.
    grid.par_iter().for_each(|line| {
        let mut left = false;
        line.iter().for_each(|char| {
            if char.cell == CellType::BoxRight && !left {
                println!("Count: {:?}", direction_count);
                debug_map(grid);
                panic!("Grid is broken!");
            } else {
                left = false;
            }
            if char.cell != CellType::BoxRight && left {
                println!("Count: {:?}", direction_count);
                debug_map(grid);
                panic!("Grid is broken!");
            } else {
                left = false;
            }

            if char.cell == CellType::BoxLeft {
                left = true;
            }
        })
    });
}

fn parse(input: &str, p2: bool) -> (Vec<Vec<Location>>, Vec<Direction>, Robot) {
    let mut grid: Vec<Vec<Location>> = vec![];
    let mut movement: Vec<Direction> = vec![];
    let mut robot = Robot {
        pos: Position(0, 0),
        debug: false,
    };

    input
        .trim()
        .lines()
        .skip_while(|line| line.is_empty())
        .for_each(|line| {
            let mut line_map = vec![];

            line.chars().for_each(|char| match char {
                '#' => {
                    line_map.push(Location {
                        cell: CellType::Wall,
                    });
                    if p2 {
                        line_map.push(Location {
                            cell: CellType::Wall,
                        })
                    }
                }
                'O' => {
                    if p2 {
                        line_map.push(Location {
                            cell: CellType::BoxLeft,
                        });
                        line_map.push(Location {
                            cell: CellType::BoxRight,
                        })
                    } else {
                        line_map.push(Location {
                            cell: CellType::Box,
                        });
                    }
                }
                '@' => {
                    line_map.push(Location {
                        cell: CellType::Robot,
                    });
                    if p2 {
                        line_map.push(Location {
                            cell: CellType::Empty,
                        });
                    }
                }
                '.' => {
                    line_map.push(Location {
                        cell: CellType::Empty,
                    });
                    if p2 {
                        line_map.push(Location {
                            cell: CellType::Empty,
                        })
                    }
                }
                '<' => movement.push(Direction::West),
                'v' => movement.push(Direction::South),
                '>' => movement.push(Direction::East),
                '^' => movement.push(Direction::North),
                _ => panic!("Invalid character!"),
            });

            if !line_map.is_empty() {
                grid.push(line_map);
            }
        });

    grid.iter().enumerate().for_each(|(line_index, line)| {
        line.iter().enumerate().for_each(|(pos_index, pos)| {
            if pos.cell != CellType::Robot {
                return;
            }

            robot.pos = Position(line_index, pos_index);
        });
    });

    (grid, movement, robot)
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let input: (Vec<Vec<Location>>, Vec<Direction>, Robot) = parse(input, false);
    // println!("{:#?}", input);
    let mut robot = input.2;
    let directions = input.1.to_owned();
    let mut grid = input.0.to_owned();

    // debug_map(&grid);

    for direction in directions {
        robot.try_move(&mut grid, direction);
        // println!("Direction: {:?}", direction);
        // debug_map(&grid);
    }

    // debug_map(&grid);

    grid.iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.iter()
                .enumerate()
                .map(|(pos_index, pos)| {
                    if pos.cell == CellType::Box {
                        100 * line_index as u64 + pos_index as u64
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let input: (Vec<Vec<Location>>, Vec<Direction>, Robot) = parse(input, true);
    // println!("{:#?}", input);
    let mut robot = input.2;
    let directions = input.1.to_owned();
    let mut grid = input.0.to_owned();

    // debug_map(&grid);
    // println!("{:?}", directions.len());

    let mut direction_count = 0;
    for direction in directions {
        direction_count += 1;
        robot.try_move_2(&mut grid, direction);
        check_grid(&grid, direction_count);

        if direction_count >= 65536 {
            // if direction_count >= 6047 {
            robot.debug = true;
            println!("Direction: {:?}", direction);
            debug_map(&grid);
        }
    }

    // debug_map(&grid);
    grid.iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.iter()
                .enumerate()
                .map(|(pos_index, pos)| {
                    if pos.cell == CellType::BoxLeft {
                        100 * line_index as u64 + pos_index as u64
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const EXAMPLE_2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    const TEST_1: &str = "#######
#...#.#
#.....#
#..OO@#
#...O.#
#.....#
#######

v<>^^<<<<v
";

    const TEST_2: &str = "#######
#...#.#
#.....#
#..OO@#
#...O.#
#.....#
#######

v<>^^<v
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 10092);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(EXAMPLE_2), 2028);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_1), 9021);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(EXAMPLE_3), 618);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_1), 1221);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(part2(TEST_2), 1221);
    }
}
