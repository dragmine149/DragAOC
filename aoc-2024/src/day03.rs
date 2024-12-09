use aoc_runner_derive::aoc;
use regex::Regex;

fn parse_regex(input: &str, skip: bool) -> u64 {
    // Regex used: https://regex101.com/r/PHC6Mr/5
    let regex = Regex::new(
        r"(?P<multi>mul\((?P<num1>\d*),(?P<num2>\d*)\))|(?P<do>do\(\))|(?P<dont>don't\(\))",
    )
    .unwrap();

    // capture results and process them
    let result = regex.captures_iter(input);
    let mut calculation: u64 = 0;
    let mut is_skipping: bool = false;
    for mat in result {
        // println!("{:#?}", mat);

        // enable or disable if we are skipping over some
        if !mat.name("do").is_none() && skip {
            // println!("Stop Skipping");
            is_skipping = false;
            continue;
        }
        if !mat.name("dont").is_none() && skip {
            // println!("Started Skipping");
            is_skipping = true;
            continue;
        }

        if !skip && (!mat.name("do").is_none() || !mat.name("dont").is_none()) {
            continue;
        }

        if is_skipping {
            continue;
        }

        // calculate the result from the numbers provided by regex match.
        let num_1 = mat
            .name("num1")
            .expect("Failed to find first number in match")
            .as_str()
            .parse::<u64>()
            .expect("Failed to parse number from regex num1 match");
        let num_2 = mat
            .name("num2")
            .expect("Failed to find second number in match")
            .as_str()
            .parse::<u64>()
            .expect("Failed to parse number from regex num2 match");

        // println!("{:#?}", vec![num_1, num_2]);

        calculation += num_1 * num_2;
    }

    calculation
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    parse_regex(input, false)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    parse_regex(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_2), 48);
    }
}
