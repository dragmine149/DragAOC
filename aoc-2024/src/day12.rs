use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

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

    fn get_area(&self) -> u8 {
        1
    }
}

impl fmt::Debug for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Plot {{ id: {:?}, pos: {:?}, a_f: {:?} }}",
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
        _ => 0,
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
        _ => "?",
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Vec<Plot>> {
    input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.chars()
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

// #[aoc(day12, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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

    //     const EXAMPLE4: &str = "EEEEE
    // EXXXX
    // EEEEE
    // EXXXX
    // EEEEE";

    //     const EXAMPLE5: &str = "AAAAAA
    // AAABBA
    // AAABBA
    // ABBAAA
    // ABBAAA
    // AAAAAA";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse(EXAMPLE1)), 140);
        assert_eq!(part1(&parse(EXAMPLE2)), 772);
        assert_eq!(part1(&parse(EXAMPLE3)), 1930);
    }

    // #[test]
    // fn part2_example1() {
    //     assert_eq!(part2(&parse(EXAMPLE1)), 80);
    // }

    // #[test]
    // fn part2_example2() {
    //     assert_eq!(part2(&parse(EXAMPLE2)), 436);
    // }

    // #[test]
    // fn part2_example3() {
    //     assert_eq!(part2(&parse(EXAMPLE3)), 1206);
    // }

    // #[test]
    // fn part2_example4() {
    //     assert_eq!(part2(&parse(EXAMPLE4)), 236);
    // }

    // #[test]
    // fn part2_example5() {
    //     assert_eq!(part2(&parse(EXAMPLE5)), 368);
    // }
}
