use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::Ordering;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let mut rules: Vec<Vec<u16>> = vec![];
    let mut update: Vec<Vec<u16>> = vec![];
    let mut update_rules = true;

    // split the rules and pages into two seperate vectors
    input.lines().for_each(|line| {
        if line.is_empty() {
            update_rules = false;
            return;
        }

        if update_rules {
            rules.push(
                line.split('|')
                    .map(|x| {
                        x.parse::<u16>()
                            .expect("Failed to parse number in rules input")
                    })
                    .collect::<Vec<u16>>(),
            );
        } else {
            update.push(
                line.split(',')
                    .map(|x| {
                        x.parse::<u16>()
                            .expect("Failed to parse number in update input")
                    })
                    .collect::<Vec<u16>>(),
            );
        }
    });

    rules.sort_by(|a, b| {
        (a.first().expect("Failed to get first element"))
            .cmp(b.first().expect("Failed to get element"))
    });
    (rules, update)
}

fn check_rules(rules: &[Vec<u16>], num_a: u16, numbers: &[u16]) -> bool {
    rules
        .iter()
        .filter(|x| {
            // get the rule based off the num input.
            let rule_first = x.first().expect("Failed to get first number of rule");
            let rule_last = x.get(1).expect("Failed to get second number of rule");

            (rule_first == &num_a && numbers.contains(rule_last))
                || (rule_last == &num_a && numbers.contains(rule_first))
        })
        // .collect::<Vec<Vec<u16>>>();
        // println!("Found rules: {:#?}", rule_filter);
        .all(|rule| {
            // checks to see if the rule is valid.
            let rule_first = rule.first().expect("Failed to get first number of rule");
            let rule_last = rule.get(1).expect("Failed to get second number of rule");
            let rule_first_index = numbers.iter().position(|x| x == rule_first);
            let rule_last_index = numbers.iter().position(|x| x == rule_last);

            // println!(
            //     "Checking rule: {:?} (first: {:?}, last: {:?})",
            //     rule, rule_first_index, rule_last_index
            // );

            if rule_first == &num_a && rule_first_index > rule_last_index {
                return false;
            }
            if rule_last == &num_a && rule_last_index < rule_first_index {
                return false;
            }

            true
        })
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<Vec<u16>>, Vec<Vec<u16>>)) -> u16 {
    // println!("{:#?}", input.0);
    // println!("{:#?}", input.1);
    let rules = &input.0;
    let updates = &input.1;

    updates
        .par_iter()
        .map(|update| {
            // println!("{:?}", update);

            for rule in update.iter() {
                let result = check_rules(rules, *rule, update);
                if !result {
                    return 0;
                }
            }

            *update.get(update.len() / 2).unwrap_or(&0)
        })
        .sum()
}

// get a list of the rules that are used with that number
fn get_desired_rules<'a>(
    rules: &'a [Vec<u16>],
    num_a: u16,
    numbers: &'a [u16],
) -> impl IntoIterator<Item = &'a Vec<u16>> + 'a {
    rules.iter().filter(move |x| {
        let rule_first = x.first().expect("Failed to get first number of rule");
        let rule_last = x.get(1).expect("Failed to get second number of rule");

        (rule_first == &num_a && numbers.contains(rule_last))
            || (rule_last == &num_a && numbers.contains(rule_first))
    })
}

fn fix_rules(rules: &[Vec<u16>], numbers: &[u16]) -> Vec<u16> {
    // println!("Fixing order: {:?}", numbers);
    let mut update_order = numbers.to_owned();
    update_order.sort_by(|a, b| {
        // println!("(a: {:?}, b: {:?})", a, b);
        let desired_rules = get_desired_rules(rules, *b, numbers);
        // println!("{:?}", desired_rules);

        // change the ordering depending on rule order
        for rule in desired_rules {
            let rule_first = rule.first().expect("Failed to get first number of rule");
            let rule_last = rule.get(1).expect("Failed to get second number of rule");

            if rule_first == a {
                return Ordering::Less;
            }

            if rule_last == a {
                return Ordering::Greater;
            }
        }

        // if we have no rule or can't determin ordering, just keep it where it is
        Ordering::Equal
    });

    update_order
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<Vec<u16>>, Vec<Vec<u16>>)) -> u16 {
    // println!("{:#?}", input.0);
    // println!("{:#?}", input.1);
    let rules = &input.0;
    let updates = &input.1;

    updates
        .par_iter()
        .map(|update| {
            for rule in update.iter() {
                let result = check_rules(rules, *rule, update);
                if !result {
                    let new_set = fix_rules(rules, update);
                    return *new_set.get(new_set.len() / 2).unwrap_or(&0);
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 123);
    }
}
