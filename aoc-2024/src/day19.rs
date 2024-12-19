use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::BinaryHeap;
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TowelColour {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl fmt::Debug for TowelColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}",
                match self {
                    TowelColour::Black => "TowelColour::Black",
                    TowelColour::Blue => "TowelColour::Blue",
                    TowelColour::Green => "TowelColour::Green",
                    TowelColour::Red => "TowelColour::Red",
                    TowelColour::White => "TowelColour::White",
                }
            )
        } else {
            write!(
                f,
                "{}",
                match self {
                    TowelColour::Black => "b",
                    TowelColour::Blue => "u",
                    TowelColour::Green => "g",
                    TowelColour::Red => "r",
                    TowelColour::White => "w",
                }
            )
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Towel {
    colours: Vec<TowelColour>,
}

impl fmt::Debug for Towel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colours = self.colours.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{:?}", c);
            output
        });

        write!(f, "Towel({:?})", colours)
    }
}

impl FromStr for Towel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colours = vec![];
        for char in s.chars() {
            if char == 'w' {
                colours.push(TowelColour::White);
                continue;
            }
            if char == 'u' {
                colours.push(TowelColour::Blue);
                continue;
            }
            if char == 'b' {
                colours.push(TowelColour::Black);
                continue;
            }
            if char == 'g' {
                colours.push(TowelColour::Green);
                continue;
            }
            if char == 'r' {
                colours.push(TowelColour::Red);
                continue;
            }
            return Err(format!("{char:?} is not a valid towel colour!"));
        }

        Ok(Self { colours })
    }
}

#[derive(Clone)]
struct TowelColourStorage {
    white: Vec<Towel>,
    black: Vec<Towel>,
    blue: Vec<Towel>,
    green: Vec<Towel>,
    red: Vec<Towel>,
}

impl TowelColourStorage {
    #[allow(dead_code)]
    fn new(towels: Vec<Towel>) -> Self {
        let mut white = vec![];
        let mut black = vec![];
        let mut blue = vec![];
        let mut green = vec![];
        let mut red = vec![];

        for towel in towels {
            match towel.colours[0] {
                TowelColour::White => white.push(towel),
                TowelColour::Black => black.push(towel),
                TowelColour::Blue => blue.push(towel),
                TowelColour::Green => green.push(towel),
                TowelColour::Red => red.push(towel),
            }
        }

        Self {
            white,
            black,
            blue,
            green,
            red,
        }
    }

    fn get(&self, colour: &TowelColour) -> impl IntoIterator<Item = &Towel> + '_ {
        match colour {
            TowelColour::Red => self.red.iter(),
            TowelColour::Blue => self.blue.iter(),
            TowelColour::Black => self.black.iter(),
            TowelColour::Green => self.green.iter(),
            TowelColour::White => self.white.iter(),
        }
    }
}

impl Towel {
    // original function for getting the count of possible ones.
    #[allow(dead_code)]
    fn make_towel(&self, combinations: &TowelColourStorage) -> u64 {
        // println!();
        // println!("Processing towel: {:?}", self);

        let mut local_cahce: Vec<(Vec<TowelColour>, Towel)> = vec![];

        // binary heap for efficiency
        let mut heap = BinaryHeap::new();
        heap.push(vec![]);
        while let Some(progress) = heap.pop() {
            // println!("--");
            // println!(
            //     "Progress: {:?}",
            //     Towel {
            //         colours: progress.to_vec()
            //     }
            // );

            // check if we aren't out of bounds
            let index = progress.len();
            if index >= self.colours.len() {
                continue;
            }

            // get the possible combinations of that colour
            let options = combinations.get(&self.colours[index]);
            let combos = options.into_iter().filter(|t| {
                // filter the combinations out so that we only have those which are
                // In length, and match the next few colours
                index + t.colours.len() <= self.colours.len()
                    && t.colours == self.colours[index..index + t.colours.len()]
            });

            // loop though them all
            for combo in combos {
                // check if we haven't already done this combination
                if let Some(res) = local_cahce.iter().find(|x| x.0 == progress) {
                    if res.1 == *combo {
                        continue;
                    }
                }

                // create a new one
                let mut prog = progress.to_owned();
                prog.append(&mut combo.colours.to_vec());
                if prog == *self.colours {
                    // if reach end
                    // println!("Processed {:?}. Returning 1!", towel);
                    return 1;
                }

                // add to heap and cache to search next.
                heap.push(prog);
                local_cahce.push((progress.to_vec(), combo.to_owned()));
            }
        }

        // println!("Processed {:?}. Returning 0", towel);
        0
    }

    fn make_towel_2(&self, combinations: &[Towel]) -> u64 {
        let r = pattern_cache(
            &self.colours,
            combinations,
            0,
            &mut vec![u64::MAX; self.colours.len()],
        );
        if r > 1 {
            1
        } else {
            0
        }
    }

    fn all_combos(&self, combinations: &[Towel]) -> u64 {
        pattern_cache(
            &self.colours,
            combinations,
            0,
            &mut vec![u64::MAX; self.colours.len()],
        )
    }
}

fn pattern_cache(
    design: &[TowelColour],
    combinations: &[Towel],
    start: usize,
    cache: &mut [u64],
) -> u64 {
    // println!("Design: {:?}", design);

    // check that the design has enough length otherwise we probably at the end here
    if design.len() <= start {
        return 1; // well, always one
    }

    // let hash = design_hash(design);
    // check if we haven't cached this size yet
    if cache[start] != u64::MAX {
        return cache[start];
    }

    // create the new design and check all combinations
    let matched_design = &design[start..];
    let mut sum = 0;
    for combination in combinations {
        // println!("{:?} {:?}", matched_design, combination);
        if !matched_design.starts_with(&combination.colours[0..]) {
            // if it doesn't match, its not a part of us so we don't care
            continue;
        }

        // repeat this whole function
        sum += pattern_cache(
            design,
            combinations,
            start + combination.colours.len(),
            cache,
        );
        // println!("Sum: {:?}", sum);
    }
    // println!("Hash: {:?} set to {:?}", start, sum);

    cache[start] = sum;

    sum
}

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<Towel>, Vec<Towel>) {
    let mut towels = input
        .lines()
        .next()
        .expect("Failed to get the first line")
        .trim()
        .replace(" ", "")
        .split(",")
        .map(|towel| Towel::from_str(towel).expect("Failed to make a towel!"))
        .collect::<Vec<Towel>>();
    let combinations = input
        .replace(" ", "")
        .lines()
        .skip(2)
        .map(|towel| Towel::from_str(towel).expect("Failed to make a towel!"))
        .collect::<Vec<Towel>>();
    towels.sort();

    (towels, combinations)
}

#[aoc(day19, part1)]
fn part1(input: &(Vec<Towel>, Vec<Towel>)) -> u64 {
    let combinations = &input.0;
    let requested = &input.1;
    // let combinations = TowelColourStorage::new(combinations.to_owned());

    requested
        // .iter()
        .par_iter()
        // .map(|towel| towel.make_towel(&combinations))
        .map(|towel| towel.make_towel_2(combinations))
        .sum::<u64>()
    // + 1
}

#[aoc(day19, part2)]
fn part2(input: &(Vec<Towel>, Vec<Towel>)) -> u64 {
    let combinations = &input.0;
    let requested = &input.1;
    // let combinations = TowelColourStorage::new(combinations.to_owned());

    requested
        // .iter()
        .par_iter()
        .map(|towel| towel.all_combos(combinations))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 16);
    }
}
