use aoc_runner_derive::aoc;

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
            // checks if we are in range
            return;
        }
        // counts the run
        if *c + 1 == input[i + 1] && *c + 2 == input[i + 2] {
            run = true;
        }
    });

    run
}

fn valid_password(input: &[u8]) -> bool {
    check_double(input) && check_run(input)
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

    // normall overflow
    if s >= 123 {
        s = 97;
        overflow_exclusion(input, position - 1);
    }
    input[position] = s;
}

// keep looping until the next password has been generated
fn next_password(password: &mut [u8]) {
    let end = password.len() - 1;
    overflow_exclusion(password, end);
    while !valid_password(password) {
        overflow_exclusion(password, end);
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    let mut password = input.as_bytes().to_owned();
    next_password(&mut password);
    String::from_utf8(password).expect("e")
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    let mut password = input.as_bytes().to_owned();
    next_password(&mut password);
    next_password(&mut password);
    String::from_utf8(password).expect("e")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&"abcdefgh"), "abcdffaa");
    }
}
