use aoc_runner_derive::aoc;

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

// #[aoc_generator(day23)]
fn parse_p1(input: &str) -> Vec<ComputerConnections> {
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
fn part1(input: &str) -> u64 {
    let computers = parse_p1(input);
    // println!("{:?}", computers);
    computers.iter().filter(|computer| computer.has_t()).count() as u64
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    name: String,
    neighbours: Vec<String>,
}

impl Node {
    fn has_node_as_neighbours(&self, other: &Self) -> bool {
        self.neighbours.contains(&other.name)
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Modified from: https://iq.opengenus.org/bron-kerbosch-algorithm/
fn find_cliques(
    potenital: Vec<Node>,
    remaining: Vec<Node>,
    mut skip: Vec<Node>,
    depth: usize,
) -> Vec<Vec<Node>> {
    if remaining.len() == 0 && skip.len() == 0 {
        println!("Clique found: {:?}", potenital);
        return vec![potenital];
    }

    let mut found_cliques: Vec<Vec<Node>> = vec![];
    for node in remaining.iter() {
        let mut new_potential = potenital.to_vec();
        new_potential.push(node.clone());

        let mut new_remaining = vec![];
        for n in &remaining {
            if skip.contains(n) {
                continue;
            }
            if node.has_node_as_neighbours(&n) {
                new_remaining.push(n.clone());
            }
        }

        let mut new_skip = vec![];
        for n in &skip {
            if node.has_node_as_neighbours(&n) {
                new_skip.push(n.clone());
            }
        }

        let mut cliques = find_cliques(new_potential, new_remaining, new_skip, depth + 1);
        found_cliques.append(&mut cliques);

        skip.push(node.clone());
    }
    found_cliques
}

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    let mut computer_nodes: Vec<Node> = vec![];
    input.lines().for_each(|line| {
        let mut computers = line.trim().split("-");
        let a = computers.next().expect("Failed to get first computer");
        let b = computers.next().expect("Failed to get second computer");

        let node = computer_nodes.iter().position(|comp| comp.name == a);
        match node {
            Some(v) => {
                let comp = &mut computer_nodes[v];
                comp.neighbours.push(b.to_string());
            }
            None => computer_nodes.push(Node {
                name: a.to_string(),
                neighbours: vec![b.to_string()],
            }),
        }

        let node2 = computer_nodes.iter().position(|comp| comp.name == b);
        match node2 {
            Some(v) => {
                let comp = &mut computer_nodes[v];
                comp.neighbours.push(a.to_string());
            }
            None => computer_nodes.push(Node {
                name: b.to_string(),
                neighbours: vec![a.to_string()],
            }),
        }
    });

    println!("{:?}", computer_nodes);
    println!("{:?}", computer_nodes.len());
    // let computer_nodes: Vec<Node> = vec![
    //     Node {
    //         name: "A".to_string(),
    //         neighbours: vec!['B'.to_string(), 'C'.to_string(), 'E'.to_string()],
    //     },
    //     Node {
    //         name: "B".to_string(),
    //         neighbours: vec![
    //             'A'.to_string(),
    //             'C'.to_string(),
    //             'D'.to_string(),
    //             'F'.to_string(),
    //         ],
    //     },
    //     Node {
    //         name: "C".to_string(),
    //         neighbours: vec![
    //             'A'.to_string(),
    //             'B'.to_string(),
    //             'D'.to_string(),
    //             'F'.to_string(),
    //         ],
    //     },
    //     Node {
    //         name: "D".to_string(),
    //         neighbours: vec![
    //             'C'.to_string(),
    //             'B'.to_string(),
    //             'E'.to_string(),
    //             'F'.to_string(),
    //         ],
    //     },
    //     Node {
    //         name: "E".to_string(),
    //         neighbours: vec!['A'.to_string(), 'D'.to_string()],
    //     },
    //     Node {
    //         name: "F".to_string(),
    //         neighbours: vec!['B'.to_string(), 'C'.to_string(), 'D'.to_string()],
    //     },
    // ];

    let cliques = find_cliques(vec![], computer_nodes, vec![], 0);
    println!("{:?}", cliques);
    println!("{:?}", cliques.len());

    let biggest = cliques
        .to_vec()
        .iter()
        .map(|clique| clique.len())
        .max()
        .expect("Failed to get max size");
    let biggest_v = cliques
        .iter()
        .filter(|clique| clique.len() == biggest)
        .map(|clique| {
            let mut c = clique.to_owned();
            c.sort();
            c.iter()
                .map(|node| node.name.to_owned())
                .collect::<Vec<String>>()
                .join(",")
        })
        .collect::<String>();

    biggest_v
}

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
        assert_eq!(part1(EXAMPLE_1), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_1), "co,de,ka,ta");
    }
}
