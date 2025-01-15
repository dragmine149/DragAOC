use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Position(usize, usize);
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Velocity(isize, isize);

impl Position {
    // work out maths to make the robot move
    fn move_distance(&mut self, distance: Velocity, max: Position) {
        let mut x = self.0 as isize + distance.0;
        let mut y = self.1 as isize + distance.1;

        // .abs() is used in the rare cases where the robot loops around more than twice at once.
        if x < 0 {
            x = max.0 as isize - (x.abs() % max.0 as isize);
        }
        if x >= max.0 as isize {
            x %= max.0 as isize;
        }
        if y < 0 {
            y = max.1 as isize - (y.abs() % max.1 as isize);
        }
        if y >= max.1 as isize {
            y %= max.1 as isize;
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

        // Check which quadrant the robot is in thanks to easy maths due to odd size grid.
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

// count how many robots are in each space and print them out. Useful for debugging or finding the christmas tree.
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

// Check to see if a line has a group of at least 30 in the IQR.
// Used for finding the tree
#[allow(dead_code)]
fn check_for_line(robots: &[Robot], grid_size: Position) -> bool {
    for y in 0..grid_size.1 {
        // don't worry about other info
        let mut robot_pos = robots
            .iter()
            .map(|robot| robot.pos)
            .collect::<Vec<Position>>();
        robot_pos.sort();
        let count = robot_pos.iter().filter(|robot| robot.1 == y);
        // not enough robots
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

    false
}

// Improved version of tree search, just hope that the tree isn't too split out across quadrants
#[allow(dead_code)]
fn check_quadrant(robots: &[Robot], grid_size: Position) -> bool {
    let robots_in_quadrants = robots
        .iter()
        .map(|robot| robot.get_quadrant(grid_size))
        .collect::<Vec<u8>>();

    let q1 = robots_in_quadrants.iter().filter(|&r| *r == 1).count();
    let q2 = robots_in_quadrants.iter().filter(|&r| *r == 2).count();
    let q3 = robots_in_quadrants.iter().filter(|&r| *r == 3).count();
    let q4 = robots_in_quadrants.iter().filter(|&r| *r == 4).count();

    q1 >= 300 || q2 >= 300 || q3 >= 300 || q4 >= 300
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

// part 1: move the robots and see where they end up at.
fn process_robots(input: &[Robot], grid_size: Position) -> usize {
    let mut robots = input.to_owned();
    // println!("Start");
    // debug_grid(&robots, grid_size, false);
    for _second in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.move_robot(grid_size));
        // println!("Second: {:?}", second);
        // debug_grid(&robots, grid_size, false);
    }

    let robots_in_quadrants = robots
        .iter()
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

// part 2 search every second until tree might have been found.
fn find_tree(input: &[Robot], grid_size: Position) -> bool {
    let mut robots = input.to_owned();
    let mut line = false;
    // let mut time = 0;
    while !line {
        // time += 1;
        robots
            .iter_mut()
            .for_each(|robot| robot.move_robot(grid_size));
        // line = check_for_line(&robots, grid_size);
        line = check_quadrant(&robots, grid_size);
    }
    // println!("Found this at {:?}", time);
    // debug_grid(&robots, grid_size, false);

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
