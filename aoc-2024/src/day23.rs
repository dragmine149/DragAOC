use aoc_runner_derive::aoc;

// store the three way computer loop
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
    // check if the computer has a 't', aka it's our historian.
    fn has_t(&self) -> bool {
        self.computer_1.starts_with('t')
            || self.computer_2.starts_with('t')
            || self.computer_3.starts_with('t')
    }

    // checks if the computer is already connected to this computer
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

fn parse_p1(input: &str) -> Vec<ComputerConnections> {
    let mut connections: Vec<ComputerConnections> = vec![];

    // go though all lines and make the connections
    input.lines().for_each(|line| {
        // println!("Line: {:?}", line);
        let mut computers = line.trim().split("-");
        // get the computers
        let a = computers.next().expect("Failed to parse A");
        let b = computers.next().expect("Failed to parse B");

        // get the computer connections
        let a_connections = input
            .lines()
            .filter(|line| line.contains(a))
            .map(|line| {
                let mut options = line
                    .trim()
                    .split("-")
                    // make sure to only include unique computers
                    .filter(|a_con| *a_con != a && *a_con != b);
                options.next().unwrap_or("")
            })
            .filter(|l| !l.is_empty());
        let b_connections = input
            .lines()
            .filter(|line| line.contains(b))
            .map(|line| {
                let mut options = line
                    .trim()
                    .split("-")
                    .filter(|b_con| *b_con != b && *b_con != a);
                options.next().unwrap_or("")
            })
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();
        // println!("{:?}, {:?}", a_connections, b_connections);

        // loop though all connections a and b has
        for a_con in a_connections {
            for b_con in b_connections.iter() {
                // check if they have any in common
                if a_con == *b_con {
                    // make a new computer
                    let computer = ComputerConnections {
                        computer_1: a.to_string(),
                        computer_2: b.to_string(),
                        computer_3: a_con.to_string(),
                    };

                    // semi-lazy, check if this computer hasn't been made before
                    if !connections.iter().any(|comp| comp.exists(&computer)) {
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

    // filter and count
    computers.iter().filter(|computer| computer.has_t()).count() as u64
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    name: &'a str,
    neighbours: Vec<&'a str>,
}

impl<'a> Node<'a> {
    // check if this node has another node as it's neighbour
    fn has_node_as_neighbours(&self, other: &Self) -> bool {
        self.neighbours.contains(&other.name)
    }
}

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Modified from: https://iq.opengenus.org/bron-kerbosch-algorithm/
// well translated and slightly modified
fn find_cliques<'a>(
    potenital: Vec<Node<'a>>,
    remaining: Vec<Node<'a>>,
    mut skip: Vec<Node<'a>>,
) -> Vec<Vec<Node<'a>>> {
    if remaining.is_empty() && skip.is_empty() {
        // println!("Clique found: {:?}", potenital);
        // Clique has been found, we can then return the list of cliques.
        return vec![potenital];
    }

    // store the found cliques
    let mut found_cliques: Vec<Vec<Node>> = vec![];
    // for all remaining nodes to check
    for node in remaining.iter() {
        // the current node will always be a potenital clique
        let mut new_potential = potenital.to_vec();
        new_potential.push(node.clone());

        // make the new remaning list
        let mut new_remaining = vec![];
        for n in &remaining {
            // ignore ones that we have already skipped
            if skip.contains(n) {
                continue;
            }
            // only add the node if it's a neighbours of ours
            if node.has_node_as_neighbours(n) {
                new_remaining.push(n.clone());
            }
        }

        // skip nodes that are neighbours of ours as well, we added them in remaining
        let mut new_skip = vec![];
        for n in &skip {
            if node.has_node_as_neighbours(n) {
                new_skip.push(n.clone());
            }
        }

        // recurrsion
        let mut cliques = find_cliques(new_potential, new_remaining, new_skip);
        found_cliques.append(&mut cliques);

        // stores the nodes we've already done
        skip.push(node.clone());
    }
    found_cliques
}

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    let mut computer_nodes: Vec<Node> = vec![];
    // generate a list of nodes and all their neighbours
    input.lines().for_each(|line| {
        let mut computers = line.trim().split("-");
        let a = computers.next().expect("Failed to get first computer");
        let b = computers.next().expect("Failed to get second computer");

        let node = computer_nodes.iter().position(|comp| comp.name == a);
        match node {
            Some(v) => {
                let comp = &mut computer_nodes[v];
                comp.neighbours.push(b);
            }
            None => computer_nodes.push(Node {
                name: a,
                neighbours: vec![b],
            }),
        }

        let node2 = computer_nodes.iter().position(|comp| comp.name == b);
        match node2 {
            Some(v) => {
                let comp = &mut computer_nodes[v];
                comp.neighbours.push(a);
            }
            None => computer_nodes.push(Node {
                name: b,
                neighbours: vec![a],
            }),
        }
    });

    // This list is the examples from the website mentioned above, used for testing.
    // println!("{:?}", computer_nodes);
    // println!("{:?}", computer_nodes.len());
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

    let cliques = find_cliques(vec![], computer_nodes, vec![]);
    // println!("{:?}", cliques);
    // println!("{:?}", cliques.len());

    // get the size of the longest
    let biggest = cliques
        .to_vec()
        .iter()
        .map(|clique| clique.len())
        .max()
        .expect("Failed to get max size");

    // loop through all the cliques
    let biggest_v = cliques
        .iter()
        // get the longest
        .filter(|clique| clique.len() == biggest)
        .map(|clique| {
            // convert the clique into the output string
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
