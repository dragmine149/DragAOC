use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::BinaryHeap;
use std::fmt;
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
        let colours = self
            .colours
            .iter()
            .map(|c| format!("{:?}", c))
            .collect::<String>();

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
    fn make_towel(
        &self,
        combinations: &TowelColourStorage,
        // cache: &mut Vec<(Vec<TowelColour>, Towel)>,
    ) -> u64 {
        // println!();
        println!("Processing towel: {:?}", self);

        let mut local_cahce: Vec<(Vec<TowelColour>, Towel)> = vec![];

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
            let index = progress.len();
            if index >= self.colours.len() {
                continue;
            }

            let options = combinations.get(&self.colours[index]);
            let combos = options.into_iter().filter(|t| {
                index + t.colours.len() <= self.colours.len()
                    && t.colours == self.colours[index..index + t.colours.len()]
            });

            for combo in combos {
                match local_cahce.iter().find(|x| x.0 == progress) {
                    Some(res) => {
                        if res.1 == *combo {
                            continue;
                        }
                    }
                    None => {}
                }

                let mut prog = progress.to_owned();
                prog.append(&mut combo.colours.to_vec());
                if prog == *self.colours {
                    // println!("Processed {:?}. Returning 1!", towel);
                    return 1;
                }

                // if

                heap.push(prog);
                local_cahce.push((progress.to_vec(), combo.to_owned()));
            }
        }

        // println!("Processed {:?}. Returning 0", towel);
        0
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<Towel>, Vec<Towel>) {
    let mut towels = input
        .lines()
        .nth(0)
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
    let combinations = TowelColourStorage::new(combinations.to_owned());

    // let mut cache = vec![];

    requested
        // .iter()
        .par_iter()
        .map(|towel| towel.make_towel(&combinations))
        .sum::<u64>()
    // + 1
}

// #[aoc(day19, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
