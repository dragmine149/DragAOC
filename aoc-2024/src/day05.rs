use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut rules: Vec<Vec<u8>> = vec![];
    let mut update: Vec<Vec<u8>> = vec![];
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
                        x.parse::<u8>()
                            .expect("Failed to parse number in rules input")
                    })
                    .collect::<Vec<u8>>(),
            );
        } else {
            update.push(
                line.split(',')
                    .map(|x| {
                        x.parse::<u8>()
                            .expect("Failed to parse number in update input")
                    })
                    .collect::<Vec<u8>>(),
            );
        }
    });

    rules.sort_by(|a, b| {
        (a.first().expect("Failed to get first element"))
            .cmp(b.first().expect("Failed to get element"))
    });
    (rules, update)
}

fn check_rules(rules: &[Vec<u8>], num_a: u8, numbers: &[u8]) -> bool {
    let rule_filter = rules
        .iter()
        .filter(|x| {
            // get the rule based off the num input.
            let rule_first = x.first().expect("Failed to get first number of rule");
            let rule_last = x.get(1).expect("Failed to get second number of rule");

            (rule_first == &num_a && numbers.contains(rule_last))
                || (rule_last == &num_a && numbers.contains(rule_first))
        })
        // .collect::<Vec<Vec<u8>>>();
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
        });
    // println!(
    //     "\n({}, {:?}) Result of rule checking: {:#?}",
    //     num_a, numbers, rule_filter
    // );

    rule_filter
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<Vec<u8>>, Vec<Vec<u8>>)) -> u16 {
    // println!("{:#?}", input.0);
    // println!("{:#?}", input.1);
    let rules = &input.0;
    let updates = &input.1;

    let mut total: u16 = 0;
    for update in updates {
        // println!("{:?}", update);

        let mut accepted_order = true;
        for rule in update.iter() {
            let result = check_rules(rules, *rule, update);
            if !result {
                accepted_order = false;
                break;
            }
        }

        if accepted_order {
            // println!("Order of {:?} accepted!", update);
            total += *update
                .get(update.len() / 2)
                .expect("Failed to get middle number of update set.") as u16;
        }
    }

    total
}

// get a list of the rules that are used with that number
fn get_desired_rules(rules: &[Vec<u8>], num_a: u8, numbers: &[u8]) -> Vec<Vec<u8>> {
    rules
        .to_owned()
        .clone()
        .into_iter()
        .filter(|x| {
            let rule_first = x.first().expect("Failed to get first number of rule");
            let rule_last = x.get(1).expect("Failed to get second number of rule");

            (rule_first == &num_a && numbers.contains(rule_last))
                || (rule_last == &num_a && numbers.contains(rule_first))
        })
        .collect::<Vec<Vec<u8>>>()
}

fn fix_rules(rules: &[Vec<u8>], numbers: &[u8]) -> Vec<u8> {
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

        // let rule_first_index = numbers.iter().position(|x| x == rule_first);
        // let rule_last_index = numbers.iter().position(|x| x == rule_last);

        // desired_rules.get
        // todo!();
        // a.cmp(b)

        // if we have no rule or can't determin ordering, just keep it where it is
        Ordering::Equal
    });

    update_order
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<Vec<u8>>, Vec<Vec<u8>>)) -> u16 {
    // println!("{:#?}", input.0);
    // println!("{:#?}", input.1);
    let rules = &input.0;
    let updates = &input.1;

    let mut total: u16 = 0;
    for update in updates {
        // println!("{:?}", update);

        let mut accepted_order = true;
        for rule in update.iter() {
            let result = check_rules(rules, *rule, update);
            if !result {
                accepted_order = false;
                break;
            }
        }

        if !accepted_order {
            let new_set = fix_rules(rules, update);
            // println!("New order of numbers {:?}", new_set);
            total += *new_set
                .get(new_set.len() / 2)
                .expect("Failed to get middle number of sorted set.") as u16;
        }
    }

    total
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
