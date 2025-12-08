use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(isize, isize, isize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Network(Vec<Point>);
#[derive(Debug, Clone)]
pub struct PointDistance(Point, Point, f64);

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
impl Ord for PointDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.total_cmp(&other.2)
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

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}
impl Network {
    fn has_points(&self, points: &[Point]) -> bool {
        points.iter().any(|p| self.0.contains(p))
    }
}
impl PointDistance {
    fn nodes(&self) -> [Point; 2] {
        [self.0, self.1]
    }
}

fn get_shortest_distances(nodes: &[Point]) -> Vec<PointDistance> {
    nodes
        .iter()
        .map(|n| {
            nodes
                .iter()
                .map(|n2| PointDistance(*n, *n2, n.distance(n2)))
                // .inspect(|d| println!("{:?}", d))
                .filter(|d| d.2 > 0.0)
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn calculate_part1(input: &[Point], mut connections: usize) -> usize {
    let mut distances = get_shortest_distances(input);
    distances.sort();
    // println!("[");
    // distances.iter().for_each(|d| println!("    {:?},", d));
    // println!("]");
    // println!();
    let mut networks: Vec<Network> = vec![];
    for dist in distances {
        if connections == 0 {
            break;
        }

        println!();
        let existing = networks
            .iter_mut()
            .filter(|net| net.has_points(&dist.nodes()))
            // .inspect(|a| println!("{:?}", a))
            .next();

        if let Some(net) = existing {
            if !net.0.contains(&dist.0) {
                net.0.push(dist.0);
                // println!("p1/added {:?} to {:?}", dist.0, net);
                connections -= 1;
                continue;
            } else if !net.0.contains(&dist.1) {
                net.0.push(dist.1);
                // println!("p2/added {:?} to {:?}", dist.1, net);
                connections -= 1;
                continue;
            }
            // println!(
            //     "Network already contains all points {:?} -> {:?}",
            //     dist, net
            // );
            continue;
        }
        // println!("New network with {:?}", vec![dist.0, dist.1]);
        networks.push(Network(vec![dist.0, dist.1]));
        connections -= 1;
    }
    networks.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    println!("{:#?}", networks);
    // 0
    networks[0].0.len() * networks[1].0.len() * networks[2].0.len()
}

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
    calculate_part1(input, 1000)
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
        assert_eq!(calculate_part1(&parse(EXAMPLE_1), 10), 40);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
