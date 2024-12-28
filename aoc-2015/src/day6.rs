use crate::utils::{Grid, Position};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

enum InstructionType {
    On,
    Off,
    Toggle,
}

struct Instruction {
    instruction_type: InstructionType,
    start: Position,
    end: Position,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"(?m)((?P<on>turn on) (?P<on_start_x>\d*),(?P<on_start_y>\d*) through (?P<on_end_x>\d*),(?P<on_end_y>\d*))|((?P<toggle>toggle) (?P<toggle_start_x>\d*),(?P<toggle_start_y>\d*) through (?P<toggle_end_x>\d*),(?P<toggle_end_y>\d*))|((?P<off>turn off) (?P<off_start_x>\d*),(?P<off_start_y>\d*) through (?P<off_end_x>\d*),(?P<off_end_y>\d*))").unwrap();

    input
        .lines()
        .map(|line| {
            let capt = regex.captures(line).unwrap();
            let start_pos: Position;
            let end_pos: Position;

            let instr_type = if capt.name("on").is_some() {
                start_pos = Position(
                    capt.name("on_start_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                    capt.name("on_start_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                );
                end_pos = Position(
                    capt.name("on_end_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                    capt.name("on_end_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                );
                InstructionType::On
            } else if capt.name("off").is_some() {
                start_pos = Position(
                    capt.name("off_start_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                    capt.name("off_start_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                );
                end_pos = Position(
                    capt.name("off_end_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                    capt.name("off_end_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                );
                InstructionType::Off
            } else if capt.name("toggle").is_some() {
                start_pos = Position(
                    capt.name("toggle_start_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                    capt.name("toggle_start_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                );
                end_pos = Position(
                    capt.name("toggle_end_x")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                    capt.name("toggle_end_y")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse"),
                );
                InstructionType::Toggle
            } else {
                println!("{:?}", capt);
                panic!("Invalid line: {:?}", line);
            };

            Instruction {
                instruction_type: instr_type,
                start: start_pos,
                end: end_pos,
            }
        })
        .collect_vec()
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> u64 {
    let mut grid = Grid::new(&Position(1000, 1000), false);

    input
        .iter()
        .for_each(|instruction| match instruction.instruction_type {
            InstructionType::On => grid.set_cell_range(&instruction.start, &instruction.end, true),
            InstructionType::Off => {
                grid.set_cell_range(&instruction.start, &instruction.end, false)
            }
            InstructionType::Toggle => {
                grid.get_set_cell_range(&instruction.start, &instruction.end, |v| !*v)
            }
        });

    grid.0
        .iter()
        .map(|a| a.iter().map(|b| if *b { 1 } else { 0 }).sum::<u64>())
        .sum()
}

// #[aoc(day6, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }
