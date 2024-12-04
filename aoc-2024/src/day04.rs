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

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<String>>) -> u64 {
    let mut xmas_found: u64 = 0;

    // println!("{:#?}", xmas_found - 3 >= 0);

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
                print!("{}", xmas_count);
                continue;
            }
            print!(".");
        }
        println!();
    }

    xmas_found
}

// #[aoc(day4, part2)]
// fn part2(input: &str) -> u64 {
//     todo!();
// }

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

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(EXAMPLE_2), 48);
    // }
}
