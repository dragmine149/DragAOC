use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> (Vec<Vec<u8>>, (u8, u8)) {
    let mut guard = (0_u8, 0_u8);

    for (index, line) in input.lines().enumerate() {
        let guard_potential_pos = line.find('^');

        match guard_potential_pos {
            Some(pos) => {
                guard = (index as u8, pos as u8);
            }
            None => {
                continue;
            }
        }
    }

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    if char == '#' {
                        return 1;
                    }
                    if char == '^' {
                        return 2;
                    }
                    0
                })
                .collect()
        })
        .collect();
    (map, guard)
}

fn translate_direction(pos: (u8, u8), direction: u8) -> (u8, u8, bool) {
    if direction == 2 {
        let out_of_map = pos.0 < 1;
        return (pos.0.saturating_sub(1), pos.1, out_of_map);
    }
    if direction == 3 {
        return (pos.0, pos.1.saturating_add(1), false);
    }
    if direction == 4 {
        return (pos.0.saturating_add(1), pos.1, false);
    }
    let out_of_map = pos.1 < 1;
    (pos.0, pos.1.saturating_sub(1), out_of_map)
}

fn move_guard(
    mut map: Vec<Vec<u8>>,
    pos: (u8, u8),
    direction: u8,
) -> (Vec<Vec<u8>>, u16, bool, (u8, u8)) {
    let mut move_count: u16 = 0;
    let mut direction_info = pos.clone();
    loop {
        let next_dir_info = translate_direction(direction_info, direction);
        if next_dir_info.0 >= map.len() as u8
            || next_dir_info.1 >= map[0].len() as u8
            || next_dir_info.2
        {
            return (map, move_count + 1, true, direction_info);
        }

        // println!(
        //     "Currently checking cell: {:?}",
        //     (next_dir_info.0, next_dir_info.1)
        // );

        let square = &map[next_dir_info.0 as usize][next_dir_info.1 as usize];

        if square == &0 {
            // new square, mark it
            move_count += 1;
            // move guard
            map[next_dir_info.0 as usize][next_dir_info.1 as usize] = direction;
            map[direction_info.0 as usize][direction_info.1 as usize] = 6;

            direction_info = (next_dir_info.0, next_dir_info.1);
            continue;
        }
        if square == &1 {
            // the square is an object
            break;
        }
        if square >= &2 && square <= &5 {
            // another guard somehow
            panic!("Why is there another guard around?");
        }
        if square == &6 {
            // the square has already been visited

            map[next_dir_info.0 as usize][next_dir_info.1 as usize] = direction;
            map[direction_info.0 as usize][direction_info.1 as usize] = 6;

            direction_info = (next_dir_info.0, next_dir_info.1);
            continue;
        }
        panic!("Unknown square!");
    }

    (map, move_count, false, direction_info)
}

fn output_map(map: &Vec<Vec<u8>>) {
    let output_map = map
        .iter()
        .map(|line| {
            let mut visible_line = line
                .iter()
                .map(|pos| {
                    if pos == &0 {
                        '.'
                    } else if pos == &1 {
                        '#'
                    } else if pos == &2 {
                        '^'
                    } else if pos == &3 {
                        '>'
                    } else if pos == &4 {
                        'v'
                    } else if pos == &5 {
                        '<'
                    } else if pos == &6 {
                        'X'
                    } else {
                        '?'
                    }
                })
                .collect::<String>();
            visible_line.push('\n');
            visible_line
        })
        .collect::<String>();
    println!("{}", output_map);
}

#[aoc(day6, part1)]
fn part1(input: &(Vec<Vec<u8>>, (u8, u8))) -> u16 {
    let mut is_out_map = false;
    let mut map = input.0.clone();
    let mut total_moves = 0;
    let mut guard_direction = 2;
    let mut pos = input.1;

    while !is_out_map {
        // output_map(&map);
        let move_count;
        (map, move_count, is_out_map, pos) = move_guard(map, pos, guard_direction);
        // println!(
        //     "Guard moved {} and is {} in map",
        //     move_count,
        //     if is_out_map { "not" } else { "still" }
        // );
        total_moves += move_count;

        if !is_out_map {
            guard_direction += 1;
            if guard_direction > 6 {
                guard_direction = 2;
            }
        }

        // output_map(&map);
    }
    output_map(&map);

    total_moves
}
// #[aoc(day6, part2)]
// fn part2(input: &(Vec<Vec<u8>>, Vec<Vec<u8>>)) -> u16 {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 6);
    }
}
