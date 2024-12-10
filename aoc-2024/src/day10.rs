use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("Failed to parse to digit"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

// Find the next possible route by checking the neighbours to see if a node is a step above the current node.
fn find_next_nodes(grid: &Vec<Vec<u32>>, cell: &(usize, usize)) -> Vec<(usize, usize)> {
    let current_node = grid[cell.1][cell.0];
    let mut possible_routes: Vec<(usize, usize)> = vec![];
    if cell.1 >= 1 {
        if grid[cell.1 - 1][cell.0] == current_node + 1 {
            possible_routes.push((cell.0, cell.1 - 1));
        }
    }
    if cell.0 < grid.len() - 1 {
        if grid[cell.1][cell.0 + 1] == current_node + 1 {
            possible_routes.push((cell.0 + 1, cell.1));
        }
    }
    if cell.1 < grid.first().expect("Failed to get first row").len() - 1 {
        if grid[cell.1 + 1][cell.0] == current_node + 1 {
            possible_routes.push((cell.0, cell.1 + 1));
        }
    }
    if cell.0 >= 1 {
        if grid[cell.1][cell.0 - 1] == current_node + 1 {
            possible_routes.push((cell.0 - 1, cell.1));
        }
    }
    possible_routes
}

// function recursive. Calls itelf until it reaches the end, or no route.
fn get_pos_of_9(
    grid: &Vec<Vec<u32>>,
    cell: &(usize, usize),
    any_route: bool,
) -> Vec<(usize, usize)> {
    if grid[cell.1][cell.0] == 9 {
        // println!("Found: {:?}", cell);
        return vec![*cell];
    }
    let routes = find_next_nodes(grid, cell);
    if routes.is_empty() {
        return vec![]; // empty array as no path
    }
    let mut cells = vec![];
    for route in routes.iter() {
        let result_squares = get_pos_of_9(grid, route, any_route);
        for result in result_squares {
            if !cells.contains(&result) || any_route {
                cells.push(result);
            }
        }
    }
    cells
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u64 {
    let results = input
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .flat_map(move |(cell_index, cell)| {
                    if *cell != 0 {
                        return vec![];
                    }
                    // println!("Start: {:?}", (cell_index, row_index));
                    let a = get_pos_of_9(input, &(cell_index, row_index), false);
                    // println!("End: {:?}", a);
                    a
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();
    // println!("{:?}", results);
    results.len() as u64
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u64 {
    let results = input
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .flat_map(move |(cell_index, cell)| {
                    if *cell != 0 {
                        return vec![];
                    }
                    // println!("Start: {:?}", (cell_index, row_index));
                    let a = get_pos_of_9(input, &(cell_index, row_index), true);
                    // println!("End: {:?}", a);
                    a
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();
    // println!("{:?}", results);
    results.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 81);
    }
}
