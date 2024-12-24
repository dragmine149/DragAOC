// A lot of imports today
use aoc_runner_derive::aoc;
use graphviz_rust::{
    cmd::Format,
    dot_generator::*,
    dot_structures::*,
    exec_dot,
    printer::{DotPrinter, PrinterContext},
};
use itertools::Itertools;
use std::{collections::HashMap, fs::File, io::Write};

// store an instruction
#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    id_1: &'a str,
    operation: u8,
    id_2: &'a str,
    out: &'a str,
}

// functions used in p2 mainly to have a better out of the instruction
impl<'a> Instruction<'a> {
    fn get_operation(&self) -> String {
        match self.operation {
            0 => "AND".to_string(),
            1 => "OR".to_string(),
            2 => "XOR".to_string(),
            _ => panic!("Invalid operation"),
        }
    }
    fn get_operation_shape(&self) -> String {
        match self.operation {
            0 => "diamond".to_string(),
            1 => "box".to_string(),
            2 => "hexagon".to_string(),
            _ => panic!("Invalid operation"),
        }
    }
}

fn parse_p1(input: &str) -> (HashMap<&str, bool>, Vec<Instruction>) {
    let mut wires = HashMap::new();
    let mut instructions = vec![];

    input
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .for_each(|line| {
            if line.contains(":") {
                // these are pre-set values
                let mut info = line.split(":");
                let id = info.next().expect("Failed to get line id").trim();
                let value = info
                    .next()
                    .expect("Failed to get line value")
                    .trim()
                    .parse::<u8>()
                    .expect("Failed to parse line value")
                    != 0;

                wires.insert(id, value);
                return;
            }

            // these are all the instructions
            if line.is_empty() {
                return;
            }
            let mut spaces = line.split_whitespace();
            let id_1 = spaces.next().expect("Failed to get first id");
            let op_str = spaces.next().expect("Failed to get operation");
            let operation = match op_str {
                "AND" => 0,
                "OR" => 1,
                "XOR" => 2,
                _ => panic!("Invalid operation!"),
            };
            let id_2 = spaces.next().expect("Failed to get second id");
            spaces.next();
            let out = spaces.next().expect("Failed to get output id");
            instructions.push(Instruction {
                id_1,
                operation,
                id_2,
                out,
            });
        });

    (wires, instructions)
}

fn process_instruction<'a>(
    instruction: &Instruction<'a>,
    wires: &mut HashMap<&'a str, bool>,
) -> bool {
    // get the wire states
    let state_a = wires.get(instruction.id_1);
    let state_b = wires.get(instruction.id_2);

    // instructions ONLY HAPPEN when we have both states, so return if we don't
    if state_a.is_none() || state_b.is_none() {
        return false;
    }

    // just translating them to get rid of the Option<>
    let state_a = state_a.expect("Failed to get state a");
    let state_b = state_b.expect("Failed to get state b");

    // apply the operation
    match instruction.operation {
        0 => wires.insert(instruction.out, *state_a && *state_b),
        1 => wires.insert(instruction.out, *state_a || *state_b),
        2 => wires.insert(instruction.out, *state_a ^ *state_b),
        _ => panic!("Invalid operation"),
    };

    true
}

// Probably a better way of doing this, but get all z numbers and calculate the output
fn calculate_num(wires: &HashMap<&str, bool>) -> u64 {
    let mut id = 0;
    let mut bit_value = 0;
    loop {
        let lid = format!("z{:0>width$}", id, width = if id < 10 { 2 } else { 0 });
        // println!("{:?}", lid);
        let v = wires.get(&lid.as_str());
        match v {
            Some(value) => {
                if *value {
                    bit_value += 2_u64.pow(id);
                }
            }
            None => break,
        }
        id += 1;
    }
    // println!("{:?}", bit_value);
    // println!("{:?}", id);

    bit_value
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    let data = parse_p1(input);
    let mut wires = data.0;
    let instructions = data.1;
    // println!("{:?}", wires);
    // println!();
    // println!("{:?}", instructions);
    loop {
        let mut unprocessed = instructions.to_vec();
        let mut failed = false;
        // unpack one at a time
        while let Some(instruction) = unprocessed.pop() {
            let result = process_instruction(&instruction, &mut wires);
            if !result {
                // set fail to loop again
                failed = true;
            }
        }
        // if fail is false then yay
        if !failed {
            break;
        }
        // recycle failed into processed.
        // unprocessed = failed;
    }

    // println!("{:?}", wires);

    calculate_num(&wires)
}

// generate a graph so we can manually find the stuff
// This was not fun to make
fn generate_graph(instructions: &Vec<Instruction>) {
    let mut g = graph!(strict di id!("Instructions"));
    for instruction in instructions {
        let id_1 = NodeId(Id::Plain(instruction.id_1.to_string()), None);
        let id_2 = NodeId(Id::Plain(instruction.id_2.to_string()), None);
        let out = NodeId(Id::Plain(instruction.out.to_string()), None);
        let operation = NodeId(
            Id::Plain(instruction.get_operation() + instruction.out),
            None,
        );

        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(Node::new(
            id_1.clone(),
            vec![Attribute(
                Id::Plain("color".to_string()),
                Id::Plain("red".to_string()),
            )],
        )));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(Node::new(
            id_2.clone(),
            vec![Attribute(
                Id::Plain("color".to_string()),
                Id::Plain("yellow".to_string()),
            )],
        )));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(Node::new(
            out.clone(),
            vec![Attribute(
                Id::Plain("color".to_string()),
                Id::Plain("green".to_string()),
            )],
        )));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(Node::new(
            operation.clone(),
            vec![
                Attribute(
                    Id::Plain("color".to_string()),
                    Id::Plain("cyan".to_string()),
                ),
                Attribute(
                    Id::Plain("shape".to_string()),
                    Id::Plain(instruction.get_operation_shape()),
                ),
                Attribute(
                    Id::Plain("label".to_string()),
                    Id::Plain(instruction.get_operation()),
                ),
            ],
        )));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Edge(Edge {
            ty: EdgeTy::Pair(Vertex::N(id_1), Vertex::N(operation.clone())),
            attributes: vec![],
        }));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Edge(Edge {
            ty: EdgeTy::Pair(Vertex::N(id_2), Vertex::N(operation.clone())),
            attributes: vec![],
        }));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Edge(Edge {
            ty: EdgeTy::Pair(Vertex::N(operation.clone()), Vertex::N(out)),
            attributes: vec![],
        }));
    }
    let dot = g.print(&mut PrinterContext::default());
    // println!("{}", dot);
    let format = Format::Png;
    let graph = exec_dot(dot, vec![format.into()]).unwrap();

    let mut file = File::create("day24_graph.png").expect("Failed to open file");
    let _ = file
        .write(graph.as_slice())
        .expect("Failed to write to file");
}

#[aoc(day24, part2)]
fn part2(input: &str) -> String {
    let data = parse_p1(input);
    // let mut wires = data.0;
    let instructions = data.1;

    generate_graph(&instructions);

    "".to_string()
}

// Thanks guy_732 once again, and for helping find the anomalies.
#[allow(unused)]
#[aoc(day24, part2, hard_coded)]
fn part2_hard_coded(input: &str) -> String {
    // Those were found by manually inspecting the graphviz output of the connections
    const SWAPPED_PAIRS: [(&str, &str); 4] = [
        // Anomaly near x10, y10 and z10
        ("kmb", "z10"),
        // Anomaly near x15, y15 and z15
        ("tvp", "z15"),
        // Anomaly near x25, y25 and z25
        ("dpg", "z25"),
        // Anomaly near x35, y35 and z35
        ("mmf", "vdk"),
    ];

    // let gates = parse(input);
    // gates
    // .dump_to_file("input.gv")
    // .expect("Could not write graph to external file");

    let mut swapped: [&str; 8] = SWAPPED_PAIRS
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect_vec()
        .try_into()
        .expect("SWAPPED_PAIRS did not generate 8 values");

    swapped.sort_unstable();
    let mut result = swapped[0].to_owned();
    for el in &swapped[1..] {
        result.push(',');
        result.push_str(el);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const EXAMPLE_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    const EXAMPLE_3: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(EXAMPLE_2), 2024);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_3), "z00,z01,z02,z05");
    }
}
