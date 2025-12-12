use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Hash)]
pub struct Present {
    pub id: u8,
    pub sq1: bool,
    pub sq2: bool,
    pub sq3: bool,
    pub sq4: bool,
    pub sq5: bool,
    pub sq6: bool,
    pub sq7: bool,
    pub sq8: bool,
    pub sq9: bool,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> String {
    // let iter = input.lines();
    todo!()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
