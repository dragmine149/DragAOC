use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

// create a reindeer
#[derive(Clone, Copy)]
struct Reindeer {
    // name: String,
    speed: u64,
    time: u64,
    rest: u64,
}

// create a reindeer with more data
struct ScoreReindeer {
    reindeer: Reindeer,
    current_distance: u64,
    current_score: u64,
    resting: u64,
    flying: u64,
}

impl Reindeer {
    fn calculate_distance_after_time(&self, time: u64) -> u64 {
        // does some basic really simple maths to get the distance
        let cycle_time = self.time + self.rest;
        let cycles = time / cycle_time;
        let distance = (self.speed * self.time) * cycles;
        let left = time % cycle_time;
        let extra = self.speed * if left > self.time { self.time } else { left };
        // let total = distance + extra;

        // println!(
        //     "A cycle takes {:?} ({:?} + {:?}) time",
        //     cycle_time, self.time, self.rest
        // );
        // println!("The total cycles possible are: {:?}", cycles);
        // println!(
        //     "Hence we can go {:?} ({:?} * {:?})",
        //     distance,
        //     self.speed * self.time,
        //     cycles
        // );
        // println!("Time left over: {:?}", left);
        // println!("equates to an extra: {:?}", extra);
        // println!("Hence we go: {:?}", total);

        // total
        distance + extra
    }
}

impl ScoreReindeer {
    // create a score version of a reindeer
    fn from(reindeer: Reindeer) -> Self {
        let fly = reindeer.time;
        Self {
            reindeer,
            current_score: 0,
            current_distance: 0,
            resting: 0,
            flying: fly,
        }
    }

    // increment a second
    fn increment_second(&mut self) {
        if self.resting == 0 {
            self.current_distance += self.reindeer.speed;
            self.flying -= 1;

            if self.flying == 0 {
                self.resting = self.reindeer.rest;
            }

            return;
        }

        self.resting -= 1;
        if self.resting == 0 {
            self.flying = self.reindeer.time;
        }
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Reindeer> {
    let regex = Regex::new(r"(?m)(?<name>\w+) .* (?P<speed>\d+) .* (?P<time>\d+) .* (?P<rest>\d+)")
        .unwrap();
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            Reindeer {
                // name: captures.name("name").unwrap().as_str().to_string(),
                speed: captures
                    .name("speed")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse speed"),
                time: captures
                    .name("time")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse time"),
                rest: captures
                    .name("rest")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse rest"),
            }
        })
        .collect_vec()
}

fn process_reindeers(input: &[Reindeer], time: u64) -> u64 {
    input
        .iter()
        .map(|r| r.calculate_distance_after_time(time))
        .max()
        .expect("Failed to get max time")
}

#[aoc(day14, part1)]
fn part1(input: &[Reindeer]) -> u64 {
    process_reindeers(input, 2503)
}

fn process_score_reindeer(input: &[Reindeer], time: u64) -> u64 {
    let mut reindeers = input.iter().map(|r| ScoreReindeer::from(*r)).collect_vec();

    for _ in 0..time {
        // increment
        reindeers.iter_mut().for_each(|r| r.increment_second());
        // get furthest
        let dist = reindeers
            .iter()
            .map(|r| r.current_distance)
            .max()
            .expect("Failed to get furthest distance");
        // give all furthest a point
        reindeers
            .iter_mut()
            .filter(|r| r.current_distance == dist)
            .for_each(|r| r.current_score += 1);
    }

    // calculate best
    reindeers
        .iter()
        .map(|r| r.current_score)
        .max()
        .expect("Failed to get best score")
}

#[aoc(day14, part2)]
fn part2(input: &[Reindeer]) -> u64 {
    process_score_reindeer(input, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

    #[test]
    fn part1_example() {
        assert_eq!(process_reindeers(&parse(EXAMPLE_1), 1000), 1120);
    }

    #[test]
    fn part2_example() {
        assert_eq!(process_score_reindeer(&parse(EXAMPLE_1), 1000), 689);
    }
}
