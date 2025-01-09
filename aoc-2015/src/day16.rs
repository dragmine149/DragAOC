use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
struct Information {
    num: u64,
    children: u64,
    cats: u64,
    samoyeds: u64,
    pomeranians: u64,
    akitas: u64,
    vizslas: u64,
    goldfish: u64,
    trees: u64,
    cars: u64,
    perfumes: u64,
}

impl PartialEq for Information {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.cats == other.cats
            && self.samoyeds == other.samoyeds
            && self.pomeranians == other.pomeranians
            && self.akitas == other.akitas
            && self.vizslas == other.vizslas
            && self.goldfish == other.goldfish
            && self.trees == other.trees
            && self.cars == other.cars
            && self.perfumes == other.perfumes
    }
}

impl Information {
    #[allow(dead_code)]
    fn debug(&self) {
        let num = format!("num: {},", self.num);
        let children = if self.children != u64::MAX {
            format!("children: {},", self.children)
        } else {
            "".to_string()
        };
        let cats = if self.cats != u64::MAX {
            format!("cats: {},", self.cats)
        } else {
            "".to_string()
        };
        let samoyeds = if self.samoyeds != u64::MAX {
            format!("samoyeds: {},", self.samoyeds)
        } else {
            "".to_string()
        };
        let pomeranians = if self.pomeranians != u64::MAX {
            format!("pomeranians: {},", self.pomeranians)
        } else {
            "".to_string()
        };
        let akitas = if self.akitas != u64::MAX {
            format!("akitas: {},", self.akitas)
        } else {
            "".to_string()
        };
        let vizslas = if self.vizslas != u64::MAX {
            format!("vizslas: {},", self.vizslas)
        } else {
            "".to_string()
        };
        let goldfish = if self.goldfish != u64::MAX {
            format!("goldfish: {},", self.goldfish)
        } else {
            "".to_string()
        };
        let trees = if self.trees != u64::MAX {
            format!("trees: {},", self.trees)
        } else {
            "".to_string()
        };
        let cars = if self.cars != u64::MAX {
            format!("cars: {},", self.cars)
        } else {
            "".to_string()
        };
        let perfumes = if self.perfumes != u64::MAX {
            format!("perfumes: {}", self.perfumes)
        } else {
            "".to_string()
        };
        println!(
            "Information {{ {} {} {} {} {} {} {} {} {} {} {} }}",
            num,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes
        );
    }

    const fn new(num: u64) -> Self {
        Self {
            num,
            ..Self::searched()
        }
    }

    const fn new_p2(num: u64) -> Self {
        Self {
            num,
            cats: u64::MAX,
            trees: u64::MAX,
            pomeranians: 0,
            goldfish: 0,
            ..Self::searched()
        }
    }

    const fn searched() -> Self {
        Self {
            num: u64::MAX,
            children: 3,
            cats: 7,
            samoyeds: 2,
            pomeranians: 3,
            akitas: 0,
            vizslas: 0,
            goldfish: 5,
            trees: 3,
            cars: 2,
            perfumes: 1,
        }
    }

    fn set(&mut self, field: &str, count: u64) {
        match field {
            "children" => self.children = count,
            "cats" => self.cats = count,
            "samoyeds" => self.samoyeds = count,
            "pomeranians" => self.pomeranians = count,
            "akitas" => self.akitas = count,
            "vizslas" => self.vizslas = count,
            "goldfish" => self.goldfish = count,
            "trees" => self.trees = count,
            "cars" => self.cars = count,
            "perfumes" => self.perfumes = count,
            _ => panic!("Invalid field! '{}'", field),
        }
    }

    fn real_eq(&self, other: &Self) -> bool {
        self.children != other.children
            || self.cats <= other.cats
            || self.samoyeds != other.samoyeds
            || self.pomeranians >= other.pomeranians
            || self.akitas != other.akitas
            || self.vizslas != other.vizslas
            || self.goldfish >= other.goldfish
            || self.trees <= other.trees
            || self.cars != other.cars
            || self.perfumes != other.perfumes
    }
}

#[aoc_generator(day16, part1)]
fn parse_p1(input: &str) -> Vec<Information> {
    input
        .lines()
        .map(|line| {
            let (num, rest) = line.split_once(":").expect("Failed to get number");
            let num = num.trim_start_matches("Sue").trim().parse().unwrap();
            let mut information = Information::new(num);
            for field in rest.split(",") {
                let (field, count) = field.split_once(':').expect("Could not split on ':'");
                let count = count.trim().parse().unwrap();
                information.set(field.trim(), count);
            }
            // information.debug();
            information
        })
        .collect_vec()
}

#[aoc_generator(day16, part2)]
fn parse_p2(input: &str) -> Vec<Information> {
    input
        .lines()
        .map(|line| {
            let (num, rest) = line.split_once(":").expect("Failed to get number");
            let num = num.trim_start_matches("Sue").trim().parse().unwrap();
            let mut information = Information::new_p2(num);
            for field in rest.split(",") {
                let (field, count) = field.split_once(':').expect("Could not split on ':'");
                let count = count.trim().parse().unwrap();
                information.set(field.trim(), count);
            }
            // information.debug();
            information
        })
        .collect_vec()
}

const TARGET: Information = Information::searched();

#[aoc(day16, part1)]
fn part1(input: &[Information]) -> u64 {
    input
        .iter()
        .find(|i| *i == &TARGET)
        .expect("Failed to find sue")
        .num
}

#[aoc(day16, part2)]
fn part2(input: &[Information]) -> u64 {
    input
        .iter()
        .find(|i| !i.real_eq(&TARGET))
        .expect("Failed to find sue")
        .num
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
//     }
// }
