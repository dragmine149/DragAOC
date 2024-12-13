use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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
    fn get_valid_directions(&self, grid: &[Vec<Plot>]) -> Vec<Direction> {
        let mut valid: Vec<Direction> = vec![];
        if self.0 >= 1 {
            valid.push(Direction::North);
        }
        if self.1 >= 1 {
            valid.push(Direction::West);
        }
        if self.0 < grid.len() - 1 {
            valid.push(Direction::South);
        }
        if self.1 < grid.first().expect("Failed to get first line").len() - 1 {
            valid.push(Direction::East);
        }
        valid
    }
    fn get_invalid_directions(&self, grid: &[Vec<Plot>]) -> Vec<Direction> {
        let mut invalid: Vec<Direction> = vec![];
        if self.0 < 1 {
            invalid.push(Direction::North);
        }
        if self.1 < 1 {
            invalid.push(Direction::West);
        }
        if self.0 >= grid.len() - 1 {
            invalid.push(Direction::South);
        }
        if self.1 >= grid.first().expect("Failed to get first line").len() - 1 {
            invalid.push(Direction::East);
        }
        invalid
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Plot {
    plot_id: u8,
    plot_pos: Position,
    accounted_for: bool,
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

    fn get_sides(&self, grid: &[Vec<Plot>]) -> Vec<Direction> {
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

fn plot_potential_in_segment(seg_list: &[Plot], seg: Plot) -> bool {
    let mut pass: bool = true;
    for elm in seg_list.iter() {
        if elm.plot_pos.0 != seg.plot_pos.0 && elm.plot_pos.1 != seg.plot_pos.1 {
            pass = false;
            break;
        }
    }
    pass
}

fn plot_in_segmenet(grid: &[Vec<Plot>], seg_list: &[Plot], seg: Plot) -> bool {
    let r = seg_list.iter().any(|x| {
        // println!(
        //     "Seg Pos: {:?}. seg list pos: {:?}",
        //     seg.plot_pos, x.plot_pos
        // );
        let mut pass = false;
        if x.plot_pos.0 >= 1 {
            pass = if x.plot_pos.0 - 1 == seg.plot_pos.0 {
                true
            } else {
                pass
            };
        }
        if x.plot_pos.1 >= 1 {
            pass = if x.plot_pos.1 - 1 == seg.plot_pos.1 {
                true
            } else {
                pass
            };
        }
        if x.plot_pos.0 < grid.len() {
            pass = if x.plot_pos.0 + 1 == seg.plot_pos.0 {
                true
            } else {
                pass
            };
        }
        if x.plot_pos.1 < grid.first().expect("Failed to get first line").len() {
            pass = if x.plot_pos.1 + 1 == seg.plot_pos.1 {
                true
            } else {
                pass
            };
        }

        pass
    });
    // println!("r: {:?}", r);
    r
}

fn segment_has_plot(seg_list: &[Plot], seg: Plot) -> bool {
    match seg_list.iter().find(|x| x.compare_pos(&seg)) {
        Some(_) => true,
        None => false,
    }
}

fn merge_segment(grid: &[Vec<Plot>], segment: Plot, segments: &mut Vec<(Vec<Plot>, Direction)>) {
    let sides = segment.get_sides(grid);
    if sides.is_empty() {
        return;
    }

    for wall in sides {
        // println!("-*-");
        let mut insert = false;
        let mut contains = false;
        for seg in segments.iter_mut() {
            if seg.1 != wall {
                continue;
            }

            if segment_has_plot(&seg.0, segment) {
                contains = true;
                continue;
            }

            if !plot_potential_in_segment(&seg.0, segment) {
                continue;
            }

            if !plot_in_segmenet(grid, &seg.0, segment) {
                continue;
            }

            // println!(
            //     "Inserted neigh {:?} with wall {:?} into {:?}",
            //     neigh, wall, seg,
            // );
            insert = true;
            seg.0.push(segment);
            seg.0.sort();
            break;
        }

        if !insert && !contains {
            // println!("Making new segments out of {:?} (Made: {:?})", neigh, wall);
            segments.push((vec![segment], wall));
        }
    }
}

fn get_land_sides(grid: &mut [Vec<Plot>], plot: Plot) -> (Vec<(Vec<Plot>, Direction)>, u64) {
    let mut types: Vec<Plot> = vec![plot];

    let mut searched: Vec<Plot> = vec![];
    let mut segments: Vec<(Vec<Plot>, Direction)> = vec![];

    while let Some(pt) = types.pop() {
        let neighs = pt.get_neighbours_of_same_type(grid);
        for neigh in neighs {
            // println!("Processing neigh: {:?}", neigh);1
            match searched.iter().find(|x| x.compare_pos(&neigh)) {
                Some(_) => {}
                None => {
                    types.push(neigh);
                }
            }

            merge_segment(grid, neigh, &mut segments);
        }
        match searched.iter().find(|x| x.compare_pos(&pt)) {
            Some(_) => {}
            None => {
                searched.push(pt);
                grid[pt.plot_pos.0][pt.plot_pos.1].accounted_for = true;
            }
        }
    }

    // let mut final_segments: Vec<(Vec<Plot>, Direction)> = vec![];
    // for segment in segments.iter() {
    //     for seg in segment.0.iter() {
    //         merge_segment(grid, *seg, &mut final_segments);
    //     }
    // }
    // segments = final_segments;

    if segments.is_empty() {
        segments.push((vec![plot], Direction::North));
        segments.push((vec![plot], Direction::East));
        segments.push((vec![plot], Direction::South));
        segments.push((vec![plot], Direction::West));
    }

    (segments, searched.len() as u64)
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
    let mut price_total: u64 = 0;

    let mut grid = input.to_owned();
    input.iter().for_each(|line| {
        line.iter().for_each(|plot| {
            if grid[plot.plot_pos.0][plot.plot_pos.1].accounted_for {
                return;
            }
            println!("-------------------------------Searching for {:?}-------{:?}-----------------------------------------------", convert_to_str(&plot.plot_id), plot);
            let sides = get_land_sides(&mut grid, *plot);
            println!("Sides: {:#?}", sides.0);
            println!("Side Count: {:?}", sides.0.len());
            println!("Price: {:?}", sides.0.len() as u64 * sides.1);
            price_total += sides.0.len() as u64 * sides.1;
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
