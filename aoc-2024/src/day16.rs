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

#[derive(Debug, Clone, Copy)]
struct MapData {
    north: u64,
    east: u64,
    south: u64,
    west: u64,
}

impl MapData {
    fn new() -> Self {
        MapData {
            north: u64::MAX,
            east: u64::MAX,
            south: u64::MAX,
            west: u64::MAX,
        }
    }

    fn get_direction_score(self, direction: Direction) -> u64 {
        match direction {
            Direction::North => self.north,
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west,
        }
    }

    fn get_smallest(&self) -> (u64, Direction) {
        let mut smallest = (self.north, Direction::North);
        if self.east < smallest.0 {
            smallest = (self.east, Direction::East);
        }
        if self.south < smallest.0 {
            smallest = (self.south, Direction::South);
        }
        if self.west < smallest.0 {
            smallest = (self.west, Direction::West);
        }
        smallest
    }

    fn get_smallest_num(&self) -> u64 {
        self.get_smallest().0
    }

    fn set_direction(&mut self, direction: Direction, score: u64) {
        match direction {
            Direction::North => self.north = score,
            Direction::East => self.east = score,
            Direction::South => self.south = score,
            Direction::West => self.west = score,
        }
    }

    fn get_directions_from_score(&self, score: u64) -> Vec<Direction> {
        let corners = get_corner_count(score);
        let score = get_forward_score(score);
        let mut directions = vec![];

        println!();
        println!("Score: {:?}, Corners: {:?}", score, corners);
        println!("Self: {:?}", self);
        println!();

        if get_forward_score(self.north) == score || get_forward_score(self.north) == score - 1 {
            if get_corner_count(self.north) == corners
                || get_corner_count(self.north) == corners - 1
            {
                directions.push(Direction::South);
            }
        }
        if get_forward_score(self.east) == score || get_forward_score(self.east) == score - 1 {
            if get_corner_count(self.east) == corners || get_corner_count(self.east) == corners - 1
            {
                directions.push(Direction::West);
            }
        }
        if get_forward_score(self.south) == score || get_forward_score(self.south) == score - 1 {
            if get_corner_count(self.south) == corners
                || get_corner_count(self.south) == corners - 1
            {
                directions.push(Direction::North);
            }
        }
        if get_forward_score(self.west) == score || get_forward_score(self.west) == score - 1 {
            if get_corner_count(self.west) == corners || get_corner_count(self.west) == corners - 1
            {
                directions.push(Direction::East);
            }
        }

        directions
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Cell>>,
    score: Vec<Vec<MapData>>,
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
                        })
                        .collect::<String>();
                    a.push('\n');
                    a
                })
                .collect::<String>()
        )
    }
}

fn get_corner_count(num: u64) -> u64 {
    num / 1000
}

fn get_forward_score(num: u64) -> u64 {
    num % 1000
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
        let score = vec![
            vec![MapData::new(); grid.first().expect("Failed to get first row").len()];
            grid.len()
        ];
        Map { grid, score }
    }

    #[allow(dead_code)]
    fn debug_score(&self, digit_count: usize) {
        for line in self.score.iter() {
            let info = line
                .iter()
                .map(|cell| {
                    let cell_score = cell.get_smallest_num();
                    if cell_score == 0 {
                        " ".repeat(digit_count + 1)
                    } else {
                        format!("{:0>width$} ", cell_score, width = digit_count).to_string()
                    }
                })
                .collect::<String>();
            println!("{:?}", info);
        }
    }

    fn visit_cell(&mut self, pos: &Position, score: u64, from_dir: &Direction) {
        if self.get_cell(pos) != Cell::Start && self.get_cell(pos) != Cell::End {
            self.grid[pos.0][pos.1] = Cell::Visited;
        }
        if score != u64::MAX {
            self.score[pos.0][pos.1].set_direction(*from_dir, score);
        }
    }

    fn has_visited(&mut self, pos: &Position) -> bool {
        self.grid[pos.0][pos.1] == Cell::Visited
    }

    fn clear_map(&mut self) {
        let y_size = self.grid.len();
        let x_size = self.grid.first().expect("Failed to get first row").len();

        for y in 0..y_size {
            for x in 0..x_size {
                let pos = &Position(y, x);
                if self.get_cell(pos) == Cell::Visited {
                    self.grid[y][x] = Cell::Empty;
                }
            }
        }
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
        self.score[pos.0][pos.1].get_smallest_num()
    }

    fn get_score_data(&self, pos: &Position) -> MapData {
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

    fn get_neighbours_score(&self, pos: Position, from_dir: &Direction) -> Vec<MapData> {
        let neighbours = self.get_possible_directions(&pos, from_dir);
        let mut scores = vec![];
        for neigh in neighbours {
            scores.push(self.get_score_data(&pos.next_pos(&neigh)));
        }
        scores
    }

    fn get_best_path_count(
        &mut self,
        mut pos: Position,
        from_dir: &Direction,
        start_score: u64,
    ) -> u64 {
        if self.has_visited(&pos) {
            return 0;
        }

        let score = get_forward_score(self.get_score(&pos));
        let ccount = get_corner_count(self.get_score(&pos));

        if score == 0 {
            return 1; // reached our destination.
        }

        let mut count = 1; // 1, aka ourselfs
        self.visit_cell(&pos, u64::MAX, from_dir);

        println!("Score: {:?}", self.get_score(&pos));
        println!("Cell: {:?}", self.get_score_data(&pos));

        let mut path: Vec<(Position, Direction, u64)> = vec![];
        for dir in self
            .get_score_data(&pos)
            .get_directions_from_score(start_score)
        {
            path.push((pos, dir, start_score));
        }

        println!("Directions: {:?}", path);
        while let Some(info) = path.pop() {
            pos = info.0.next_pos(&info.1);

            println!("Processing: {:?}", info);
            if self.has_visited(&pos) {
                println!("Visited: {:?}", info);
                continue;
            }
            if get_forward_score(self.get_score(&pos)) == 0 {
                println!("Info is start: {:?}", info);
                continue;
            }

            self.visit_cell(&pos, u64::MAX, &Direction::North);
            count += 1;

            println!("Data: {:?}", self.get_score_data(&pos));
            println!(
                "Directions: {:?}",
                self.get_score_data(&pos).get_directions_from_score(info.2)
            );
            for dir in self.get_score_data(&pos).get_directions_from_score(info.2) {
                println!("Adding dir '{:?}' to list", dir);
                path.push((
                    pos,
                    dir,
                    self.get_score_data(&pos).get_direction_score(dir.inverse()),
                ));
            }
        }

        // let neighbours = self.get_neighbours_score(pos, from_dir);
        // if neighbours.len() == 0 {
        //     return 0;
        // }

        // println!("Possible: {:?}", neighbours);
        // println!("Score: {:?}", score);

        // let options = neighbours.iter().filter(|neigh| {
        //     get_forward_score(neigh.0) == score - 1 && get_corner_count(neigh.0) <= corner_count + 1
        // });

        // // println!(
        // //     "Options left: {:?}",
        // //     options.to_owned().collect::<Vec<&(u64, Direction)>>()
        // // );

        // for option in options {
        //     // println!("Option: {:?}", option);
        //     count += self.get_best_path_count(pos.next_pos(&option.1), &option.1.inverse(), ccount);
        // }

        count + 1
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
            map.visit_cell(&self.pos, score, &self.rot);
            return;
        }

        if map.has_visited(&self.pos) {
            if map.get_score_data(&self.pos).get_direction_score(self.rot) <= score {
                // if map.get_score(&self.pos) <= score {
                // println!("Already visited: {:?} at a lower cost.", self.pos);
                return;
            }
        }

        map.visit_cell(&self.pos, score, &self.rot);

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
    // map.debug_score(6);
    map.get_score(&goal)
}

#[aoc(day16, part2)]
fn part2(input: &(Map, Reindeer)) -> u64 {
    let mut map = input.0.to_owned();
    let mut reindeer = input.1.to_owned();

    reindeer.move_reindeer(&mut map, 0);
    map.clear_map(); // clear the map of visited cells ready to be visited by scoring algorithm.

    let goal = map.find_first_cell_of_type(&Cell::End);
    // map.debug_score(5);

    // println!("Goal: {:?}", goal);
    map.get_best_path_count(goal, &Direction::East, map.get_score(&goal))
    // 0
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
}
