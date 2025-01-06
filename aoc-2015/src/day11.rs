use aoc_runner_derive::aoc;

// Returns true if either "i", "o" or "u" exists.
fn check_iou(input: &[u8]) -> bool {
    input.contains(&105) || input.contains(&108) || input.contains(&111)
}

// Returns true if input contains a double (e.g. "aa", "bb", etc)
fn check_double(input: &[u8]) -> bool {
    let mut double = 0;
    let mut used: Vec<u8> = vec![];
    input.iter().enumerate().for_each(|(i, c)| {
        if i + 1 >= input.len() {
            // break out to avoid refrencing issues
            return;
        }
        if used.contains(c) {
            return;
        }
        if *c == input[i + 1] {
            double += 1;
            used.push(*c);
        }
    });

    double >= 2
}

fn check_run(input: &[u8]) -> bool {
    let mut run = false;
    input.iter().enumerate().for_each(|(i, c)| {
        if i + 1 >= input.len() || i + 2 >= input.len() {
            return;
        }
        if *c + 1 == input[i + 1] && *c + 2 == input[i + 2] {
            run = true;
        }
    });

    run
}

fn valid_password(input: &[u8]) -> bool {
    !check_iou(input) && check_double(input) && check_run(input)
}

#[allow(dead_code)]
fn overflow(input: &mut [u8], position: usize) {
    let mut s = input[position] + 1;
    if s >= 123 {
        s = 97;
        overflow(input, position - 1);
    }
    input[position] = s;
}

fn overflow_exclusion(input: &mut [u8], position: usize) {
    let mut s = input[position] + 1;
    // check to see if not `i`, `o`, `l` and just skip all of them.
    // Desgined to speed up a tad by not having to check all of the invalid ones.
    if s == 105 || s == 108 || s == 111 {
        s += 1;
        for i in position..(input.len() - 1) {
            input[i] = 97;
        }
    }

    if s >= 123 {
        s = 97;
        overflow_exclusion(input, position - 1);
    }
    input[position] = s;
}

fn next_password(input: &str) -> String {
    let mut password = input.as_bytes().to_owned();
    let end = password.len() - 1;
    overflow_exclusion(&mut password, end);
    while !valid_password(&password) {
        overflow_exclusion(&mut password, end);
    }

    String::from_utf8(password.to_vec()).expect("e")
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    next_password(input)
}

// #[aoc(day11, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(next_password(&"abcdefgh"), "abcdffaa");
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
