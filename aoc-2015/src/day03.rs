use crate::utils::Direction;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

pub fn next_pos(pos: (isize, isize), direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
        _ => panic!("Invalid direction!"),
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Direction> {
    input
        .trim()
        .chars()
        .map(|char| match char {
            '>' => Direction::East,
            'v' => Direction::South,
            '^' => Direction::North,
            '<' => Direction::West,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<Direction>>()
}

#[aoc(day3, part1)]
fn part1(input: &[Direction]) -> usize {
    let mut pos: (isize, isize) = (0, 0);
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    map.insert(pos, 1);

    for dir in input.iter() {
        pos = next_pos(pos, dir);
        let v = map.get(&pos);
        match v {
            Some(count) => {
                let _ = map.insert(pos, count + 1);
            }
            None => {
                let _ = map.insert(pos, 1);
            }
        }
    }

    // println!("{:?}", map);
    map.keys().count()
}

#[aoc(day3, part2)]
fn part2(input: &[Direction]) -> usize {
    let mut pos: (isize, isize) = (0, 0);
    let mut pos2: (isize, isize) = (0, 0);
    let mut map: HashSet<(isize, isize)> = HashSet::new();
    let mut map2: HashSet<(isize, isize)> = HashSet::new();
    let mut turn = true;
    map.insert(pos);

    for dir in input.iter() {
        let v: bool;

        if turn {
            pos = next_pos(pos, dir);
            v = map.contains(&pos);
        } else {
            pos2 = next_pos(pos2, dir);
            v = map2.contains(&pos2);
        }

        match v {
            true => {}
            false => {
                if turn {
                    let _ = map.insert(pos);
                } else {
                    let _ = map2.insert(pos2);
                }
            }
        }
        turn = !turn;
    }

    // println!("{:?}", map);
    // println!("{:?}", map2);
    // println!("{:?}", map.union(&map2));

    map.union(&map2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse(">")), 2);
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse("^>v<")), 4);
    }
    #[test]
    fn part1_example_3() {
        assert_eq!(part1(&parse("^v^v^v^v^v")), 2);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse("^v")), 3);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse("^>v<")), 3);
    }
    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&parse("^v^v^v^v^v")), 11);
    }
}
