use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// Probably a better way of doing this
#[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<(char, Vec<(u8, u8)>)>, (u8, u8)) {
    // Vector of [[Type, [(X, Y), (X, Y)]], [Type, [(X, Y), (X, Y)]]]
    let mut positions: Vec<(char, Vec<(u8, u8)>)> = vec![];

    for (line_index, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char == '.' || char == '#' {
                // don't need to worry about empty spaces.
                continue;
            }

            let pos = positions.iter().position(|x| x.0 == char);
            match pos {
                Some(idx) => {
                    positions[idx].1.push((line_index as u8, char_index as u8));
                }
                None => {
                    positions.push((char, vec![(line_index as u8, char_index as u8)]));
                }
            }
        }
    }

    (
        positions,
        (input.lines().count() as u8, input.lines().count() as u8),
    )
}

fn calculate_antinodes(
    frequency_locations: &Vec<(u8, u8)>,
    map_size: &(u8, u8),
    expand: bool,
) -> Vec<(u8, u8)> {
    let mut positions: Vec<(u8, u8)> = vec![];

    frequency_locations.iter().for_each(|pos| {
        frequency_locations.iter().for_each(|pos2| {
            if pos == pos2 {
                return;
            }

            println!("{:?} {:?}", pos, pos2);
            let distance = (pos2.0 as i8 - pos.0 as i8, pos2.1 as i8 - pos.1 as i8);
            let big_distance = (distance.0 * 2_i8, distance.1 * 2_i8);
            let goal = (pos2.0 as i8 - big_distance.0, pos2.1 as i8 - big_distance.1);
            println!("{:?}, {:?}, {:?}", distance, big_distance, goal);

            if goal.0 >= 0 && goal.1 >= 0 && goal.0 < map_size.0 as i8 && goal.1 < map_size.1 as i8
            {
                let big_u8 = (goal.0 as u8, goal.1 as u8);

                let has_pos = positions.contains(&big_u8);
                if !has_pos {
                    positions.push(big_u8);
                }
            }
        });
    });

    positions
}

fn build_grid(unique: &Vec<(u8, u8)>, map_size: &(u8, u8)) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; map_size.1 as usize]; map_size.0 as usize];
    for pos in unique {
        grid[pos.0 as usize][pos.1 as usize] = '#';
    }
    grid.iter()
        .map(|line| {
            let mut line_output = line.iter().collect::<String>();
            line_output.push('\n');
            line_output
        })
        .collect::<String>()
}

#[aoc(day8, part1)]
fn part1(input: &(Vec<(char, Vec<(u8, u8)>)>, (u8, u8))) -> u32 {
    let positions = &input.0;
    let map_size = &input.1;
    println!("Pos: {:#?}", positions);
    println!("Map: {:#?}", map_size);

    let mut unique: Vec<(u8, u8)> = vec![];
    for pos in positions.iter() {
        println!("Calculating antinodes for frequency: {:?}", &pos.0);
        let antinodes = calculate_antinodes(&pos.1, map_size, false);
        println!("Antinodes: {:#?}", antinodes);

        for antinode in antinodes.iter() {
            if !unique.contains(antinode) {
                unique.push(*antinode);
            }
        }
    }
    println!("Uniquenodes: {:#?}", unique);
    let map = build_grid(&unique, map_size);
    println!("{}", map);

    (unique.len()) as u32
}

#[aoc(day8, part2)]
fn part2(input: &(Vec<(char, Vec<(u8, u8)>)>, (u8, u8))) -> u32 {
    let positions = &input.0;
    let map_size = &input.1;
    println!("Pos: {:#?}", positions);
    println!("Map: {:#?}", map_size);

    let mut unique: Vec<(u8, u8)> = vec![];
    for pos in positions.iter() {
        println!("Calculating antinodes for frequency: {:?}", &pos.0);
        let antinodes = calculate_antinodes(&pos.1, map_size, true);
        println!("Antinodes: {:#?}", antinodes);

        for antinode in antinodes.iter() {
            if !unique.contains(antinode) {
                unique.push(*antinode);
            }
        }
    }
    println!("Uniquenodes: {:#?}", unique);
    let map = build_grid(&unique, map_size);
    println!("{}", map);

    (unique.len()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    const EXAMPLE_2: &str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 14);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 2);
    }

    const EXAMPLE_3: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 34);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_3)), 9);
    }
}
