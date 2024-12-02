// use aoc_runner_derive::{aoc, aoc_generator};
use aoc_runner_derive::aoc;
// #[aoc_generator(day2)]
// fn parse(input: &str) -> String {
//     todo!()
// }

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let mut safe_count = 0;

    for line in input.lines() {
        println!("----");
        if line.is_empty() {
            continue;
        }

        let levels: Vec<u32> = line
            .trim()
            .split(" ")
            .map(|x| x.parse::<u32>().expect("Failed to convert to number"))
            .collect();
        println!("{:?}", levels);

        let mut levels_iter = levels.iter().enumerate();
        levels_iter.next();
        let mut state: u8 = 0;
        let mut result = true;
        for (index, level) in levels_iter {
            println!("{}, {}, {}", index, level, state);

            let previous_level = levels
                .get(index - 1)
                .expect("Failed to get previous level index");

            if level == previous_level {
                result = false;
                break;
            }

            if level > previous_level && *level <= previous_level + 3_u32 {
                if state == 2 {
                    result = false;
                    break;
                }

                state = 1;
                continue;
            }

            if level < previous_level && *level >= previous_level.saturating_sub(3) {
                if state == 1 {
                    result = false;
                    break;
                }
                state = 2;
                continue;
            }

            result = false;
            break;
        }
        println!("Pass: {}", result);
        safe_count += if result { 1 } else { 0 };
    }

    safe_count
}

// #[aoc(day2, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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
        assert_eq!(part1(EXAMPLE_1), 2);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
