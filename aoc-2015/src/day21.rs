use std::env::consts::ARCH;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    cost: u16,
    damage: u8,
    armor: u8,
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
const ARMORS: [Item; 6] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];
const RINGS: [Item; 7] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

fn fight(status: &Item, boss_status: &Item) -> bool {
    // println!("Fighting {:?} with {:?}", boss_status, status);

    let damage_to_self = boss_status.damage.saturating_sub(status.armor).max(1);
    let damage_to_boss = status.damage.saturating_sub(boss_status.armor).max(1);

    let self_hits_to_death = (status.cost as u8).div_ceil(damage_to_self);
    let boss_hits_to_death = (boss_status.cost as u8).div_ceil(damage_to_boss);

    // println!(
    //     "Result: {:?} -> {:?}",
    //     self_hits_to_death, boss_hits_to_death
    // );

    self_hits_to_death >= boss_hits_to_death
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Item {
    let stats: (u8, u8, u8) = input
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .expect("Failed to split line")
                .parse()
                .expect("Failed to turn into number")
        })
        .collect_tuple()
        .expect("Failed to parse input");
    Item {
        cost: stats.0 as u16,
        damage: stats.1,
        armor: stats.2,
    }
}

fn shop<F: Fn(&Item, &Item, &Item, &Item) -> u16>(result: F) -> u16 {
    WEAPONS
        .iter()
        .map(|weapon| {
            ARMORS
                .iter()
                .map(|armor| {
                    RINGS
                        .iter()
                        .map(|ring| {
                            RINGS
                                .iter()
                                .map(|ring2| result(weapon, armor, ring, ring2))
                                .min()
                                .expect("Failed to get min of ring 2")
                        })
                        .min()
                        .expect("Failed to get min of ring 1")
                })
                .min()
                .expect("Failed to get min of armors")
        })
        .min()
        .expect("Failed to get min of weapons")
}

#[aoc(day21, part1)]
fn part1(input: &Item) -> u16 {
    shop(|weapon, armor, ring, ring2| {
        if ring2 == ring {
            return u16::MAX;
        }

        let self_status = Item {
            cost: 100,
            damage: weapon.damage + armor.damage + ring.damage + ring2.damage,
            armor: weapon.armor + armor.armor + ring.armor + ring2.armor,
        };
        let result = fight(&self_status, input);
        if result {
            let cost = weapon.cost + armor.cost + ring.cost + ring2.cost;
            // println!(
            //     "Result of fighting with {:?}: {:?} ({:?}, {:?}, {:?}, {:?})",
            //     self_status, cost, weapon, armor, ring, ring2
            // );
            cost
        } else {
            u16::MAX
        }
    })
}

// #[aoc(day21, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            fight(
                &Item {
                    cost: 8,
                    damage: 5,
                    armor: 5
                },
                &Item {
                    cost: 12,
                    damage: 7,
                    armor: 2,
                }
            ),
            true
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
