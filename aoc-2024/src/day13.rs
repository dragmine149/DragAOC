use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Position(u64, u64);

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            button_a: Position(0, 0),
            button_b: Position(0, 0),
            prize: Position(0, 0),
        }
    }
}

impl Machine {
    fn new() -> Self {
        Default::default()
    }

    fn simultaneous(&self) -> (u8, u8) {
        // THANKS: https://www.reddit.com/r/adventofcode/comments/1hde7e4/you_dont_have_to_use_linear_algebra_you_can_use/
        let a = (self.button_a.0 as f64, self.button_a.1 as f64);
        let b = (self.button_b.0 as f64, self.button_b.1 as f64);
        let p = (self.prize.0 as f64, self.prize.1 as f64);

        let x = ((p.1 * b.0) - (p.0 * b.1)) / ((a.1 * b.0) - (a.0 * b.1));
        let y = ((-(a.0) * x) + p.0) / b.0;

        println!("X: {:?}, Y: {:?}", x, y);
        if x >= 0.0 && x <= 100.0 && y >= 0.0 && y <= 100.0 && x.fract() == 0.0 && y.fract() == 0.0
        {
            return (x as u8, y as u8);
        }

        (101, 101)
    }

    fn machine_cost(&self) -> u64 {
        println!(
            "Attempting to do simulataneous equations on machine: {:?}",
            self
        );
        let (a_count, b_count) = self.simultaneous();
        println!("Result of sim: (X: {:?}, y: {:?})", a_count, b_count);
        if a_count > 100 || b_count > 100 {
            return 0_u64;
        }

        a_count as u64 * 3 + b_count as u64
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];
    let mut machine: Machine = Machine::new();

    // https://regex101.com/r/5w0bA7/1
    let regex = Regex::new(
        r"((?P<ButtonA>Button A: X\+(?P<ax>\d*), Y\+(?P<ay>\d*)))|((?P<ButtonB>Button B: X\+(?P<bx>\d*), Y\+(?P<by>\d*)))|((?P<Prize>Prize: X=(?P<px>\d*), Y=(?P<py>\d*)))|(?P<empty>\n\n)",
    ).unwrap();
    let groups = regex.captures_iter(input);
    for mat in groups {
        if mat.name("ax").is_some() {
            machine.button_a.0 = mat
                .name("ax")
                .expect("Failed to find ax")
                .as_str()
                .parse()
                .expect("Failed to parse ax");
        }
        if mat.name("ay").is_some() {
            machine.button_a.1 = mat
                .name("ay")
                .expect("Failed to find ay")
                .as_str()
                .parse()
                .expect("Failed to parse ay");
        }
        if mat.name("bx").is_some() {
            machine.button_b.0 = mat
                .name("bx")
                .expect("Failed to find bx")
                .as_str()
                .parse()
                .expect("Failed to parse bx");
        }
        if mat.name("by").is_some() {
            machine.button_b.1 = mat
                .name("by")
                .expect("Failed to find by")
                .as_str()
                .parse()
                .expect("Failed to parse by");
        }
        if mat.name("px").is_some() {
            machine.prize.0 = mat
                .name("px")
                .expect("Failed to find px")
                .as_str()
                .parse()
                .expect("Failed to parse px");
        }
        if mat.name("py").is_some() {
            machine.prize.1 = mat
                .name("py")
                .expect("Failed to find py")
                .as_str()
                .parse()
                .expect("Failed to parse py");
        }
        if mat.name("empty").is_some() {
            machines.push(machine);
            machine = Machine::new();
        }
    }

    machines.push(machine);
    machines
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> u64 {
    println!("{:#?}", input);

    input
        .into_par_iter()
        // .iter()
        .map(|machine| machine.machine_cost())
        .sum()
}

// #[aoc(day13, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    const SIM_TEST: &str = "Button A: X+1, Y+4
Button B: X+2, Y+5
Prize: X=3, Y=12
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 480);
    }

    #[test]
    fn sim_test() {
        assert_eq!(part1(&parse(SIM_TEST)), 9);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
