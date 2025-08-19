use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for line in input.lines() {
        result.push(line[0..line.len() - 1].to_string())
    }
    result
}

fn numeric_keypad() -> HashMap<(i32, i32), Vec<char>> {
    HashMap::from([
        ((-1, 0), vec!['<', 'A']),
        ((-1, 1), vec!['^', '<', '<', 'A']),
        ((-1, 2), vec!['<', '^', 'A']),
        ((-1, 3), vec!['^', 'A']),
        ((-1, 4), vec!['^', '^', '<', '<', 'A']),
        ((-1, 5), vec!['<', '^', '^', 'A']),
        ((-1, 6), vec!['^', '^', 'A']),
        ((-1, 9), vec!['^', '^', '^', 'A']),
        ((0, 1), vec!['^', '<', 'A']),
        ((0, 2), vec!['^', 'A']),
        ((0, 3), vec!['^', '>', 'A']),
        ((0, 4), vec!['^', '^', '<', 'A']),
        ((0, 5), vec!['^', '^', 'A']),
        ((0, 6), vec!['^', '^', '>', 'A']),
        ((0, 7), vec!['^', '^', '^', '<', 'A']),
        ((0, 8), vec!['^', '^', '^', 'A']),
        ((0, 9), vec!['^', '^', '^', '>', 'A']),
        ((0, -1), vec!['>', 'A']),
        ((1, 0), vec!['>', 'v', 'A']),
        ((1, 2), vec!['>', 'A']),
        ((1, 3), vec!['>', '>', 'A']),
        ((1, 4), vec!['^', 'A']),
        ((1, 5), vec!['^', '>', 'A']),
        ((1, 6), vec!['^', '>', 'A']),
        ((1, 7), vec!['^', '^', 'A']),
        ((1, 8), vec!['^', '^', '>', 'A']),
        ((1, 9), vec!['^', '^', '>', '>', 'A']),
        ((1, -1), vec!['>', '>', 'v', 'A']),
        ((2, 9), vec!['^', '^', '>', 'A']),
        ((3, 1), vec!['<', '<', 'A']), //
        ((3, 7), vec!['<', '<', '^', '^', 'A']),
        ((3, 8), vec!['<', '^', '^', 'A']),       //
        ((4, -1), vec!['>', '>', 'v', 'v', 'A']), //
        ((4, 5), vec!['>', 'A']),
        ((5, 0), vec!['v', 'v', 'A']),
        ((5, 6), vec!['>', 'A']),
        ((6, -1), vec!['v', 'v', 'A']),
        ((6, 3), vec!['v', 'A']),                //
        ((6, 7), vec!['<', '<', '^', 'A']),      //
        ((7, 0), vec!['>', 'v', 'v', 'v', 'A']), //
        ((7, 4), vec!['v', 'A']),                //
        ((7, 9), vec!['>', '>', 'A']),
        ((8, -1), vec!['v', 'v', 'v', '>', 'A']), //
        ((8, 0), vec!['v', 'v', 'v', 'A']),
        ((9, -1), vec!['v', 'v', 'v', 'A']),
        ((9, 7), vec!['<', '<', 'A']), //
        ((9, 8), vec!['<', 'A']),
    ])
}

fn direction_keypad() -> HashMap<(char, char), Vec<char>> {
    HashMap::from([
        (('A', 'A'), vec!['A']),
        (('A', '^'), vec!['<', 'A']),
        (('A', '<'), vec!['v', '<', '<', 'A']),
        (('A', 'v'), vec!['<', 'v', 'A']),
        (('A', '>'), vec!['v', 'A']),
        (('^', '^'), vec!['A']),
        (('^', 'A'), vec!['>', 'A']),
        (('^', '<'), vec!['v', '<', 'A']),
        (('^', 'v'), vec!['v', 'A']),
        (('^', '>'), vec!['v', '>', 'A']),
        (('<', '<'), vec!['A']),
        (('<', 'A'), vec!['>', '>', '^', 'A']),
        (('<', '^'), vec!['>', '^', 'A']),
        (('<', 'v'), vec!['>', 'A']),
        (('<', '>'), vec!['>', '>', 'A']),
        (('v', 'v'), vec!['A']),
        (('v', 'A'), vec!['^', '>', 'A']),
        (('v', '^'), vec!['^', 'A']),
        (('v', '<'), vec!['<', 'A']),
        (('v', '>'), vec!['>', 'A']),
        (('>', '>'), vec!['A']),
        (('>', 'A'), vec!['^', 'A']),
        (('>', '^'), vec!['<', '^', 'A']),
        (('>', '<'), vec!['<', '<', 'A']),
        (('>', 'v'), vec!['<', 'A']),
    ])
}

fn calc_directional_buttons_length(
    buttons: &[char],
    to_go: u8,
    bot_cache: &mut HashMap<(char, char, u8), u64>,
) -> u64 {
    let keypad = direction_keypad();
    let mut length: u64 = 0;
    let mut start_pos = 'A';
    for button in buttons {
        if let Some(score) = bot_cache.get(&(start_pos, *button, to_go)) {
            // println!("cache hit {:?}", (start_pos, *button, to_go));
            length += score;
        } else {
            let key_vector = keypad.get(&(start_pos, *button)).unwrap();
            // println!("{} {} {} {:?}", to_go, start_pos, *button, key_vector);
            let score: u64 = if to_go == 0 {
                key_vector.len() as u64
            } else {
                calc_directional_buttons_length(key_vector, to_go - 1, bot_cache)
            };
            bot_cache.insert((start_pos, *button, to_go), score);
            length += score;
        }
        start_pos = *button;
    }
    length
}

fn calc_length(
    line: &str,
    initial_bots: u8,
    cache: &mut HashMap<(i32, i32), u64>,
    bot_cache: &mut HashMap<(char, char, u8), u64>,
) -> u64 {
    let keypad = numeric_keypad();
    let mut start_pos: i32 = -1;
    let mut length: u64 = 0;
    for char in line.chars() {
        let num: i32 = char.to_string().parse().unwrap();
        // println!("{} {}", start_pos, num);
        if let Some(score) = cache.get(&(start_pos, num)) {
            length += score;
        } else {
            let score = calc_directional_buttons_length(
                keypad.get(&(start_pos, num)).unwrap(),
                initial_bots - 1,
                bot_cache,
            );
            cache.insert((start_pos, num), score);
            length += score;
        }

        start_pos = num;
    }
    // println!("{} {}", start_pos, -1);
    length += calc_directional_buttons_length(
        keypad.get(&(start_pos, -1)).unwrap(),
        initial_bots - 1,
        bot_cache,
    );
    length
}

#[aoc(day21, part1)]
fn part1(input: &[String]) -> u64 {
    let mut cache: HashMap<(i32, i32), u64> = HashMap::new();
    let mut bot_cache: HashMap<(char, char, u8), u64> = HashMap::new();
    let mut result: u64 = 0;
    for line in input {
        // println!("line {}", line);
        let length = calc_length(line, 2, &mut cache, &mut bot_cache);
        let number: u64 = line.parse().unwrap();
        result += number * length;
        // println!("{}", length);
    }
    result
}

#[aoc(day21, part2)]
fn part2(input: &[String]) -> u64 {
    let mut cache: HashMap<(i32, i32), u64> = HashMap::new();
    let mut bot_cache: HashMap<(char, char, u8), u64> = HashMap::new();
    let mut result: u64 = 0;
    for line in input {
        // println!("line {}", line);
        let length = calc_length(line, 25, &mut cache, &mut bot_cache);
        let number: u64 = line.parse().unwrap();
        result += number * length;
        // println!("{}", length);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_example() {
        assert_eq!(parse("670A")[0], "670");

        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    // }
}
