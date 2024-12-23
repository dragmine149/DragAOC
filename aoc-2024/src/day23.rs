use aoc_runner_derive::aoc;

// store the three way computer loop
struct ComputerConnections {
    computer_1: usize,
    computer_2: usize,
    computer_3: usize,
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
    fn has_t(&self, mapping: &[String]) -> bool {
        mapping[self.computer_1].starts_with('t')
            || mapping[self.computer_2].starts_with('t')
            || mapping[self.computer_3].starts_with('t')
    }

    // checks if the computer is already connected to this computer
    fn exists(&self, con_1: usize, con_2: usize, con_3: usize) -> bool {
        (self.computer_1 == con_1 || self.computer_2 == con_1 || self.computer_3 == con_1)
            && (self.computer_1 == con_2 || self.computer_2 == con_2 || self.computer_3 == con_2)
            && (self.computer_1 == con_3 || self.computer_2 == con_3 || self.computer_3 == con_3)
    }
}

#[aoc_generator(day23, part1)]
fn parse(input: &str) -> (Vec<String>, Vec<(usize, usize)>) {
    let mut keys = vec![];
    // create an index based
    let indexs = input
        .lines()
        .map(|line| {
            let mut computers = line.trim().split("-");
            let a = computers.next().expect("Failed to get first computer");
            let a_pos = keys.iter().position(|key| key == a);
            let a_index = match a_pos {
                Some(v) => v,
                None => {
                    keys.push(a.to_string());
                    keys.len() - 1
                }
            };

            let b = computers.next().expect("Failed to get second computer");
            let b_pos = keys.iter().position(|key| key == b);
            let b_index = match b_pos {
                Some(v) => v,
                None => {
                    keys.push(b.to_string());
                    keys.len() - 1
                }
            };

            (a_index, b_index)
        })
        .collect();
    (keys, indexs)
}

fn parse_p1(input: &[(usize, usize)]) -> Vec<ComputerConnections> {
    let mut connections: Vec<ComputerConnections> = vec![];

    // for each computer pair
    input.iter().for_each(|computer_set| {
        let a = computer_set.0;
        let b = computer_set.1;

        // get the connections
        let a_connections = input
            .iter()
            .filter(|c| c.0 == a || c.1 == a)
            .map(|c| {
                if c.0 != a && c.0 != b {
                    c.0
                } else if c.1 != a && c.1 != b {
                    c.1
                } else {
                    usize::MAX
                }
            })
            .filter(|c| *c != usize::MAX);
        let b_connections = input
            .iter()
            .filter(|c| c.0 == b || c.1 == b)
            .map(|c| {
                if c.0 != a && c.0 != b {
                    c.0
                } else if c.1 != a && c.1 != b {
                    c.1
                } else {
                    usize::MAX
                }
            })
            .filter(|c| *c != usize::MAX)
            .collect::<Vec<usize>>();

        // make and check the connections
        for a_con in a_connections {
            for b_con in &b_connections {
                if a_con == *b_con && !connections.iter().any(|comp| comp.exists(a, b, a_con)) {
                    let computer = ComputerConnections {
                        computer_1: a,
                        computer_2: b,
                        computer_3: a_con,
                    };
                    connections.push(computer);
                }
            }
        }
    });

    connections
}

#[aoc(day23, part1)]
fn part1(input: &(Vec<String>, Vec<(usize, usize)>)) -> u64 {
    let computers = parse_p1(&input.1);
    // println!("{:?}", computers);

    // filter and count
    computers
        .iter()
        .filter(|computer| computer.has_t(&input.0))
        .count() as u64
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
        assert_eq!(part1(&parse(EXAMPLE_1)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_1), "co,de,ka,ta");
    }
}
