use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    id_1: &'a str,
    operation: u8,
    id_2: &'a str,
    out: &'a str,
}

fn parse_p1(input: &str) -> (HashMap<&str, bool>, Vec<Instruction>) {
    let mut wires = HashMap::new();
    let mut instructions = vec![];

    input
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .for_each(|line| {
            if line.contains(":") {
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

            if line.is_empty() {
                return;
            }
            let mut spaces = line.trim().split_whitespace();
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
    let state_a = wires.get(instruction.id_1);
    let state_b = wires.get(instruction.id_2);

    if state_a.is_none() || state_b.is_none() {
        return false;
    }

    let state_a = state_a.expect("Failed to get state a");
    let state_b = state_b.expect("Failed to get state b");

    match instruction.operation {
        0 => wires.insert(instruction.out, *state_a && *state_b),
        1 => wires.insert(instruction.out, *state_a || *state_b),
        2 => wires.insert(instruction.out, *state_a ^ *state_b),
        _ => panic!("Invalid operation"),
    };

    return true;
}

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
        let mut failed = vec![];
        while let Some(instruction) = unprocessed.pop() {
            let result = process_instruction(&instruction, &mut wires);
            if !result {
                failed.push(instruction);
            }
        }
        if failed.is_empty() {
            break;
        }
        unprocessed = failed;
    }

    // println!("{:?}", wires);

    calculate_num(&wires)
}

// #[aoc(day24, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(EXAMPLE_2), 2024);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
