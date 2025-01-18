use crate::utils::{Direction, Grid, Position};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Visited,
    Start,
    End,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => '.',
                Cell::Wall => '#',
                Cell::Visited => 'O',
                Cell::Start => 'S',
                Cell::End => 'E',
            }
        )
    }
}

impl Direction {
    // in order to go that way it costs. Returns the amount of score depending on the cost.
    fn get_score_from_rotation(&self, new: Self) -> u64 {
        match self {
            Direction::North => match new {
                Direction::North => 1,
                Direction::East => 1001,
                Direction::South => 2001,
                Direction::West => 1001,
                _ => panic!("Invalid direction"),
            },
            Direction::East => match new {
                Direction::North => 1001,
                Direction::East => 1,
                Direction::South => 1001,
                Direction::West => 2001,
                _ => panic!("Invalid direction"),
            },
            Direction::South => match new {
                Direction::North => 2001,
                Direction::East => 1001,
                Direction::South => 1,
                Direction::West => 1001,
                _ => panic!("Invalid direction"),
            },
            Direction::West => match new {
                Direction::North => 1001,
                Direction::East => 2001,
                Direction::South => 1001,
                Direction::West => 1,
                _ => panic!("Invalid direction"),
            },
            _ => panic!("Invalid direction"),
        }
    }
}

trait Count {
    fn corner_count(&self) -> u64;
    fn forward_score(&self) -> u64;
}

impl Count for u64 {
    fn corner_count(&self) -> u64 {
        self / 1000
    }

    fn forward_score(&self) -> u64 {
        self % 1000
    }
}

// using the current score, check if any path is either the same or 1 turn away and is either the same or 1 score away
fn get_directions_from_score<'a>(
    cells: &'a HashMap<(Position, Direction), u64>,
    position: &'a Position,
    score: u64,
) -> impl IntoIterator<Item = (Position, Direction, u64)> + 'a {
    Direction::all(false)
        .into_iter()
        .flat_map(|dir| {
            let p = position.next_pos(&dir.inverse());
            cells
                .iter()
                .filter(|c| c.0 .0 == p)
                .map(|c| (c.0 .0, c.0 .1, *c.1))
                .collect_vec()
        })
        .filter(move |(_pos, _dir, dir_score)| {
            (dir_score.forward_score() == score.forward_score()
                || dir_score.forward_score() == score.forward_score() - 1)
                && (dir_score.corner_count() == score.corner_count()
                    || dir_score.corner_count() == score.corner_count() - 1)
        })
}

// Search in reverse to get the count of all the best paths
fn get_best_path_count(cells: HashMap<(Position, Direction), u64>, goal: Position) -> u64 {
    let valid_cells = cells
        .iter()
        .filter(|c| {
            c.1 <= cells
                .iter()
                .filter(|c| c.0 .0 == goal)
                .map(|c| c.1)
                .min()
                .unwrap_or(&0)
        })
        .map(|((p, d), s)| ((*p, *d), *s))
        .collect::<HashMap<(Position, Direction), u64>>();

    let mut path: BinaryHeap<(Position, Direction)> = BinaryHeap::new();
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    valid_cells.iter().filter(|c| c.0 .0 == goal).for_each(|c| {
        path.push(*c.0);
    });

    while let Some((pos, dir)) = path.pop() {
        visited.insert((pos, dir));
        let cell_score = valid_cells
            .iter()
            .filter(|c| c.0 .0 == pos && c.0 .1 == dir)
            .map(|c| *c.1)
            .min()
            .unwrap_or(u64::MAX);
        if cell_score == 0 {
            // well, we've reached a 0 cell. Most likely the start
            continue;
        }

        get_directions_from_score(&valid_cells, &pos, cell_score)
            .into_iter()
            .filter(|(pos, dir, _)| !visited.contains(&(*pos, *dir)))
            .for_each(|(pos, dir, _)| {
                path.push((pos, dir));
            });
    }

    let mut result: HashSet<Position> = HashSet::new();
    visited.iter().for_each(|v| {
        result.insert(v.0);
    });

    result.len() as u64
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Data {
    pos: Position,
    dir: Direction,
    score: u64,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Visits every single cell and gives them a score
fn get_cells(map: &Grid<Cell>) -> HashMap<(Position, Direction), u64> {
    // find the start cell and map size
    let s = map.cell_position(|v: Cell| v == Cell::Start).unwrap();
    let grid_size = &map.get_size();

    // set up the storages
    let mut positions: HashMap<(Position, Direction), u64> = HashMap::new();
    let mut to_seach = BinaryHeap::new();
    let mut previous: Data = Data {
        pos: Position::empty(),
        dir: Direction::East,
        score: 0,
    };
    to_seach.push(Data {
        pos: s,
        dir: Direction::East,
        score: 0,
    });
    // looping, using smallest score first
    while let Some(data) = to_seach.pop() {
        // In case it gets added during the loop, just check the score once again here.
        // Otherwise we get some really weird cases.
        // Would love to check during the loop, but that is harder to do without making things overly complicated.
        if positions.get(&(data.pos, data.dir)).unwrap_or(&u64::MAX) < &data.score {
            continue;
        }

        // valid cell and searched hence add
        positions.insert((data.pos, data.dir), data.score);
        data.pos
            // get possible directions
            .get_valid_directions(grid_size, false)
            .iter()
            // map with position
            .map(|dir| (data.pos.next_pos(dir), dir))
            // remove those positions which end up being blocked
            .filter(|(pos, _)| *map.get_cell_refrence(pos) != Cell::Wall)
            // Ignore the square we just came from. Kinda no need to back track.
            .filter(|(pos, _)| *pos != previous.pos)
            // remove those positions where we have already visited from that direction.
            // Score is not worried due to an eariler check and score being processed by smallest first.
            .filter(|(pos, dir)| !positions.contains_key(&(*pos, **dir)))
            // map everything
            .map(|(pos, new_dir)| {
                (
                    pos,
                    new_dir,
                    data.score + data.dir.get_score_from_rotation(*new_dir),
                )
            })
            // insert it back into the list
            .for_each(|(pos, dir, score)| {
                to_seach.push(Data {
                    pos,
                    dir: *dir,
                    score,
                });
            });
        previous = data;
    }
    positions
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Grid<Cell> {
    Grid::from_str(input, Cell::Empty, |c| match c {
        '#' => Cell::Wall,
        '.' => Cell::Empty,
        'S' => Cell::Start,
        'E' => Cell::End,
        'O' => Cell::Visited,
        _ => panic!("Invalid cell!"),
    })
}

#[aoc(day16, part1)]
fn part1(input: &Grid<Cell>) -> u64 {
    let goal = input.cell_position(|v: Cell| v == Cell::End).unwrap();
    let cells = get_cells(input);
    *cells
        .iter()
        .filter(|c| c.0 .0 == goal)
        .map(|c| c.1)
        .min()
        .unwrap_or(&0)
}

#[aoc(day16, part2)]
fn part2(input: &Grid<Cell>) -> u64 {
    let goal = input.cell_position(|v: Cell| v == Cell::End).unwrap();
    let cells = get_cells(input);

    get_best_path_count(cells, goal)
}

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

    const TEST_1: &str = "###########
#.#.#E....#
#.#.#####.#
#...#...#.#
###.#.#.#.#
#...#.#.#.#
#####.#.#.#
#...#.....#
#.#.#.###.#
#.#...#...#
#.###.#.#.#
#...#.#...#
#.#.#.###.#
#.#.#.....#
#.#.#####.#
#.#.#...#.#
###.#.#.#.#
#...#.#...#
#.#.#.###.#
#.#.#.#...#
#.###.#.#.#
#S......#.#
###########";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 7036);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 11048);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 45);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 64);
    }

    #[test]
    fn part1_test1() {
        assert_eq!(part1(&parse(TEST_1)), 4032);
    }

    #[test]
    fn part2_test1() {
        assert_eq!(part2(&parse(TEST_1)), 33);
    }
}
