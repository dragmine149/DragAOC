use aoc_runner_derive::{aoc, aoc_generator};

struct ComputerConnections {
    computer_1: String,
    computer_2: String,
    computer_3: String,
}

impl std::fmt::Debug for ComputerConnections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            if !f.alternate() {
                format!(
                    "{}-{}-{}",
                    self.computer_1, self.computer_2, self.computer_3
                )
            } else {
                format!(
                    "ComputerConnection {{ computer_1: {}, computer_2: {}, computer_3: {} }}",
                    self.computer_1, self.computer_2, self.computer_3
                )
            }
        )
    }
}

impl ComputerConnections {
    fn has_t(&self) -> bool {
        self.computer_1.starts_with('t')
            || self.computer_2.starts_with('t')
            || self.computer_3.starts_with('t')
    }

    fn exists(&self, other: &Self) -> bool {
        (self.computer_1 == other.computer_1
            || self.computer_2 == other.computer_1
            || self.computer_3 == other.computer_1)
            && (self.computer_1 == other.computer_2
                || self.computer_2 == other.computer_2
                || self.computer_3 == other.computer_2)
            && (self.computer_1 == other.computer_3
                || self.computer_2 == other.computer_3
                || self.computer_3 == other.computer_3)
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<ComputerConnections> {
    let mut connections: Vec<ComputerConnections> = vec![];

    input.lines().for_each(|line| {
        // println!("Line: {:?}", line);
        let mut computers = line.trim().split("-");
        let a = computers.next().expect("Failed to parse A");
        let b = computers.next().expect("Failed to parse B");

        let a_connections = input
            .lines()
            .filter(|line| line.contains(a))
            .map(|line| {
                let mut options = line
                    .trim()
                    .split("-")
                    .filter(|a_con| *a_con != a && *a_con != b);
                match options.next() {
                    Some(v) => v,
                    None => "",
                }
            })
            .filter(|l| *l != "");
        // .collect::<Vec<&str>>();
        let b_connections = input
            .lines()
            .filter(|line| line.contains(b))
            .map(|line| {
                let mut options = line
                    .trim()
                    .split("-")
                    .filter(|b_con| *b_con != b && *b_con != a);
                match options.next() {
                    Some(v) => v,
                    None => "",
                }
            })
            .filter(|l| *l != "")
            .collect::<Vec<&str>>();
        // println!("{:?}, {:?}", a_connections, b_connections);
        for a_con in a_connections {
            for b_con in b_connections.iter() {
                if a_con == *b_con {
                    let computer = ComputerConnections {
                        computer_1: a.to_string(),
                        computer_2: b.to_string(),
                        computer_3: a_con.to_string(),
                    };
                    if connections
                        .iter()
                        .find(|comp| comp.exists(&computer))
                        .is_none()
                    {
                        connections.push(computer);
                    }
                }
            }
        }
    });

    connections
}

#[aoc(day23, part1)]
fn part1(input: &[ComputerConnections]) -> u64 {
    // println!("{:?}", input);

    input.iter().filter(|computer| computer.has_t()).count() as u64
}

// #[aoc(day23, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 7);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
