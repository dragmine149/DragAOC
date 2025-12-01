use std::{
    fs::DirBuilder,
    ops::{AddAssign, SubAssign},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let info = line.trim().split_at(1);

            let dir = match info.0 {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction ({:?})", info.0),
            } as i64;

            info.1.parse::<i64>().unwrap() * dir
        })
        .collect::<Vec<i64>>()
}

#[aoc(day1, part1)]
fn part1(input: &[i64]) -> u64 {
    let mut pos = Pos::default();
    let mut zero_count = 0;
    input.iter().for_each(|dir| {
        if *dir > 0 {
            pos.turn_right(*dir as u64);
        }
        if *dir < 0 {
            pos.turn_left(dir.abs() as u64);
        }
        if pos.0 == 0 {
            zero_count += 1;
        }

        // println!("Pos: {:?}", pos);
        // pos += dir;

        // // loop around code.
        // if pos < 0 {
        //     pos = 100 + (pos % 100);
        // }
        // if pos >= 100 {
        //     pos = pos % 100;
        // }
        // // add to 0.
        // if pos == 0 {
        //     zero_count += 1;
        // }
    });
    zero_count
}

#[derive(Debug)]
pub struct Pos(i16);

impl Default for Pos {
    fn default() -> Self {
        Self(50)
    }
}

impl Pos {
    fn wrap(&mut self) -> u64 {
        if self.0 < 0 {
            self.0 = 100 + self.0;
        }
        if self.0 > 100 {
            self.0 = self.0 - 100;
        }
        if self.0 == 100 {
            self.0 = 0;
        }
        // if self.0 == 0 {
        //     return 1;
        // }
        0
    }

    fn turn_left(&mut self, left: u64) -> u64 {
        let mut d = left / 100;
        let m = (left % 100) as i16;
        if self.0 - m < 0 && self.0 - m != 0 {
            d += 1;
        }
        self.0 -= m;
        self.wrap();

        d
    }

    fn turn_right(&mut self, right: u64) -> u64 {
        let mut d = right / 100;
        let m = (right % 100) as i16;
        if self.0 + m >= 100 && self.0 + m != 100 {
            d += 1;
        }

        // println!("{:?} {:?}", self.0, m);
        self.0 += m;
        self.wrap();
        d
    }
}

#[aoc(day1, part2)]
fn part2(input: &[i64]) -> u64 {
    let mut pos = Pos::default();
    let mut zero_count = 0;
    input.iter().for_each(|dir| {
        println!("{:?} {:?} {:?}", pos, zero_count, dir);
        if *dir > 0 {
            zero_count += pos.turn_right(*dir as u64);
        }
        if *dir < 0 {
            zero_count += pos.turn_left(dir.abs() as u64);
        }
        if pos.0 == 0 {
            zero_count += 1;
        }
    });
    println!("{:?} {:?} {:?}", pos, zero_count, "e");
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            )),
            3
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            )),
            6
        );
    }

    #[test]
    fn divmod_test() {
        assert_eq!(-18 / 100, 0);
        assert_eq!(-18 % 100, -18);
        assert_eq!(18 / 100, 0);
        assert_eq!(18 % 100, 18);
    }

    #[test]
    fn wrap_test() {
        let a = 4_u8;
        let b = 8_u8;
        assert_eq!(a.wrapping_sub(b), 252);
        assert_eq!(a, 4_u8);
        assert_eq!(b, 8_u8);
    }
}
