use aoc_runner_derive::aoc;

fn process(input: &mut dyn Iterator<Item = String>) -> u64 {
    let mut sum_a: u64 = 0;
    let mut sum_b: u64 = 0;
    input
        .map(|line| {
            let code_count = line.len();
            // println!("{}", line);
            // println!("{:?}", code_count);

            let mut true_size = 0;
            let mut iter = line.chars();
            iter.next();
            while let Some(c) = iter.next() {
                if c == '\\' {
                    let x = iter.next();
                    // if hex, just keep skipping as hex although len 3 is technically just len 1
                    if x.unwrap() == 'x' {
                        // true_size += 3;
                        // true_size += 1;
                        iter.next();
                        iter.next();
                        // } else {
                        // true_size += 1;
                    }
                }
                true_size += 1;
            }
            true_size -= 1;

            // println!("{:?}", true_size);

            (code_count as u64, true_size)
        })
        .for_each(|line| {
            sum_a += line.0;
            sum_b += line.1;
        });

    sum_a - sum_b
}

#[aoc(day8, part1)]
fn part1(input: &str) -> u64 {
    process(
        &mut input
            .lines()
            .skip_while(|line| line.is_empty())
            .map(|line| line.to_string()),
    )
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u64 {
    process(
        &mut input
            .lines()
            .skip_while(|line| line.is_empty())
            .map(|line| format!("{:?}", line)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_1), 19);
    }
}
