use aoc_runner_derive::aoc;
use regex::Regex;
use std::ops::BitXor;

struct Register {
    value: usize,
}

// computer, hello computer
// https://www.youtube.com/watch?v=hShY6xZWVGE
struct Computer {
    regiseter_a: Register,
    regiseter_b: Register,
    regiseter_c: Register,
    instruction_pointer: usize,
    instructions: Vec<u8>,
    halt: bool,
    jump: bool,
    output: Vec<u8>,
}

impl Computer {
    // quick way to test everything
    fn test(&mut self) -> Vec<u8> {
        while !self.halt {
            self.process_instruction();
        }
        let out = self.output.to_owned();
        self.reset();
        out
    }

    // quick way to reset as most of the data is already 0 to begin with.
    fn reset(&mut self) {
        self.regiseter_a = Register { value: 0 };
        self.regiseter_b = Register { value: 0 };
        self.regiseter_c = Register { value: 0 };
        self.instruction_pointer = 0;
        self.halt = false;
        self.jump = false;
        self.output.clear();
    }

    // process all the instructions
    fn process_instructions(&mut self) -> String {
        while !self.halt {
            self.process_instruction();
        }

        self.output
            .iter()
            .map(|out| out.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    // procress an instruction
    fn process_instruction(&mut self) {
        if self.halt {
            // check if not broken
            return;
        }

        // check if not at end
        if self.instruction_pointer >= self.instructions.len() {
            self.halt = true;
            return;
        }
        self.jump = false; // reset jumping

        let opcode = self.instructions[self.instruction_pointer];
        let operand = self.instructions[self.instruction_pointer + 1];

        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => {
                println!("Opcode: {:?}", opcode);
                panic!("Invalid opcode")
            }
        }

        // don't increment if we jumped
        if !self.jump {
            self.instruction_pointer += 2;
        }
    }

    // operand can be different based on the intruction
    fn get_operand_value(&self, operand: u8, literal: bool) -> usize {
        if literal {
            return operand as usize;
        }

        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.regiseter_a.value,
            5 => self.regiseter_b.value,
            6 => self.regiseter_c.value,
            7 => {
                // reserved so shouldn't be called but just in case.
                panic!("Operand 7 is reservered.");
            }
            _ => {
                println!("Operand: {:?}", operand);
                panic!("Invalid operand")
            }
        }
    }

    // main opcode processing.

    fn adv(&mut self, operand: u8) {
        let operand = self.get_operand_value(operand, false);
        self.regiseter_a.value /= 2_usize.pow(operand as u32);
    }

    fn bxl(&mut self, operand: u8) {
        let operand = self.get_operand_value(operand, true);
        self.regiseter_b.value = self.regiseter_b.value.bitxor(operand);
    }

    fn bst(&mut self, operand: u8) {
        let operand = self.get_operand_value(operand, false);
        self.regiseter_b.value = operand % 8;
    }

    fn jnz(&mut self, operand: u8) {
        if self.regiseter_a.value == 0 {
            return;
        }

        let operand = self.get_operand_value(operand, true);
        self.jump = true;
        self.instruction_pointer = operand;
    }

    fn bxc(&mut self, _operand: u8) {
        self.regiseter_b.value = self.regiseter_b.value.bitxor(self.regiseter_c.value);
    }

    fn out(&mut self, operand: u8) {
        let operand = self.get_operand_value(operand, false);
        self.output.push((operand % 8) as u8);
    }

    fn bdv(&mut self, operand: u8) {
        let operand = self.get_operand_value(operand, false);
        self.regiseter_b.value = self.regiseter_a.value / 2_usize.pow(operand as u32);
    }

    fn cdv(&mut self, operand: u8) {
        let operand = self.get_operand_value(operand, false);
        self.regiseter_c.value = self.regiseter_a.value / 2_usize.pow(operand as u32);
    }
}

// generate the computer
fn parse(input: &str) -> Computer {
    let regex = Regex::new(
        r"Register A: (?P<rega>\d*)\nRegister B: (?P<regb>\d*)\nRegister C: (?P<regc>\d*)\n\nProgram: (?P<p>(\d,)*\d)",
    )
    .unwrap();
    let groups = regex
        .captures(input)
        .expect("Failed to get groups on input!");
    // println!("{:?}", groups);

    Computer {
        regiseter_a: Register {
            value: groups
                .name("rega")
                .expect("Failed to get reg a value")
                .as_str()
                .parse()
                .expect("Failed to convert reg a to num"),
        },
        regiseter_b: Register {
            value: groups
                .name("regb")
                .expect("Failed to get reg b value")
                .as_str()
                .parse()
                .expect("Failed to convert reg b to num"),
        },
        regiseter_c: Register {
            value: groups
                .name("regc")
                .expect("Failed to get reg c value")
                .as_str()
                .parse()
                .expect("Failed to convert reg c to num"),
        },
        instructions: groups
            .name("p")
            .expect("Failed to find program")
            .as_str()
            .split(",")
            .map(|char| char.parse::<u8>().expect("Failed to parse program number"))
            .collect::<Vec<u8>>(),
        instruction_pointer: 0,
        halt: false,
        jump: false,
        output: vec![],
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let mut computer = parse(input);
    computer.process_instructions()
}

fn get_x(data: Vec<u8>, end: usize) -> Vec<u8> {
    data.to_owned().split_off(end)
}

// backtack algorithm to work out a, stolen from a couple of sources on reddit.
fn backtrack(computer: &mut Computer) -> u64 {
    let mut potential: Vec<u64> = vec![];
    // stack, so looping until we found something
    let mut stack: Vec<(usize, usize)> = vec![(0, computer.instructions.len() - 1)];
    computer.reset(); // reset computer for a clean slate.
    while let Some(info) = stack.pop() {
        let a = info.0;
        let distance = info.1;

        // each chunk of numbers is exactly the same and will only increase every 8.
        for chunk in 0..8 {
            let next_a = (a << 3) + chunk; // add the next number to the current value of a, shifted.
            computer.regiseter_a.value = next_a;
            let result = computer.test(); // run the computer with the proposed value
                                          // println!("result: {:?}", result);
            let target = get_x(computer.instructions.to_owned(), distance);
            // println!("Target: {:?}", target);

            // check if the computer result is what we are looking for
            if result != target {
                continue;
            }

            // if we are at the end of the line, store it
            if distance == 0 {
                potential.push(next_a as u64);
            } else {
                // this location could be a possible one for the whole number, so add it to the stack to loop again later.
                stack.push((next_a, distance - 1));
            }
        }
    }

    // println!("Pot: {:?}", potential);
    *potential.iter().min().expect("Failed to get minimum item")
}

#[aoc(day17, part2)]
fn part2(input: &str) -> u64 {
    let mut computer = parse(input);

    backtrack(&mut computer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_2), 117440);
    }
}
