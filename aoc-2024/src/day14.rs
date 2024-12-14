use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use regex::Regex;
use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Position(usize, usize);
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Velocity(isize, isize);

impl Position {
    fn move_distance(&mut self, distance: Velocity, max: Position) {
        let mut x = self.0 as isize + distance.0;
        let mut y = self.1 as isize + distance.1;

        if x < 0 {
            x = max.0 as isize - (x.abs() % max.0 as isize);
        }
        if x >= max.0 as isize {
            x = x % max.0 as isize;
        }
        if y < 0 {
            y = max.1 as isize - (y.abs() % max.1 as isize);
        }
        if y >= max.1 as isize {
            y = y % max.1 as isize;
        }

        self.0 = x as usize;
        self.1 = y as usize;
    }
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos({:?}, {:?})", self.0, self.1)
    }
}
impl fmt::Debug for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vel({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Robot {
    pos: Position,
    vec: Velocity,
}

impl Robot {
    fn new(start_pos: Position, velocity: Velocity) -> Self {
        Robot {
            pos: start_pos,
            vec: velocity,
        }
    }

    fn move_robot(&mut self, grid_size: Position) {
        self.pos.move_distance(self.vec, grid_size);
    }

    fn get_quadrant(self, grid_size: Position) -> u8 {
        // println!("{:?}", self);

        // 0 = None. 1 = top left, 2 = top right, 3 = bottom left, 4 = bottom right
        let x_split = grid_size.0 / 2;
        let y_split = grid_size.1 / 2;

        if self.pos.0 < x_split && self.pos.1 < y_split {
            return 1;
        }
        if self.pos.0 > x_split && self.pos.1 < y_split {
            return 2;
        }
        if self.pos.0 < x_split && self.pos.1 > y_split {
            return 3;
        }
        if self.pos.0 > x_split && self.pos.1 > y_split {
            return 4;
        }
        0
    }
}

#[allow(dead_code)]
fn debug_grid(robots: &[Robot], grid_size: Position, split: bool) {
    let x_split = grid_size.0 / 2;
    let y_split = grid_size.1 / 2;
    for y in 0..grid_size.1 {
        if y == y_split && split {
            println!();
            continue;
        }
        for x in 0..grid_size.0 {
            if x == x_split && split {
                print!(" ");
                continue;
            }
            let robots_square = robots
                .iter()
                .filter(|robot| robot.pos == Position(x, y))
                .count();
            if robots_square > 0 {
                print!("{:?}", robots_square);
                continue;
            }
            print!(".");
        }
        println!();
    }
}

fn check_for_line(robots: &[Robot], grid_size: Position) -> bool {
    for y in 0..grid_size.1 {
        let mut robot_pos = robots
            .iter()
            .map(|robot| robot.pos)
            .collect::<Vec<Position>>();
        robot_pos.sort();
        let count = robot_pos.iter().filter(|robot| robot.1 == y);
        if count.count() < 30 {
            continue;
        }
        let min = robot_pos.first().expect("Failed to get first robot pos").0 as f64;
        let max = robot_pos.last().expect("Failed to get last robot pos").0 as f64;
        let range = (max - min) * 0.25;
        let lqr = min + range;
        let uqr = max - range;
        // println!(
        //     "LQR: {:?}, UQR: {:?}. MIN: {:?}. MAX: {:?}",
        //     lqr, uqr, min, max
        // );

        let count2 = robot_pos
            .iter()
            .filter(|robot| robot.0 as f64 >= lqr && robot.0 as f64 <= uqr && robot.1 == y);
        // let c2cl = count2.to_owned();
        let c2c = count2.count();
        // println!("Count2: {:?}. count2: {:#?}", c2c, c2cl);
        if c2c >= 30 {
            return true;
        }
    }

    return false;
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|line| {
            // https://regex101.com/r/mqYDbr/2
            let regex = Regex::new(
                r"(?m)p=((?P<px>\d*),(?P<py>\d*)) v=((?P<vx>(-?)(\d*)),(?P<vy>(-?)(\d*)))",
            )
            .unwrap();
            let groups = regex.captures(line).expect("Failed to capture any groups!");

            Robot::new(
                Position(
                    groups
                        .name("px")
                        .expect("Failed to get x start pos")
                        .as_str()
                        .parse()
                        .expect("Failed to parse x start pos"),
                    groups
                        .name("py")
                        .expect("Failed to get y start pos")
                        .as_str()
                        .parse()
                        .expect("Failed to parse y start pos"),
                ),
                Velocity(
                    groups
                        .name("vx")
                        .expect("Failed to get x velocity")
                        .as_str()
                        .parse()
                        .expect("Failed to parse x velocity"),
                    groups
                        .name("vy")
                        .expect("Failed to get y velocity")
                        .as_str()
                        .parse()
                        .expect("Failed to parse y velocity"),
                ),
            )
        })
        .collect()
}

fn process_robots(input: &[Robot], grid_size: Position) -> usize {
    let mut robots = input.to_owned();
    // println!("Start");
    // debug_grid(&robots, grid_size, false);
    for second in 0..100 {
        robots
            .par_iter_mut()
            .for_each(|robot| robot.move_robot(grid_size));
        // println!("Second: {:?}", second);
        // debug_grid(&robots, grid_size, false);
    }

    let robots_in_quadrants = robots
        .par_iter()
        .map(|robot| robot.get_quadrant(grid_size))
        .collect::<Vec<u8>>();

    // println!("Result");
    // debug_grid(&robots, grid_size, true);
    // println!("Riq: {:?}", robots_in_quadrants);
    let q1 = robots_in_quadrants.iter().filter(|&r| *r == 1).count();
    let q2 = robots_in_quadrants.iter().filter(|&r| *r == 2).count();
    let q3 = robots_in_quadrants.iter().filter(|&r| *r == 3).count();
    let q4 = robots_in_quadrants.iter().filter(|&r| *r == 4).count();

    q1 * q2 * q3 * q4
}

fn find_tree(input: &[Robot], grid_size: Position) -> bool {
    let mut robots = input.to_owned();
    let mut line = false;
    let mut time = 0;
    while !line {
        time += 1;
        robots
            .par_iter_mut()
            .for_each(|robot| robot.move_robot(grid_size));
        line = check_for_line(&robots, grid_size);
    }
    println!("Found this at {:?}", time);
    debug_grid(&robots, grid_size, false);

    line
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    process_robots(input, Position(101, 103))
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> bool {
    find_tree(input, Position(101, 103))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn part1_example() {
        assert_eq!(process_robots(&parse(EXAMPLE_1), Position(11, 7)), 12);
    }
}
