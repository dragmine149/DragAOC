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

// calculate score
fn calculate_score(ingredients: &[Ingredient], amounts: &[u64], care_calories: bool) -> u64 {
    // calculate value of everything
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
    // 0 if any are negative
    if capacity < 0 || durability < 0 || flavour < 0 || texture < 0 {
        return 0;
    }

    // calories don't match, aka failed
    if care_calories && calories != 500 {
        return 0;
    }

    // total
    let total_c: i64 = capacity * durability * flavour * texture;
    if total_c < 0 {
        // and of cause, no negatives
        0
    } else {
        total_c as u64
    }
}

// incrementation system
fn better_increment(amounts: &mut [u64], position: usize) {
    amounts[position] += 1;

    // checks if the sum of all the values is more than 100, aka that combination won't do anything
    if amounts.iter().sum::<u64>() > 100 && position > 0 {
        // set itself and all the ones below it to 0, because no matter what we change here or below it will always be 100. Hence we can skip them
        amounts[position] = 1;
        for pi in position..(amounts.len() - 1) {
            amounts[pi] = 1;
        }

        // call itself to increment the next digit, and to keep going in terms of number counting.
        better_increment(amounts, position - 1);
    }
}

// calculate the best of the best
fn calculate(ingredients: &[Ingredient], care_calories: bool) -> u64 {
    // defaults and useable for other stuff
    let mut best = 0;
    // start at index of 1, because any index of 0 would just cause things to be 0 hence we skip those
    let mut amounts: Vec<u64> = vec![1; ingredients.len()];
    let last_pos = amounts.len() - 1;

    while amounts[0] != 99 {
        better_increment(&mut amounts, last_pos); // increment before stuff to save a tad bit of time (due to 0 start)
        let score = calculate_score(ingredients, &amounts, care_calories); // calculate
        best = best.max(score); // and max for the best
    }
    best
}

#[aoc(day15, part1)]
fn part1(input: &[Ingredient]) -> u64 {
    calculate(input, false)
}

#[aoc(day15, part2)]
fn part2(input: &[Ingredient]) -> u64 {
    calculate(input, true)
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
