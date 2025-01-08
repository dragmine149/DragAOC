use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavour: i64,
    texture: i64,
    calories: i64,
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<Ingredient> {
    let regex = Regex::new(r"capacity (?P<cap>-?\d), durability (?P<dur>-?\d), flavor (?P<fla>-?\d), texture (?P<tex>-?\d), calories (?P<cal>-?\d)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            Ingredient {
                capacity: captures
                    .name("cap")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse capacity"),
                durability: captures
                    .name("dur")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse durability"),
                flavour: captures
                    .name("fla")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse flavour"),
                texture: captures
                    .name("tex")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse texture"),
                calories: captures
                    .name("cal")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Failed to parse calories"),
            }
        })
        .collect_vec()
}

fn calculate_score(ingredients: &[Ingredient], amounts: &[u64], care_calories: bool) -> u64 {
    let capacity = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.capacity * amounts[i] as i64)
        .sum::<i64>();
    let durability = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.durability * amounts[i] as i64)
        .sum::<i64>();
    let flavour = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.flavour * amounts[i] as i64)
        .sum::<i64>();
    let texture = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.texture * amounts[i] as i64)
        .sum::<i64>();
    let calories = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.calories * amounts[i] as i64)
        .sum::<i64>();
    if capacity < 0 || durability < 0 || flavour < 0 || texture < 0 {
        return 0;
    }

    if care_calories && calories != 500 {
        return 0;
    }

    let total_c: i64 = capacity * durability * flavour * texture;
    if total_c < 0 {
        0
    } else {
        // println!("{:?}", total);
        total_c as u64
    }
}

fn better_increment(amounts: &mut [u64], position: usize) {
    amounts[position] += 1;
    // println!("{:?}", amounts);
    if amounts.iter().sum::<u64>() > 100 && position > 0 {
        // println!(">= 100!");
        amounts[position] = 0;
        for pi in position..(amounts.len() - 1) {
            amounts[pi] = 0;
        }
        better_increment(amounts, position - 1);
    }
}

#[aoc(day15, part1)]
fn part1(input: &[Ingredient]) -> u64 {
    let mut best = 0;
    let mut amounts: Vec<u64> = vec![0; input.len()];
    let last_pos = amounts.len() - 1;
    while amounts[0] != 99 {
        let score = calculate_score(input, &amounts, false);
        best = best.max(score);
        better_increment(&mut amounts, last_pos);
    }
    best
}

#[aoc(day15, part2)]
fn part2(input: &[Ingredient]) -> u64 {
    let mut best = 0;
    let mut amounts: Vec<u64> = vec![0; input.len()];
    let last_pos = amounts.len() - 1;
    while amounts[0] != 99 {
        let score = calculate_score(input, &amounts, true);
        best = best.max(score);
        better_increment(&mut amounts, last_pos);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 62842880);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 57600000);
    }
}
