use aoc_runner_derive::aoc;
use core::fmt;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str;

#[derive(Clone, Copy)]
enum Action {
    Set,
    Or,
    Not,
    And,
    LShift,
    RShift,
}

#[derive(Clone, Copy)]
struct InsturctionParamter<'a> {
    gate: &'a [u8],
    value: u16,
}

impl<'a> InsturctionParamter<'a> {
    fn new(info: &'a str) -> Self {
        let a = info.parse::<u16>();
        if let Ok(value) = a {
            Self { gate: &[], value }
        } else {
            Self {
                gate: info.as_bytes(),
                value: u16::MAX,
            }
        }
    }

    fn empty() -> Self {
        Self {
            gate: &[],
            value: u16::MAX,
        }
    }

    fn get_value(&self, wires: &'a HashMap<&'a [u8], u16>) -> Option<&u16> {
        if self.value != u16::MAX {
            Some(&self.value)
        } else {
            wires.get(self.gate)
        }
    }

    fn as_debug(&self) -> String {
        if self.value != u16::MAX {
            self.value.to_string()
        } else {
            "value stored in ".to_string() + String::from_utf8(self.gate.to_vec()).unwrap().as_str()
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction<'a> {
    action: Action,
    gate_a: InsturctionParamter<'a>,
    gate_b: InsturctionParamter<'a>,
    gate_c: InsturctionParamter<'a>,
}

impl<'a> Instruction<'a> {
    fn process(&self, wires: &mut HashMap<&'a [u8], u16>) -> bool {
        match self.action {
            Action::Set => {
                let a_value = self.gate_a.get_value(wires);
                if let Some(v) = a_value {
                    wires.insert(self.gate_b.gate, *v);
                    true
                } else {
                    false
                }
            }
            Action::Or => {
                let a_value = self.gate_a.get_value(wires);
                let b_value = self.gate_b.get_value(wires);

                if let Some(a) = a_value {
                    if let Some(b) = b_value {
                        wires.insert(self.gate_c.gate, a | b);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Action::Not => {
                let a_value = self.gate_a.get_value(wires);

                if let Some(v) = a_value {
                    wires.insert(self.gate_b.gate, !v);
                    true
                } else {
                    false
                }
            }
            Action::And => {
                let a_value = self.gate_a.get_value(wires);
                let b_value = self.gate_b.get_value(wires);

                if let Some(a) = a_value {
                    if let Some(b) = b_value {
                        wires.insert(self.gate_c.gate, a & b);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Action::LShift => {
                let a_value = self.gate_a.get_value(wires);
                let b_value = self.gate_b.get_value(wires);

                if let Some(a) = a_value {
                    if let Some(b) = b_value {
                        wires.insert(self.gate_c.gate, a << b);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Action::RShift => {
                let a_value = self.gate_a.get_value(wires);
                let b_value = self.gate_b.get_value(wires);

                if let Some(a) = a_value {
                    if let Some(b) = b_value {
                        wires.insert(self.gate_c.gate, a >> b);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Debug for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.action {
                Action::Set => {
                    format!(
                        "SET {:?} to {:?}",
                        self.gate_a.as_debug(),
                        self.gate_b.as_debug(),
                    )
                }
                Action::Or => format!(
                    "BITWISE OR {:?} and {:?} storing in {:?}",
                    self.gate_a.as_debug(),
                    self.gate_b.as_debug(),
                    self.gate_c.as_debug(),
                ),
                Action::Not => format!(
                    "BITWISE NOT of {:?} storing in {:?}",
                    self.gate_a.as_debug(),
                    self.gate_b.as_debug(),
                ),
                Action::And => format!(
                    "BITWISE AND {:?} and {:?} storing in {:?}",
                    self.gate_a.as_debug(),
                    self.gate_b.as_debug(),
                    self.gate_c.as_debug(),
                ),
                Action::LShift => format!(
                    "LEFT SHIFT of {:?} by {:?} storing in {:?}",
                    self.gate_a.as_debug(),
                    self.gate_b.as_debug(),
                    self.gate_c.as_debug(),
                ),
                Action::RShift => format!(
                    "RIGHT SHIFT of {:?} by {:?} storing in {:?}",
                    self.gate_a.as_debug(),
                    self.gate_b.as_debug(),
                    self.gate_c.as_debug(),
                ),
            }
        )
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let regex_a = Regex::new(r"(?m)((?P<num>\w+) -> (?P<char>\w+))").unwrap();
    let regex_b = Regex::new(r"(?m)((?P<a>\w+) AND (?P<b>\w+) -> (?P<c>\w+))").unwrap();
    let regex_c = Regex::new(r"(?m)((?P<a>\w+) OR (?P<b>\w+) -> (?P<c>\w+))").unwrap();
    let regex_d = Regex::new(r"(?m)((?P<a>\w+) LSHIFT (?P<b>\w+) -> (?P<c>\w+))").unwrap();
    let regex_e = Regex::new(r"(?m)((?P<a>\w+) RSHIFT (?P<b>\w+) -> (?P<c>\w+))").unwrap();
    let regex_f = Regex::new(r"(?m)NOT (?P<a>\w+) -> (?P<b>\w+)").unwrap();
    input
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|line| {
            // println!("{:?}", line);
            let cb = regex_b.captures(line);
            if cb.is_some() {
                let groups = cb.unwrap();
                return Instruction {
                    action: Action::And,
                    gate_a: InsturctionParamter::new(
                        groups.name("a").expect("Failed to get a").as_str(),
                    ),
                    gate_b: InsturctionParamter::new(
                        groups.name("b").expect("Failed to get b").as_str(),
                    ),
                    gate_c: InsturctionParamter::new(
                        groups.name("c").expect("Failed to get c").as_str(),
                    ),
                };
            }
            let cc = regex_c.captures(line);
            if cc.is_some() {
                let groups = cc.unwrap();
                return Instruction {
                    action: Action::Or,
                    gate_a: InsturctionParamter::new(
                        groups.name("a").expect("Failed to get a").as_str(),
                    ),
                    gate_b: InsturctionParamter::new(
                        groups.name("b").expect("Failed to get b").as_str(),
                    ),
                    gate_c: InsturctionParamter::new(
                        groups.name("c").expect("Failed to get c").as_str(),
                    ),
                };
            }
            let cd = regex_d.captures(line);
            if cd.is_some() {
                let groups = cd.unwrap();
                return Instruction {
                    action: Action::LShift,
                    gate_a: InsturctionParamter::new(
                        groups.name("a").expect("Failed to get a").as_str(),
                    ),
                    gate_b: InsturctionParamter::new(
                        groups.name("b").expect("Failed to get b").as_str(),
                    ),
                    gate_c: InsturctionParamter::new(
                        groups.name("c").expect("Failed to get c").as_str(),
                    ),
                };
            }
            let ce = regex_e.captures(line);
            if ce.is_some() {
                let groups = ce.unwrap();
                return Instruction {
                    action: Action::RShift,
                    gate_a: InsturctionParamter::new(
                        groups.name("a").expect("Failed to get a").as_str(),
                    ),
                    gate_b: InsturctionParamter::new(
                        groups.name("b").expect("Failed to get b").as_str(),
                    ),
                    gate_c: InsturctionParamter::new(
                        groups.name("c").expect("Failed to get c").as_str(),
                    ),
                };
            }
            let cf = regex_f.captures(line);
            if cf.is_some() {
                let groups = cf.unwrap();
                return Instruction {
                    action: Action::Not,
                    gate_a: InsturctionParamter::new(
                        groups.name("a").expect("Failed to get a").as_str(),
                    ),
                    gate_b: InsturctionParamter::new(
                        groups.name("b").expect("Failed to get b").as_str(),
                    ),
                    gate_c: InsturctionParamter::empty(),
                };
            }

            let ca = regex_a.captures(line);
            if ca.is_some() {
                let groups = ca.unwrap();
                return Instruction {
                    action: Action::Set,
                    gate_a: InsturctionParamter::new(
                        groups.name("num").expect("Failed to get number").as_str(),
                    ),
                    gate_b: InsturctionParamter::new(
                        groups.name("char").expect("Failed to get char").as_str(),
                    ),
                    gate_c: InsturctionParamter::empty(),
                };
            }

            panic!("No valid instruction found!");
        })
        .collect_vec()
}

#[allow(dead_code)]
fn debug_wires(wires: &HashMap<&[u8], u16>) {
    let keys = wires.keys();
    for wire in keys.sorted() {
        let info = wires.get(wire);
        println!(
            "{:?}: {:?}",
            str::from_utf8(wire).expect("Failed to translate wire code into name"),
            info.expect("Failed to get wire data")
        );
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u16 {
    let mut wires: HashMap<&[u8], u16> = HashMap::new();
    let mut instructions = parse(input).to_vec();
    let mut skipped: Vec<Instruction<'_>> = vec![];
    // println!("{:?}", wires);
    // println!("{:?}", instructions);

    while let Some(instruction) = instructions.pop() {
        // println!("Processing instruction: {:?}", instruction);
        // println!("{:?}", skipped);
        let result = instruction.process(&mut wires);
        if !result {
            // println!("Failed");
            skipped.push(instruction);
        }

        if instructions.is_empty() {
            instructions.append(&mut skipped);
        }
    }

    // instructions.iter().for_each(|instruction| {
    //     println!("Processing instruction: {:?}", instruction);
    //     instruction.process(&mut wires);
    // });

    // debug_wires(&wires);

    *wires
        .get("a".as_bytes())
        .expect("Failed to get value of wire a")
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u16 {
    let r = part1(input);
    let input = input.replace(
        "44430 -> b",
        (r.to_string().as_str().to_owned() + " -> b")
            .to_string()
            .as_str(),
    );
    part1(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 2);
    }
}
