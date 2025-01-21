use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// From the AOC page, the shop selling stuff
const WEAPONS: [(u8, u8, u8); 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMORS: [(u8, u8, u8); 6] = [
    (0, 0, 0), // "Optional"
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];
const RINGS: [(u8, u8, u8); 7] = [
    (0, 0, 0), // "Optional"
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

// Fight the boss
fn fight(status: &(u8, u8, u8), boss_status: &(u8, u8, u8)) -> bool {
    // println!("Fighting {:?} with {:?}", boss_status, status);

    // To save time and calculations, we don't actually need to simulate it
    let damage_to_self = boss_status.1.saturating_sub(status.2).max(1);
    let damage_to_boss = status.1.saturating_sub(boss_status.2).max(1);

    // Getting how many hits we can survive is more than enough as nothing changes
    let self_hits_to_death = status.0.div_ceil(damage_to_self);
    let boss_hits_to_death = boss_status.0.div_ceil(damage_to_boss);

    // println!(
    //     "Result: {:?} -> {:?}",
    //     self_hits_to_death, boss_hits_to_death
    // );

    // Hence as long as we have more or equal (as we go first, it doesn't matter if they die in the same amount of hits)
    // we win
    self_hits_to_death >= boss_hits_to_death
}

#[aoc_generator(day21)]
fn parse(input: &str) -> (u8, u8, u8) {
    input
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .expect("Failed to split line")
                .parse()
                .expect("Failed to turn into number")
        })
        .collect_tuple()
        .expect("Failed to parse input")
}

// Time to go shopping (the iter stack was too big)
fn shop<F: Fn(&(u8, u8, u8), &(u8, u8, u8), &(u8, u8, u8), &(u8, u8, u8)) -> u16>(
    result: F,
) -> u16 {
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

// Shopping p2 but max this time
fn shop_biased<F: Fn(&(u8, u8, u8), &(u8, u8, u8), &(u8, u8, u8), &(u8, u8, u8)) -> u16>(
    result: F,
) -> u16 {
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
                                .max()
                                .expect("Failed to get min of ring 2")
                        })
                        .max()
                        .expect("Failed to get min of ring 1")
                })
                .max()
                .expect("Failed to get min of armors")
        })
        .max()
        .expect("Failed to get min of weapons")
}

fn process(
    boss_status: &(u8, u8, u8),
    weapon: &(u8, u8, u8),
    armor: &(u8, u8, u8),
    ring: &(u8, u8, u8),
    ring2: &(u8, u8, u8),
    failed: u16,
) -> u16 {
    // we can't have two of the sam item
    if ring == ring2 {
        // unless those items are neither
        if ring.0 != 0 || ring.1 != 0 || ring.2 != 0 {
            return failed;
        }
    }

    // fight, let see who wins
    let result = fight(
        &(
            100,
            weapon.1 + armor.1 + ring.1 + ring2.1,
            weapon.2 + armor.2 + ring.2 + ring2.2,
        ),
        boss_status,
    );
    // calculate and return.
    if result {
        weapon.0 as u16 + armor.0 as u16 + ring.0 as u16 + ring2.0 as u16
    } else {
        // it's free if we loose
        failed
    }
}

#[aoc(day21, part1)]
fn part1(input: &(u8, u8, u8)) -> u16 {
    shop(|weapon, armor, ring, ring2| process(input, weapon, armor, ring, ring2, u16::MAX))
}

#[aoc(day21, part2)]
fn part2(input: &(u8, u8, u8)) -> u16 {
    shop_biased(|weapon, armor, ring, ring2| process(input, weapon, armor, ring, ring2, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(fight(&(8, 5, 5), &(12, 7, 2)), true);
    }
}
