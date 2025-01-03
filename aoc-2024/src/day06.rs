use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> (Vec<Vec<u8>>, (u8, u8)) {
    let mut guard = (0_u8, 0_u8);

    // get the location of the guard
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

    // translate the map into a grid of numbers.
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => 1,
                    '^' => 2,
                    _ => 0,
                })
                .collect()
        })
        .collect();
    (map, guard)
}

// Returns the next position to go to and if that is inside or outside of the map.
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

// Move the guard around the map
// Takes: map, pos, direction
// Returns:
// - map: The map
// - move count: How many cells the guard moved this time
// - Is out of map: Is guard inside or outside of the border
// - Position: Position of the guard (or last position guard was in before going outside)
fn move_guard(
    mut map: Vec<Vec<u8>>,
    pos: (u8, u8),
    direction: u8,
) -> (Vec<Vec<u8>>, u16, bool, (u8, u8)) {
    let mut move_count: u16 = 0;
    let mut direction_info = pos.to_owned();
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
            // move guard by setting the next square to the guard and the previous square to an x.
            // This also accidently prevents an issue in p2
            map[next_dir_info.0 as usize][next_dir_info.1 as usize] = direction;
            map[direction_info.0 as usize][direction_info.1 as usize] = 6;

            direction_info = (next_dir_info.0, next_dir_info.1);
            continue;
        }
        if square == &1 || square == &7 {
            // the square is an object
            break;
        }
        if (&2..&5).contains(&square) {
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

// Normal guard movement
fn move_and_rotate_guard(input: &(Vec<Vec<u8>>, (u8, u8))) -> u16 {
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
            // overflow rotation system
            guard_direction += 1;
            if guard_direction >= 6 {
                guard_direction = 2;
            }
        }

        // output_map(&map);
    }
    // output_map(&map);

    total_moves
}

// Try to block the guard by doing the same as the above but slightly different.
fn move_and_block_guard(input: &(Vec<Vec<u8>>, (u8, u8)), obstruction_pos: (u8, u8)) -> bool {
    let mut map = input.0.clone();
    map[obstruction_pos.0 as usize][obstruction_pos.1 as usize] = 7; // the square to block

    let mut guard_direction = 2;
    let mut pos = input.1;
    let mut is_out_map = false;

    let mut visited: Vec<(u8, u8)> = vec![pos]; // list of positions

    while !is_out_map {
        (map, _, is_out_map, pos) = move_guard(map, pos, guard_direction);

        // If the guard has been here before break. (Pos is only when the guard rotates, and since we're back here we are 99% likely to be in a loop)
        // The `visited.last().expect("Failed to find last elm") != &pos` check prevents the guard from giving up if it has just rotated and already got stuck without moving.
        if visited.contains(&pos) && visited.last().expect("Failed to find last elm") != &pos {
            // output_map(&map);
            // println!("{:?}\t{:?}", visited, pos);
            // println!("Obstruction worked!");
            return true;
        }
        visited.push(pos);

        // println!("{:?}\t{:?}\t{:?}", visited, pos, is_out_map);
        if !is_out_map {
            // println!("{}", guard_direction);
            guard_direction += 1;
            if guard_direction >= 6 {
                guard_direction = 2;
            }
            // println!("{}", guard_direction);
        }
    }
    // output_map(&map);
    // println!("Obstruction failed");
    false
}

// Debug, convert and print map.
#[allow(dead_code)]
fn output_map(map: &[Vec<u8>]) {
    let output_map = map
        .iter()
        .map(|line| {
            let mut visible_line = line
                .iter()
                .map(|pos| match *pos {
                    0 => '.',
                    1 => '#',
                    2 => '^',
                    3 => '>',
                    4 => 'v',
                    5 => '<',
                    6 => 'X',
                    7 => 'O',
                    _ => '?',
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
    move_and_rotate_guard(input)
}
#[aoc(day6, part2)]
fn part2(input: &(Vec<Vec<u8>>, (u8, u8))) -> u16 {
    let mut obstruction_count = 0;
    for (line_index, line) in input.0.iter().enumerate() {
        for (pos_index, _) in line.iter().enumerate() {
            // println!("Adding obstruction at {:?}", (line_index, pos_index));
            if input.0[line_index][pos_index] == 1 {
                // println!("Canceled! Obstruction already at place.");
                continue;
            }

            let looped = move_and_block_guard(input, (line_index as u8, pos_index as u8));
            obstruction_count += if looped { 1 } else { 0 }
        }
    }

    obstruction_count
}

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
