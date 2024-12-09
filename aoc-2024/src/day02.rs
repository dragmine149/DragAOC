use aoc_runner_derive::{aoc, aoc_generator};
// use aoc_runner_derive::aoc;
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut data: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let levels: Vec<u32> = line
            .trim()
            .split(" ")
            .map(|x| x.parse::<u32>().expect("Failed to convert to number"))
            .collect();
        data.push(levels);
    }

    data
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut safe_count = 0;

    for levels in input {
        // println!("----");

        // Pop the first one out of the list to make maths easier. (and so we don't work with negative values)
        let mut levels_iter = levels.iter().enumerate();
        levels_iter.next();
        let mut state: u8 = 0;
        let mut result = true;
        // for all the rest.
        for (index, level) in levels_iter {
            // println!("{}, {}, {}", index, level, state);

            let previous_level = levels
                .get(index - 1)
                .expect("Failed to get previous level index");

            // check if safe or not depending on the rest of them.
            let diff = safe_diff(previous_level, level, &state);
            if diff == 0 {
                result = false;
                break;
            }
            state = diff;
        }
        // println!("Pass: {}", result);
        safe_count += if result { 1 } else { 0 };
    }

    safe_count
}

// check if the level is safe or not depending on its relationship to the over levels.
fn safe_diff(input_a: &u32, input_b: &u32, state: &u8) -> u8 {
    let diff = compare_levels(input_a, input_b);
    if diff == 1 && state == &2 {
        return 0;
    }
    if diff == 2 && state == &1 {
        return 0;
    }
    diff
}

// Get the result of the difference in the two levels.
fn compare_levels(input_a: &u32, input_b: &u32) -> u8 {
    if input_a > &input_b.saturating_add(3) {
        return 0;
    }
    if input_a == input_b {
        return 0;
    }
    if input_a < &input_b.saturating_sub(3) {
        return 0;
    }
    let diff = if input_a > input_b { 1 } else { 2 };
    diff
}

fn p2_parse_level(input: &Vec<u32>, retry: bool) -> bool {
    // println!("{:?}", input);
    let mut iter = input.iter().enumerate();
    iter.next();

    // can use 0 as index because we skip 0 due to above line.
    let mut break_index: usize = 0;
    let mut state: u8 = 0;
    for (index, level) in iter {
        let previous_level = input
            .get(index - 1)
            .expect("Failed to get previous level index");

        let diff = safe_diff(previous_level, level, &state);
        if diff == 0 {
            break_index = index;
            break;
        }
        state = diff;
    }

    // println!("{}", break_index);

    // if something did break, retry again 3 times but removing either the start, one before or one after this one to see if it still breaks.
    if break_index != 0 {
        if retry {
            let mut new_input = input.clone();
            new_input.remove(break_index);
            let check_a = p2_parse_level(&new_input, false);

            let mut new_input_2 = input.clone();
            new_input_2.remove(break_index - 1);
            let check_b = p2_parse_level(&new_input_2, false);

            let mut new_input_3 = input.clone();
            new_input_3.remove(0);
            let check_c = p2_parse_level(&new_input_3, false);

            return check_a || check_b || check_c;
        }
        return false;
    }

    true
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut safe_count: u32 = 0;

    for levels in input {
        safe_count += if p2_parse_level(&levels, true) { 1 } else { 0 };
    }

    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 4);
    }
}
