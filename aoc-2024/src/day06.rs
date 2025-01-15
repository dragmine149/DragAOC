use crate::utils::{Direction, Grid, Position};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty,
    Obstruction,
    North,
    East,
    South,
    West,
    Visited,
    Block,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Obstruction => '#',
                Self::North => '^',
                Self::East => '>',
                Self::South => 'v',
                Self::West => '<',
                Self::Visited => 'X',
                Self::Block => 'O',
            }
        )
    }
}

impl From<Direction> for Cell {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self::North,
            Direction::East => Self::East,
            Direction::South => Self::South,
            Direction::West => Self::West,
            Direction::NorthEast => panic!("Invalid direction"),
            Direction::SouthEast => panic!("Invalid direction"),
            Direction::SouthWest => panic!("Invalid direction"),
            Direction::NorthWest => panic!("Invalid direction"),
        }
    }
}

impl From<Cell> for Direction {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => panic!("Invalid Cell"),
            Cell::Obstruction => panic!("Invalid Cell"),
            Cell::North => Self::North,
            Cell::East => Self::East,
            Cell::South => Self::South,
            Cell::West => Self::West,
            Cell::Visited => panic!("Invalid Cell"),
            Cell::Block => panic!("Invalid Cell"),
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (Grid<Cell>, Position) {
    let grid = Grid::from_str(input, Cell::Empty, |cell| match cell {
        '.' => Cell::Empty,
        '#' => Cell::Obstruction,
        '^' => Cell::North,
        '>' => Cell::East,
        'v' => Cell::South,
        '<' => Cell::West,
        'X' => Cell::Visited,
        'O' => Cell::Block,
        _ => panic!("Invalid cell!"),
    });

    // get the location of the guard
    let mut guard: Position = Position::empty();
    for (index, line) in input.lines().enumerate() {
        let guard_potential_pos = line.find('^');

        match guard_potential_pos {
            Some(pos) => {
                guard = Position(pos, index);
            }
            None => {
                continue;
            }
        }
    }

    (grid, guard)
}

// Move the guard around the map
// Takes: map, pos, direction
// Returns:
// - map: The map
// - move count: How many cells the guard moved this time
// - Is out of map: Is guard inside or outside of the border
// - Position: Position of the guard (or last position guard was in before going outside)
fn move_guard(
    map: &mut Grid<Cell>,
    mut pos: Position,
    direction: Direction,
) -> (u16, bool, Position) {
    let mut move_count: u16 = 0;
    loop {
        if !pos.is_next_valid(&direction, &map.get_size()) {
            return (move_count + 1, true, pos);
        }
        let next_dir_info = pos.next_pos(&direction);
        let square = map.get_unmut_cell(&next_dir_info);

        if square == Cell::Empty {
            // new square, mark it
            move_count += 1;
            // move guard by setting the next square to the guard and the previous square to an x.
            // This also accidently prevents an issue in p2
            map.set_cell(&next_dir_info, Cell::from(direction));
            map.set_cell(&pos, Cell::Visited);

            pos = next_dir_info;
            continue;
        }
        if square == Cell::Obstruction || square == Cell::Block {
            // the square is an object
            break;
        }
        if [Cell::North, Cell::East, Cell::South, Cell::West].contains(&square) {
            // another guard somehow
            panic!("Why is there another guard around?");
        }
        if square == Cell::Visited {
            // the square has already been visited

            map.set_cell(&next_dir_info, Cell::from(direction));
            map.set_cell(&pos, Cell::Visited);

            pos = next_dir_info;
            continue;
        }
        panic!("Unknown square!");
    }

    (move_count, false, pos)
}

// Normal guard movement
fn move_and_rotate_guard(input: &(Grid<Cell>, Position)) -> u16 {
    let mut is_out_map = false;
    let mut map = input.0.clone();
    let mut total_moves = 0;
    let mut guard_direction = Direction::North;
    let mut pos = input.1;

    while !is_out_map {
        // output_map(&map);
        let move_count;
        (move_count, is_out_map, pos) = move_guard(&mut map, pos, guard_direction);
        // println!("{:?}", map);
        // println!(
        //     "Guard moved {} and is {} in map",
        //     move_count,
        //     if is_out_map { "not" } else { "still" }
        // );
        total_moves += move_count;

        if !is_out_map {
            guard_direction = guard_direction.rotate(false);
        }
    }

    total_moves
}

fn move_and_path_guard(input: &(Grid<Cell>, Position)) -> Vec<Position> {
    let mut is_out_map = false;
    let mut map = input.0.clone();
    let mut guard_direction = Direction::North;
    let mut pos = input.1;
    let mut positions = vec![];

    while !is_out_map {
        let end_pos;
        (_, is_out_map, end_pos) = move_guard(&mut map, pos, guard_direction);

        pos.iter_positions(&end_pos).into_iter().for_each(|p| {
            if !positions.contains(&p) {
                positions.push(p)
            }
        });
        pos = end_pos;

        if !is_out_map {
            guard_direction = guard_direction.rotate(false);
        }
    }

    positions
}

// Try to block the guard by doing the same as the above but slightly different.
fn move_and_block_guard(input: &(Grid<Cell>, Position), obstruction_pos: Position) -> bool {
    let mut map = input.0.clone();
    map.set_cell(&obstruction_pos, Cell::Obstruction); // the square to block

    let mut guard_direction = Direction::North;
    let mut pos = input.1;
    let mut is_out_map = false;

    let mut visited: Vec<Position> = vec![pos]; // list of positions

    while !is_out_map {
        (_, is_out_map, pos) = move_guard(&mut map, pos, guard_direction);

        // If the guard has been here before break. (Pos is only when the guard rotates, and since we're back here we are 99% likely to be in a loop)
        // The `visited.last().expect("Failed to find last elm") != &pos` check prevents the guard from giving up if it has just rotated and already got stuck without moving.
        if visited.contains(&pos) && visited.last().expect("Failed to find last elm") != &pos {
            // output_map(&map);
            // println!("{:?}\t{:?}", visited, pos);
            // println!("Obstruction worked!");
            return true;
        }
        visited.push(pos);

        // println!("{:?}\t{:?}\t{:?}", visited, pos, is_out_map);
        if !is_out_map {
            guard_direction = guard_direction.rotate(false);
        }
    }
    // output_map(&map);
    // println!("Obstruction failed");
    false
}

#[aoc(day6, part1)]
fn part1(input: &(Grid<Cell>, Position)) -> u16 {
    move_and_rotate_guard(input)
}
#[aoc(day6, part2)]
fn part2(input: &(Grid<Cell>, Position)) -> u16 {
    move_and_path_guard(input)
        .par_iter()
        .map(|pos| {
            if input.0.get_unmut_cell(pos) == Cell::Obstruction {
                return 0;
            }

            let looped = move_and_block_guard(input, *pos);
            if looped {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 6);
    }
}
