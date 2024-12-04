use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            let a: Vec<String> = line.chars().map(|char| char.to_string()).collect();
            a
        })
        .collect()
}

fn check_for_xmas(
    input: &Vec<Vec<String>>,
    start_line_index: isize,
    start_char_index: isize,
    line_length: usize,
) -> u64 {
    let mut count: u64 = 0;

    // north
    if start_line_index - 3 >= 0 {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;

        if input[sli - 1][sci] == "M" && input[sli - 2][sci] == "A" && input[sli - 3][sci] == "S" {
            count += 1;
        }
    }
    // north east
    if start_line_index - 3 >= 0 && start_char_index + 3 < line_length as isize {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;

        if input[sli - 1][sci + 1] == "M"
            && input[sli - 2][sci + 2] == "A"
            && input[sli - 3][sci + 3] == "S"
        {
            count += 1;
        }
    }
    // east
    if start_char_index + 3 < line_length as isize {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;

        if input[sli][sci + 1] == "M" && input[sli][sci + 2] == "A" && input[sli][sci + 3] == "S" {
            count += 1;
        }
    }
    // south east
    if start_line_index + 3 < input.len() as isize && start_char_index + 3 < line_length as isize {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;

        if input[sli + 1][sci + 1] == "M"
            && input[sli + 2][sci + 2] == "A"
            && input[sli + 3][sci + 3] == "S"
        {
            count += 1;
        }
    }
    // south
    if start_line_index + 3 < input.len() as isize {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;
        if input[sli + 1][sci] == "M" && input[sli + 2][sci] == "A" && input[sli + 3][sci] == "S" {
            count += 1;
        }
    }
    // south west
    if start_line_index + 3 < input.len() as isize && start_char_index - 3 >= 0 {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;

        if input[sli + 1][sci - 1] == "M"
            && input[sli + 2][sci - 2] == "A"
            && input[sli + 3][sci - 3] == "S"
        {
            count += 1;
        }
    }
    // west
    if start_char_index - 3 >= 0 {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;
        if input[sli][sci - 1] == "M" && input[sli][sci - 2] == "A" && input[sli][sci - 3] == "S" {
            count += 1;
        }
    }
    // north west
    if start_line_index - 3 >= 0 && start_char_index - 3 >= 0 {
        let sli = start_line_index as usize;
        let sci = start_char_index as usize;

        if input[sli - 1][sci - 1] == "M"
            && input[sli - 2][sci - 2] == "A"
            && input[sli - 3][sci - 3] == "S"
        {
            count += 1;
        }
    }

    count
}

fn find_as_from_mas(
    input: &Vec<Vec<String>>,
    index_a: (isize, isize),
    index_b: (isize, isize),
    line_length: isize,
) -> u64 {
    let mut count = 0;
    // top bottom
    if index_a.0 == index_b.0 {
        if index_a.0 - 2 >= 0 {
            let mid = input[index_a.0 as usize - 1][index_b.1 as usize - 1] == "A";
            let top_left = input[index_a.0 as usize - 2][index_a.1 as usize] == "S";
            let top_right = input[index_b.0 as usize - 2][index_b.1 as usize] == "S";

            if mid && top_left && top_right {
                count += 1;
            }
        }
        if index_a.0 + 2 < input.len() as isize {
            let mid = input[index_a.0 as usize + 1][index_b.1 as usize - 1] == "A";
            let bottom_left = input[index_a.0 as usize + 2][index_a.1 as usize] == "S";
            let bottom_right = input[index_b.0 as usize + 2][index_b.1 as usize] == "S";

            if mid && bottom_left && bottom_right {
                count += 1;
            }
        }

        return count;
    }

    // left right
    if index_a.1 == index_b.1 {
        if index_a.1 - 2 >= 0 {
            let mid = input[index_a.0 as usize + 1][index_b.1 as usize - 1] == "A";
            let left_top = input[index_a.0 as usize][index_a.1 as usize - 2] == "S";
            let left_bottom = input[index_b.0 as usize][index_b.1 as usize - 2] == "S";

            if mid && left_top && left_bottom {
                count += 1;
            }
        }
        if index_a.1 + 2 < line_length {
            let mid = input[index_a.0 as usize + 1][index_b.1 as usize + 1] == "A";
            let right_top = input[index_a.0 as usize][index_a.1 as usize + 2] == "S";
            let right_bottom = input[index_b.0 as usize][index_b.1 as usize + 2] == "S";

            if mid && right_top && right_bottom {
                count += 1;
            }
        }
    }

    count
}

fn check_for_x_mas(
    input: &Vec<Vec<String>>,
    start_line_index: usize,
    start_char_index: usize,
    line_length: usize,
) -> u64 {
    let mut count: u64 = 0;

    if start_char_index + 2 < line_length {
        if input[start_line_index][start_char_index + 2] == "M" {
            count += find_as_from_mas(
                input,
                (start_line_index as isize, start_char_index as isize),
                (start_line_index as isize, start_char_index as isize + 2),
                line_length as isize,
            );
        }
    }

    if start_line_index + 2 < input.len() {
        if input[start_line_index + 2][start_char_index] == "M" {
            count += find_as_from_mas(
                input,
                (start_line_index as isize, start_char_index as isize),
                (start_line_index as isize + 2, start_char_index as isize),
                line_length as isize,
            );
        }
    }

    count
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<String>>) -> u64 {
    let mut xmas_found: u64 = 0;

    for (line_index, line) in input.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if char == "X" {
                let xmas_count =
                    check_for_xmas(input, line_index as isize, char_index as isize, line.len());
                xmas_found += xmas_count;
                // println!(
                //     "Xmas found at: ({}, {}): {} (total: {})",
                //     line_index, char_index, xmas_count, xmas_found
                // );
                // print!("{}", xmas_count);
                // continue;
            }
            // print!(".");
        }
        // println!();
    }

    xmas_found
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Vec<String>>) -> u64 {
    let mut xmas_found: u64 = 0;

    for (line_index, line) in input.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if char == "M" {
                let xmas_count = check_for_x_mas(input, line_index, char_index, line.len());
                xmas_found += xmas_count;
            }
        }
    }

    xmas_found
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 9);
    }
}
