use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day20)]
fn parse(input: &str) -> u64 {
    input.parse().expect("Failed to parse input")
}

// House 1 : 1                       = 1
// House 2 : 1 2                     = 3
// House 3 : 1   3                   = 4
// House 4 : 1 2   4                 = 7
// House 5 : 1       5               = 6
// House 6 : 1 2 3     6             = 12
// House 7 : 1           7           = 8
// House 8 : 1 2   4       8         = 15
// House 9 : 1   3           9       = 13
// House 10: 1 2     5         10    = 22

fn loop_elf(goal: &u64, elf_count: u64) -> u64 {
    let mut count = 0;
    let mut max = 0;
    (1..elf_count).map(|elf| elf * 10).for_each(|elf| {
        if &max < goal {
            count += 1;
        }
        max += elf;
    });

    if max < *goal {
        // break out before doing anything if we have no chance at working something out.
        return 0;
    }
    println!("elfs: {:?}. min: {:?} max: {:?}", elf_count, count, max);
    return 0;

    let mut house = 1;
    loop {
        let score = (1..elf_count)
            .filter(|elf| house % elf == 0)
            .map(|elf| elf * 10)
            .sum::<u64>();
        // println!("Elfs {:?} at house {:?} for {:?}", elf_count, house, score);
        if score >= *goal {
            break house;
        }
        if house > *goal {
            break 0;
        }
        house += 1;
    }
}

#[aoc(day20, part1)]
fn part1(input: &u64) -> u64 {
    let elfs = u64::MAX;
    let mut min_elf_count = 0;
    let mut sum = 0;
    for elf in (1..elfs).map(|elf| elf * 10) {
        if &sum > input {
            break;
        }
        sum += elf;
        min_elf_count += 1;
    }

    println!("min: {:?}", min_elf_count);
    return 0;

    let mut lowest = u64::MAX;
    let mut elf_count = 2;
    loop {
        let house = loop_elf(input, elf_count);
        // println!(
        //     "Elfs: {:?}, house: {:?}, Goal: {:?}, Lowest: {:?}",
        //     elf_count, house, input, lowest
        // );
        elf_count += 1;

        if house == 0 {
            continue;
        }

        if house == lowest {
            // at this point, no matter what we do we're going to hit the house so...
            break;
        }
        lowest = lowest.min(house);
    }
    lowest
}

// #[aoc(day20, part2)]
// fn part2(input: &u64) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("60")), 4);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse("120")), 6);
    }
    #[test]
    fn part1_example3() {
        assert_eq!(part1(&parse("130")), 9);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
