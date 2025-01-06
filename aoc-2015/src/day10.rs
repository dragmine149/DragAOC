use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<u32> {
    // convert into a vector of numbers
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec()
}

fn process_line(line: Vec<u32>) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];
    let mut num: u32 = u32::MAX;
    let mut count: u32 = 0;

    // for each number
    for line_num in line.iter() {
        if *line_num != num {
            // if we have reached a different number
            if num != u32::MAX {
                // push to list if not the max (default) value
                numbers.push(count);
                numbers.push(num);
            }
            // reset
            count = 0;
            num = *line_num;
        }
        // add to count
        count += 1;
    }
    numbers.push(count);
    numbers.push(num);

    numbers
}

// repeat the above x times, used for testing.
fn process_x_lines(mut start_line: Vec<u32>, count: u16) -> String {
    for _ in 0..count {
        start_line = process_line(start_line);
    }

    // combine into a string
    start_line
        .iter()
        .fold(String::new(), |acc, x| acc + x.to_string().as_str())
}

#[aoc(day10, part1)]
fn part1(input: &[u32]) -> usize {
    process_x_lines(input.to_vec(), 40).len()
}

#[aoc(day10, part2)]
fn part2(input: &[u32]) -> usize {
    process_x_lines(input.to_vec(), 50).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(process_x_lines(vec![1], 1), "11");
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(process_x_lines(vec![1, 1], 1), "21");
    }
    #[test]
    fn part1_example_3() {
        assert_eq!(process_x_lines(vec![2, 1], 1), "1211");
    }
    #[test]
    fn part1_example_4() {
        assert_eq!(process_x_lines(vec![1, 2, 1, 1], 1), "111221");
    }
    #[test]
    fn part1_example_5() {
        assert_eq!(process_x_lines(vec![1, 1, 1, 2, 2, 1], 1), "312211");
    }
}
