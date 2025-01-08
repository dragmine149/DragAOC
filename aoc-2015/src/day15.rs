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
    let total = Ingredient {
        capacity: ingredients
            .iter()
            .enumerate()
            .map(|(i, v)| v.capacity * amounts[i] as i64)
            .sum(),
        durability: ingredients
            .iter()
            .enumerate()
            .map(|(i, v)| v.durability * amounts[i] as i64)
            .sum(),
        flavour: ingredients
            .iter()
            .enumerate()
            .map(|(i, v)| v.flavour * amounts[i] as i64)
            .sum(),
        texture: ingredients
            .iter()
            .enumerate()
            .map(|(i, v)| v.texture * amounts[i] as i64)
            .sum(),
        calories: ingredients
            .iter()
            .enumerate()
            .map(|(i, v)| v.calories * amounts[i] as i64)
            .sum(),
    };
    if total.capacity < 0 || total.durability < 0 || total.flavour < 0 || total.texture < 0 {
        return 0;
    }

    if care_calories && total.calories != 500 {
        return 0;
    }

    let total_c = total.capacity * total.durability * total.flavour * total.texture;
    if total_c < 0 {
        0
    } else {
        // println!("{:?}", total);
        total_c as u64
    }
}

fn increment_amount(amounts: &mut Vec<u64>, position: usize) {
    amounts[position] += 1;
    if amounts[position] >= 100 {
        amounts[position] = 0;
        if position > 0 {
            increment_amount(amounts, position - 1);
        }
    }
}

#[aoc(day15, part1)]
fn part1(input: &[Ingredient]) -> u64 {
    let mut best = 0;
    let mut amounts: Vec<u64> = vec![0; input.len()];
    let last_pos = amounts.len() - 1;
    while !amounts.iter().all(|i| *i == 99) {
        let score = if amounts.iter().sum::<u64>() == 100 {
            // println!("{:?}", amounts.iter().sum::<u64>());
            calculate_score(input, &amounts, false)
        } else {
            0
        };

        // if score > 0 {
        //     println!("{:?} gives {:?}", amounts, score);
        // }

        best = best.max(score);

        // if score > best {
        //     println!(
        //         "NEW BEST! Amounts of: {:?} gives score of {:?}",
        //         amounts, score
        //     );
        //     best = score;
        // }
        increment_amount(&mut amounts, last_pos);
    }
    best
}

#[aoc(day15, part2)]
fn part2(input: &[Ingredient]) -> u64 {
    let mut best = 0;
    let mut amounts: Vec<u64> = vec![0; input.len()];
    let last_pos = amounts.len() - 1;
    while !amounts.iter().all(|i| *i == 99) {
        let score = if amounts.iter().sum::<u64>() == 100 {
            calculate_score(input, &amounts, true)
        } else {
            0
        };

        best = best.max(score);
        increment_amount(&mut amounts, last_pos);
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
