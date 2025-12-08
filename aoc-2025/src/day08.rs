use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(isize, isize, isize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Network(Vec<Point>);
#[derive(Debug)]
pub struct PointDistance(Point, f64);

impl From<(isize, isize, isize)> for Point {
    fn from(value: (isize, isize, isize)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
impl From<Point> for Network {
    fn from(value: Point) -> Self {
        Self(vec![value])
    }
}
impl Point {
    fn distance(&self, other: &Self) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}
impl Ord for PointDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.total_cmp(&other.1)
    }
}
impl PartialOrd for PointDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for PointDistance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for PointDistance {}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            Point::from(
                line.split(",")
                    .map(|p| {
                        p.parse::<isize>()
                            .expect(&format!("Failed to parse to usize ({:?})", p))
                    })
                    .collect_tuple::<(isize, isize, isize)>()
                    .expect("3 numbers in a tuble"),
            )
        })
        .collect_vec()
}

#[aoc(day8, part1)]
fn part1(input: &[Point]) -> usize {
    let mut networks: Vec<Network> = vec![];
    let mut connections_left = 10_usize;
    input.iter().for_each(|node| {
        // no point scanning the distances if we can't connect.
        if connections_left == 0 {
            return;
        }

        let closest = input
            .iter()
            .map(|p| PointDistance(*p, node.distance(p)))
            .filter(|d| d.1 > 0.0)
            .min()
            .expect("Failed to find closest");
        println!("closest to {:?} = {:?}", node, closest);
        let existing = networks
            .iter_mut()
            .filter(|net| net.0.contains(node) || net.0.contains(&closest.0))
            .next();
        if let Some(net) = existing {
            if !net.0.contains(node) {
                net.0.push(*node);
            } else if !net.0.contains(&closest.0) {
                net.0.push(closest.0);
            }
        } else {
            networks.push(Network(vec![*node, closest.0]));
        }
        connections_left -= 1;
    });
    println!("{:#?}", networks);

    networks.sort();
    networks[0].0.len() * networks[1].0.len() * networks[2].0.len()
}

// #[aoc(day8, part2)]
// fn part2(input: &[Point]) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 40);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
