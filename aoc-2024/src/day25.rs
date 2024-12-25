use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fmt::Debug;

#[derive(Debug)]
enum KeyLockType {
    Key,
    Lock,
}

#[derive(Clone, Copy)]
struct KeyLock {
    column_1: [u8; 7],
    column_2: [u8; 7],
    column_3: [u8; 7],
    column_4: [u8; 7],
    column_5: [u8; 7],
}

impl Debug for KeyLock {
    //  Custom debug just so we can see it easier.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c1 = self.column_1.map(|v| if v == 1 { '#' } else { '.' });
        let c2 = self.column_2.map(|v| if v == 1 { '#' } else { '.' });
        let c3 = self.column_3.map(|v| if v == 1 { '#' } else { '.' });
        let c4 = self.column_4.map(|v| if v == 1 { '#' } else { '.' });
        let c5 = self.column_5.map(|v| if v == 1 { '#' } else { '.' });

        writeln!(
            f,
            "KeyLock ({:?}/{:?}): {{",
            self.get_type(),
            self.get_pattern()
        )?;
        writeln!(f, "\t{}{}{}{}{}", c1[0], c2[0], c3[0], c4[0], c5[0])?;
        writeln!(f, "\t{}{}{}{}{}", c1[1], c2[1], c3[1], c4[1], c5[1])?;
        writeln!(f, "\t{}{}{}{}{}", c1[2], c2[2], c3[2], c4[2], c5[2])?;
        writeln!(f, "\t{}{}{}{}{}", c1[3], c2[3], c3[3], c4[3], c5[3])?;
        writeln!(f, "\t{}{}{}{}{}", c1[4], c2[4], c3[4], c4[4], c5[4])?;
        writeln!(f, "\t{}{}{}{}{}", c1[5], c2[5], c3[5], c4[5], c5[5])?;
        writeln!(f, "\t{}{}{}{}{}", c1[6], c2[6], c3[6], c4[6], c5[6])?;
        writeln!(f, "}}")
    }
}

impl KeyLock {
    // create a blank slate
    fn new() -> Self {
        Self {
            column_1: [u8::MAX; 7],
            column_2: [u8::MAX; 7],
            column_3: [u8::MAX; 7],
            column_4: [u8::MAX; 7],
            column_5: [u8::MAX; 7],
        }
    }

    fn insert_from_string(&mut self, string: &str) {
        // as we do stuff in rows and require it columns, easier to do a row by row isnert
        let row = self.column_1.iter().position(|p| *p == u8::MAX);
        match row {
            Some(r) => {
                // for all chacters
                // although probably a better way of doing this, maybe a 2d array?
                for i in 0..5 {
                    match i {
                        0 => {
                            self.column_1[r] =
                                (string.chars().nth(i).expect("Can't find char at pos") == '#')
                                    as u8
                        }
                        1 => {
                            self.column_2[r] =
                                (string.chars().nth(i).expect("Can't find char at pos") == '#')
                                    as u8
                        }
                        2 => {
                            self.column_3[r] =
                                (string.chars().nth(i).expect("Can't find char at pos") == '#')
                                    as u8
                        }
                        3 => {
                            self.column_4[r] =
                                (string.chars().nth(i).expect("Can't find char at pos") == '#')
                                    as u8
                        }
                        4 => {
                            self.column_5[r] =
                                (string.chars().nth(i).expect("Can't find char at pos") == '#')
                                    as u8
                        }
                        _ => panic!("Invalid number!"),
                    }
                }
            }
            None => panic!("KeyLock has been filled, so why are we trying to write to it?"),
        }
    }

    // get the type based off the first row (because the last row will be the opposite)
    fn get_type(&self) -> KeyLockType {
        if self.column_1[0] == 1
            && self.column_2[0] == 1
            && self.column_3[0] == 1
            && self.column_4[0] == 1
            && self.column_5[0] == 1
        {
            KeyLockType::Lock
        } else {
            KeyLockType::Key
        }
    }

    // get the column pattern
    fn get_pattern(&self) -> [u8; 5] {
        [
            self.column_1[0]
                + self.column_1[1]
                + self.column_1[2]
                + self.column_1[3]
                + self.column_1[4]
                + self.column_1[5]
                + self.column_1[6]
                - 1,
            self.column_2[0]
                + self.column_2[1]
                + self.column_2[2]
                + self.column_2[3]
                + self.column_2[4]
                + self.column_2[5]
                + self.column_2[6]
                - 1,
            self.column_3[0]
                + self.column_3[1]
                + self.column_3[2]
                + self.column_3[3]
                + self.column_3[4]
                + self.column_3[5]
                + self.column_3[6]
                - 1,
            self.column_4[0]
                + self.column_4[1]
                + self.column_4[2]
                + self.column_4[3]
                + self.column_4[4]
                + self.column_4[5]
                + self.column_4[6]
                - 1,
            self.column_5[0]
                + self.column_5[1]
                + self.column_5[2]
                + self.column_5[3]
                + self.column_5[4]
                + self.column_5[5]
                + self.column_5[6]
                - 1,
        ]
    }

    // test to see if any column when combined with any other column is above 6, aka something must overlap.
    fn test_with_opposite(&self, other: &Self) -> bool {
        let pattern_a = self.get_pattern();
        let pattern_b = other.get_pattern();
        !(pattern_a[0] + pattern_b[0] >= 6
            || pattern_a[1] + pattern_b[1] >= 6
            || pattern_a[2] + pattern_b[2] >= 6
            || pattern_a[3] + pattern_b[3] >= 6
            || pattern_a[4] + pattern_b[4] >= 6)
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> (Vec<KeyLock>, Vec<KeyLock>) {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut current = KeyLock::new();

    input.lines().for_each(|line| {
        // println!("Line: {:?}", line);
        if line.trim().is_empty() {
            // insert into seperate vectors
            match current.get_type() {
                KeyLockType::Key => keys.push(current),
                KeyLockType::Lock => locks.push(current),
            }

            // reser ready for next one
            current = KeyLock::new();
            return;
        }

        current.insert_from_string(line.trim());
    });

    // add the left over one
    match current.get_type() {
        KeyLockType::Key => keys.push(current),
        KeyLockType::Lock => locks.push(current),
    }

    (keys, locks)
}

#[aoc(day25, part1)]
fn part1(input: &(Vec<KeyLock>, Vec<KeyLock>)) -> u64 {
    let keys = &input.0;
    let locks = &input.1;

    // for all lock and keys
    locks
        .par_iter()
        // .iter()
        .map(|lock| {
            // keys.iter()
            keys.par_iter()
                .map(|key| if lock.test_with_opposite(key) { 1 } else { 0 })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 3);
    }
}
