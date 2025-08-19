use crate::utils::{Grid, Position};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day18)]
fn parse(input: &str) -> Grid<u8> {
    Grid::from_str(input, 0, |c| match c {
        '#' => 1,
        '.' => 0,
        _ => panic!("Invalid character: '{:?}'", c),
    })
}

impl Grid<u8> {
    fn flip_check(&self, position: &Position) -> bool {
        // println!("{:?}", position);
        let neighbours = position.get_neighbours(&self.get_size(), false);
        // println!("{:?}", position.get_valid_directions(&self.get_size()));
        // println!("{:?}", neighbours);
        let neighbour_count = neighbours
            .iter()
            .map(|p| self.get_unmut_cell(p))
            .filter(|v| *v == 1)
            .count();
        let state = self.get_unmut_cell(position);
        let new = match state {
            1 => neighbour_count == 2 || neighbour_count == 3,
            0 => neighbour_count == 3,
            _ => panic!("Invalid numbers"),
        } as u8;
        state != new
    }

    fn flip(&mut self, position: &Position) {
        let previous = self.get_unmut_cell(position);
        let next = if previous == 0 {
            1
        } else if previous == 1 {
            0
        } else {
            panic!("Invalid value!")
        };
        self.set_cell(position, next);
    }
}

fn process_lights(lights: &mut Grid<u8>, steps: u64, p2: bool) -> u64 {
    for _step in 0..steps {
        let positions = lights
            .iter_positions()
            .into_iter()
            .filter(|cell| {
                if p2 {
                    !cell.is_corner(&lights.get_size())
                } else {
                    true
                }
            })
            .filter(|cell| lights.flip_check(cell))
            .collect_vec();
        positions.iter().for_each(|pos| lights.flip(pos));

        // println!("step: {:?}", _step + 1);
        // println!("{:?}", lights);
    }

    lights
        .iter_unmut_cells()
        .into_iter()
        .filter(|v| *v == 1)
        .count() as u64
}

#[aoc(day18, part1)]
fn part1(input: &Grid<u8>) -> u64 {
    let mut g = input.to_owned();
    process_lights(&mut g, 100, false)
}

#[aoc(day18, part2)]
fn part2(input: &Grid<u8>) -> u64 {
    let mut g = input.to_owned();
    g.set_corners(1);
    process_lights(&mut g, 100, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..
";

    #[test]
    fn part1_example() {
        let input = &parse(EXAMPLE_1);
        let mut g = input.to_owned();
        assert_eq!(process_lights(&mut g, 4, false), 4);
    }

    #[test]
    fn part2_example() {
        let input = &parse(EXAMPLE_1);
        let mut g = input.to_owned();
        g.set_corners(1);
        assert_eq!(process_lights(&mut g, 5, true), 17);
    }
}
