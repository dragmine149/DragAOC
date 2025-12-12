use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Present([bool; 9]);

impl Present {
    fn area_sum(&self) -> u8 {
        self.0.iter().filter(|b| **b).count() as u8
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> (Vec<Present>, Vec<(u8, u8, Vec<u8>)>) {
    let mut iter = input.lines();
    let mut presents = vec![];
    for _ in 0..6 {
        iter.next();
        let present = iter
            .by_ref()
            .take(3)
            .map(|s| {
                s.chars().map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid character {:?}", c),
                })
            })
            .flatten()
            .collect_array()
            .expect("An array of 9 items!");
        presents.push(Present(present));
        iter.next();
    }
    let grid = iter
        .map(|line| {
            let mut split = line.split(':');
            let mut split2 = split.next().unwrap().split('x');
            let width = split2.next().unwrap().parse::<u8>().unwrap();
            let height = split2.next().unwrap().parse::<u8>().unwrap();
            let count = split
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<u8>()
                        .unwrap_or_else(|_| panic!("Failed to parse {:?} to u8", s))
                })
                .collect_vec();
            (width, height, count)
        })
        .collect_vec();
    (presents, grid)
}

#[aoc(day12, part1)]
fn part1(input: &(Vec<Present>, Vec<(u8, u8, Vec<u8>)>)) -> usize {
    // println!("{:?}", input);
    input
        .1
        .iter()
        .map(|i| {
            let area = i.0 as u16 * i.1 as u16;
            let min_required =
                i.2.iter()
                    .enumerate()
                    .map(|(idx, want)| input.0[idx].area_sum() as u16 * *want as u16)
                    .sum::<u16>();
            if min_required > area {
                return 0;
            }
            if min_required as f64 * 1.5 > area as f64 {
                return 2;
            }

            println!(
                "{:?} (min: {:?}, area: {:?})",
                i,
                min_required as f64 * 1.5,
                area
            );
            1
        })
        .filter(|i| *i == 1)
        .count()
}

// #[aoc(day12, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 2);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
