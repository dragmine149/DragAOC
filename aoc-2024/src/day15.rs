use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Wall,
    Box,
    Robot,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pos: Position,
}

impl Location {
    fn get_score(&self) -> u64 {
        if self.cell != CellType::Box {
            return 0;
        }

        100 * self.pos.0 as u64 + self.pos.1 as u64
    }

    fn printable_type(&self) -> char {
        match self.cell {
            CellType::Box => 'O',
            CellType::Empty => '.',
            CellType::Robot => '@',
            CellType::Wall => '#',
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Position,
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

    fn try_move(&mut self, grid: &mut Vec<Vec<Location>>, direction: Direction) -> bool {
        let empty = self.find_empty_cell(grid, direction);
        if empty.is_none() {
            return false;
        }

        self.move_cells(grid, empty.expect("Failed to get cell somehow?"), direction);

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

#[aoc_generator(day15)]
fn parse(input: &str) -> (Vec<Vec<Location>>, Vec<Direction>, Robot) {
    let mut grid: Vec<Vec<Location>> = vec![];
    let mut movement: Vec<Direction> = vec![];
    let mut robot = Robot {
        pos: Position(0, 0),
    };

    input
        .trim()
        .lines()
        .skip_while(|line| line.is_empty())
        .enumerate()
        .for_each(|(line_index, line)| {
            let mut line_map = vec![];

            line.chars()
                .enumerate()
                .for_each(|(char_index, char)| match char {
                    '#' => line_map.push(Location {
                        cell: CellType::Wall,
                        pos: Position(line_index, char_index),
                    }),
                    'O' => line_map.push(Location {
                        cell: CellType::Box,
                        pos: Position(line_index, char_index),
                    }),
                    '@' => {
                        line_map.push(Location {
                            cell: CellType::Robot,
                            pos: Position(line_index, char_index),
                        });
                        robot.pos = Position(line_index, char_index);
                    }
                    '.' => line_map.push(Location {
                        cell: CellType::Empty,
                        pos: Position(line_index, char_index),
                    }),
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
    (grid, movement, robot)
}

#[aoc(day15, part1)]
fn part1(input: &(Vec<Vec<Location>>, Vec<Direction>, Robot)) -> u64 {
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
    grid.par_iter()
        .map(|line| line.par_iter().map(|pos| pos.get_score()).sum::<u64>())
        .sum()
}

// #[aoc(day15, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 10092);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 2028);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
