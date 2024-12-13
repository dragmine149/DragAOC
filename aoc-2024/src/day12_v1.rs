use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
enum Directions {
    North,
    East,
    South,
    West,
}

impl Directions {
    fn rotate_clock(&self) -> Directions {
        match self {
            Directions::North => Directions::East,
            Directions::East => Directions::South,
            Directions::South => Directions::West,
            Directions::West => Directions::North,
        }
    }
    fn rotate_count_clock(&self) -> Directions {
        match self {
            Directions::North => Directions::West,
            Directions::West => Directions::South,
            Directions::South => Directions::East,
            Directions::East => Directions::North,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Position(usize, usize);

impl Position {
    fn get_valid_positions(&self, grid: &[Vec<Plot>]) -> Vec<Position> {
        let mut valid: Vec<Position> = vec![];
        if self.0 >= 1 {
            valid.push(Position(self.0 - 1, self.1));
        }
        if self.1 >= 1 {
            valid.push(Position(self.0, self.1 - 1));
        }
        if self.0 < grid.len() - 1 {
            valid.push(Position(self.0 + 1, self.1));
        }
        if self.1 < grid.first().expect("Failed to get first line").len() - 1 {
            valid.push(Position(self.0, self.1 + 1));
        }
        valid
    }
    fn get_valid_directions(&self, grid: &[Vec<Plot>]) -> Vec<Directions> {
        let mut valid: Vec<Directions> = vec![];
        if self.0 >= 1 {
            valid.push(Directions::North);
        }
        if self.1 >= 1 {
            valid.push(Directions::West);
        }
        if self.0 < grid.len() - 1 {
            valid.push(Directions::South);
        }
        if self.1 < grid.first().expect("Failed to get first line").len() - 1 {
            valid.push(Directions::East);
        }
        valid
    }
    fn get_invalid_directions(&self, grid: &[Vec<Plot>]) -> Vec<Directions> {
        let mut invalid: Vec<Directions> = vec![];
        if self.0 < 1 {
            invalid.push(Directions::North);
        }
        if self.1 < 1 {
            invalid.push(Directions::West);
        }
        if self.0 >= grid.len() - 1 {
            invalid.push(Directions::South);
        }
        if self.1 >= grid.first().expect("Failed to get first line").len() - 1 {
            invalid.push(Directions::East);
        }
        invalid
    }
    fn next_pos_from_direction(&self, grid: &[Vec<Plot>], direction: Directions) -> Position {
        match direction {
            Directions::North => {
                if self.0 >= 1 {
                    return Position(self.0 - 1, self.1);
                }
                panic!("North: Direction not safe!");
            }
            Directions::East => {
                if self.1 < grid.first().expect("Failed to get first line").len() - 1 {
                    return Position(self.0, self.1 + 1);
                }
                panic!("East: Direction not safe!");
            }
            Directions::South => {
                if self.0 < grid.len() - 1 {
                    return Position(self.0 + 1, self.1);
                }
                panic!("South: Direction not safe!");
            }
            Directions::West => {
                if self.1 >= 1 {
                    return Position(self.0, self.1 - 1);
                }
                panic!("West: Direction not safe!");
            }
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, Copy)]
struct Plot {
    plot_id: u8,
    plot_pos: Position,
    accounted_for: bool,
    north: bool,
    east: bool,
    west: bool,
    south: bool,
}

impl Default for Plot {
    fn default() -> Plot {
        Plot {
            plot_id: 90,
            plot_pos: Position(0, 0),
            accounted_for: true,
            north: true,
            east: true,
            west: true,
            south: true,
        }
    }
}
impl Plot {
    fn compare_plot(&self, other: &Plot) -> bool {
        self.plot_id == other.plot_id
    }
    fn compare_pos(&self, other: &Plot) -> bool {
        self.plot_pos == other.plot_pos
    }

    fn get_neighbours_of_same_type(&self, grid: &[Vec<Plot>]) -> Vec<Plot> {
        let directions = self.plot_pos.get_valid_positions(grid);
        let mut neighbours = vec![];
        for direction in directions {
            let plot = grid[direction.0][direction.1];
            if plot.compare_plot(self) {
                neighbours.push(plot);
            }
        }
        neighbours
    }

    fn get_border_count(&self, grid: &[Vec<Plot>]) -> u8 {
        let directions = self.plot_pos.get_valid_positions(grid).len() as u8;
        let neighbours = self.get_neighbours_of_same_type(grid).len() as u8;

        match directions {
            4 => 4_u8 - neighbours,
            3 => (3_u8 - neighbours) + 1_u8,
            2 => (2_u8 - neighbours) + 2_u8,
            1 => (1_u8 - neighbours) + 3_u8,
            0 => 4_u8,
            _ => panic!("Unknown direction count!"),
        }
    }

    fn get_sides(&self, grid: &[Vec<Plot>]) -> Vec<Directions> {
        let directions = self.plot_pos.get_valid_positions(grid);
        let valid_direct = self.plot_pos.get_valid_directions(grid);
        let mut invalid_direct = self.plot_pos.get_invalid_directions(grid);

        let mut sides = vec![];
        sides.append(&mut invalid_direct);
        for (direction_index, direction) in directions.iter().enumerate() {
            let plot = grid[direction.0][direction.1];
            if !plot.compare_plot(self) {
                sides.push(valid_direct[direction_index]);
            }
        }
        sides.sort();
        sides
    }

    fn get_area(&self) -> u8 {
        1
    }

    fn has_checked_wall(&self, wall: &Directions) -> bool {
        match wall {
            Directions::North => self.north,
            Directions::East => self.east,
            Directions::South => self.south,
            Directions::West => self.west,
        }
    }

    fn check_wall(&mut self, wall: &Directions) {
        match wall {
            Directions::North => self.north = true,
            Directions::East => self.east = true,
            Directions::South => self.south = true,
            Directions::West => self.west = true,
        }
    }

    fn has_wall(&self, grid: &[Vec<Plot>], wall: &Directions) -> bool {
        match self.get_sides(grid).iter().find(|x| *x == wall) {
            Some(_) => true,
            None => false,
        }
    }
}

impl fmt::Debug for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Plot {{ id: {:?}, pos: {:?}, af: {:?} }}",
            convert_to_str(&self.plot_id),
            self.plot_pos,
            self.accounted_for
        )
    }
}

// Probably a better way of doing this but well whatever
fn convert_to_num(input: &char) -> u8 {
    match *input {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => {
            println!("WARNING: {:?}", input);
            0
        }
    }
}
fn convert_to_str(input: &u8) -> &str {
    match input {
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        9 => "i",
        10 => "j",
        11 => "k",
        12 => "l",
        13 => "m",
        14 => "n",
        15 => "o",
        16 => "p",
        17 => "q",
        18 => "r",
        19 => "s",
        20 => "t",
        21 => "u",
        22 => "v",
        23 => "w",
        24 => "x",
        25 => "y",
        26 => "z",
        0 => "!",
        _ => "?",
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Vec<Plot>> {
    input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(char_index, char)| Plot {
                    plot_id: convert_to_num(&char.to_ascii_lowercase()),
                    plot_pos: Position(line_index, char_index),
                    accounted_for: false,
                    ..Default::default()
                })
                .collect::<Vec<Plot>>()
        })
        .collect::<Vec<Vec<Plot>>>()
}

fn get_all_of_type(grid: &[Vec<Plot>], plot: Plot) -> Vec<Plot> {
    let mut types: Vec<Plot> = vec![plot];
    let mut neighbours: Vec<Plot> = vec![];
    while let Some(pt) = types.pop() {
        let neighs = pt.get_neighbours_of_same_type(grid);
        for neigh in neighs {
            match neighbours.iter().find(|x| x.compare_pos(&neigh)) {
                Some(_) => {}
                None => {
                    types.push(neigh);
                }
            }
        }
        match neighbours.iter().find(|x| x.compare_pos(&pt)) {
            Some(_) => {}
            None => {
                neighbours.push(pt);
            }
        }
        // neighbours.push(pt);
    }

    neighbours
}

fn get_plot_price(grid: &mut [Vec<Plot>], plots: &mut [Plot]) -> u64 {
    let mut area = 0;
    let mut perimeter = 0;
    // println!("Plots: {:?}", plots);
    for plot in plots.iter_mut() {
        area += plot.get_area() as u64;
        perimeter += plot.get_border_count(grid) as u64;

        grid[plot.plot_pos.0][plot.plot_pos.1].accounted_for = true;
    }
    // println!("Area: {:?}. Perimeter: {:?}", area, perimeter);
    area * perimeter
}

fn is_plot_corner(grid: &[Vec<Plot>], wall: Directions, plot: Plot) -> bool {
    println!("Checking for {:?} at {:?}", wall.rotate_clock(), plot);
    // println!("{:?}", plot.get_sides(grid));
    plot.has_wall(grid, &wall.rotate_clock())
}

fn rotate_corner(
    grid: &mut [Vec<Plot>],
    mut wall: Directions,
    plot: Plot,
) -> (u8, Plot, Directions) {
    let mut wall_count = 1;

    println!("{:?}  {:?}    {:?}", plot, plot.get_sides(grid), wall);
    if !plot.has_wall(grid, &wall) {
        println!("No walls on left! Rotating couter clockwise and continuing");
        let nwall = wall.rotate_count_clock();
        return (1, move_wall(grid, nwall, plot), nwall);
    }

    if plot.has_checked_wall(&wall) {
        wall_count -= 1;
    }

    grid[plot.plot_pos.0][plot.plot_pos.1].check_wall(&wall);

    wall = wall.rotate_clock();
    if is_plot_corner(grid, wall, plot) {
        println!("Rotating corner corner!");
        wall_count += 1;
        if plot.has_checked_wall(&wall) {
            wall_count -= 1;
        }
        grid[plot.plot_pos.0][plot.plot_pos.1].check_wall(&wall);
        wall = wall.rotate_clock();
    }
    (wall_count, move_wall(grid, wall, plot), wall)
}

fn move_wall(grid: &[Vec<Plot>], wall: Directions, plot: Plot) -> Plot {
    let next_plot = plot
        .plot_pos
        .next_pos_from_direction(grid, wall.rotate_clock());
    grid[next_plot.0][next_plot.1]
}

fn move_and_rotate(grid: &mut [Vec<Plot>], wall: Directions, plot: Plot) -> (u8, Plot, Directions) {
    if plot.get_sides(grid).is_empty() {
        println!("No walls! Rotating couter clockwise and continuing");
        let nwall = wall.rotate_count_clock();
        return (1, move_wall(grid, nwall, plot), nwall);
    }

    if is_plot_corner(grid, wall, plot) {
        println!("Rotating corner!");
        return rotate_corner(grid, wall, plot);
    }

    if !plot.has_wall(grid, &wall) {
        println!("No walls on left! Rotating couter clockwise and continuing");
        let nwall = wall.rotate_count_clock();
        return (1, move_wall(grid, nwall, plot), nwall);
    }

    (0, move_wall(grid, wall, plot), wall)
}

fn get_plot_side_price(grid: &mut [Vec<Plot>], plots: &mut [Plot]) -> u64 {
    if plots.len() == 1 {
        // Assumption: There is one plot hence there is no same value, hence we have walls everywhere, hence the answer will always be 4
        return 4;
    }
    if plots.len() == 2 {
        // Assumption: As plots can't be diagonal. It must be in a I-ish shape. Hence we have walls on 4 sides as no corners. hence the answer will always be 8
        return 8;
    }

    let mut area = 0;
    let mut side_count = 0;

    let mut side_map: Vec<Vec<Plot>> =
        vec![
            vec![
                Plot {
                    ..Default::default()
                };
                grid.first().expect("Failed to get first line").len()
            ];
            grid.len()
        ];
    // println!("Plots: {:?}", plots);
    for plot in plots.iter_mut() {
        grid[plot.plot_pos.0][plot.plot_pos.1].accounted_for = true;

        area += plot.get_area() as u64;
        let plot_sides = plot.get_sides(grid);
        // println!("Plot: {:?}, sides: {:?}", plot, plot_sides);
        if !plot_sides.is_empty() {
            plot.north = false;
            plot.east = false;
            plot.south = false;
            plot.west = false;
        }
        side_map[plot.plot_pos.0][plot.plot_pos.1] = *plot;
    }
    // println!("{:?}", side_map);

    let mut result = 0;

    side_map.to_owned().iter().for_each(|tile_row| {
        for tile in tile_row.iter() {
            if !tile.has_wall(grid, &Directions::North) {
                continue;
            }

            let start_pos = tile;
            let mut current_wall = Directions::North;
            let mut current_pos = *tile;
            // Assumption: Plots[0].plot_pos will always have walls [North, West]
            // DO one outside the loop to offset us
            println!(
                "Investigating plot: {:?} Wall: {:?}",
                current_pos, current_wall
            );
            let (wall_count, new_pos, new_wall) =
                move_and_rotate(&mut side_map, current_wall, current_pos);
            side_count += wall_count as u64;
            current_wall = new_wall;
            current_pos = new_pos;

            // let count = 100;
            // let mut cc = 0;

            // while !start_pos.compare_pos(&current_pos) && cc < count {
            while !start_pos.compare_pos(&current_pos) {
                println!(
                    "Investigating plot: {:?} Wall: {:?}",
                    current_pos, current_wall
                );
                println!("Sides: {:?}", side_count);

                let (wall_count, new_pos, new_wall) =
                    move_and_rotate(&mut side_map, current_wall, current_pos);
                side_count += wall_count as u64;
                current_wall = new_wall;
                current_pos = new_pos;
                // cc += 1;
            }

            // Do another one just to finish the loop
            let (wall_count, _, _) = move_and_rotate(&mut side_map, current_wall, current_pos);
            side_count += wall_count as u64;
            // current_wall = new_wall;
            // current_pos = new_pos;
        }

        println!("Sides: {:?}", side_count);
        result += area * side_count;
    });

    result
}

#[aoc(day12, part1)]
fn part1(input: &[Vec<Plot>]) -> u64 {
    let mut price_total = 0;

    let mut grid = input.to_owned();
    input.iter().enumerate().for_each(|(line_index, line)| {
        line.iter().enumerate().for_each(|(plot_index, plot)| {
            if grid[line_index][plot_index].accounted_for {
                return;
            }

            let mut plots = get_all_of_type(&grid, *plot);
            let price = get_plot_price(&mut grid, &mut plots);
            // println!("Price of {:?}: {:?}", convert_to_str(&plot.plot_id), price);
            price_total += price;
        })
    });

    price_total
}

#[aoc(day12, part2)]
fn part2(input: &[Vec<Plot>]) -> u64 {
    let mut price_total = 0;

    let mut grid = input.to_owned();
    input.iter().enumerate().for_each(|(line_index, line)| {
        line.iter().enumerate().for_each(|(plot_index, plot)| {
            if grid[line_index][plot_index].accounted_for {
                return;
            }
            println!("----------------------------------------------------------------------------------");

            let mut plots = get_all_of_type(&grid, *plot);
            let price = get_plot_side_price(&mut grid, &mut plots);
            println!("Price of {:?}: {:?}", convert_to_str(&plot.plot_id), price);
            price_total += price;
        })
    });

    price_total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE4: &str = "EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE";

    const EXAMPLE5: &str = "AAAAAA
    AAABBA
    AAABBA
    ABBAAA
    ABBAAA
    AAAAAA";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse(EXAMPLE1)), 140);
        assert_eq!(part1(&parse(EXAMPLE2)), 772);
        assert_eq!(part1(&parse(EXAMPLE3)), 1930);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE1)), 80);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(EXAMPLE2)), 436);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&parse(EXAMPLE3)), 1206);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2(&parse(EXAMPLE4)), 236);
    }

    #[test]
    fn part2_example5() {
        assert_eq!(part2(&parse(EXAMPLE5)), 368);
    }
}
