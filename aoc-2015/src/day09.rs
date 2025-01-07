use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse(input: &str) -> HashMap<String, HashMap<String, u64>> {
    let mut distances: HashMap<String, HashMap<String, u64>> = HashMap::new();

    // create a map of maps of distances
    input.lines().for_each(|line| {
        let regex = Regex::new(r"(?m)(?P<start>\w+) to (?P<end>\w+) = (?P<dist>\d+)").unwrap();
        let info = regex.captures(line).unwrap();
        let start = info.name("start").unwrap().as_str();

        // get the start location map
        let temp = distances.get_mut(start);
        let loc: &mut HashMap<String, u64> = if let Some(v) = temp {
            v
        } else {
            let tloc = HashMap::new();
            distances.insert(start.to_string(), tloc);
            distances.get_mut(start).unwrap()
        };

        // insert the distance to the end location
        let end_str = info.name("end").unwrap().as_str();
        let distance = info
            .name("dist")
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse distance");
        loc.insert(end_str.to_string(), distance);

        // get the end location map
        let e_temp = distances.get_mut(end_str);
        let e_loc: &mut HashMap<String, u64> = if let Some(v) = e_temp {
            v
        } else {
            let e_tloc = HashMap::new();
            distances.insert(end_str.to_string(), e_tloc);
            distances.get_mut(end_str).unwrap()
        };

        // insert the end location distance
        let distance = info
            .name("dist")
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse distance");
        e_loc.insert(start.to_string(), distance);
    });

    distances
}

fn create_shortest(
    map: &HashMap<String, HashMap<String, u64>>,
    start: &str,
    exclude: Vec<String>,
) -> u64 {
    // println!("{:?}", start);
    // return the distance by
    map.get(start)
        .unwrap() // getting the distances
        .iter()
        .filter(|n| !exclude.iter().contains(n.0)) // ignoring those we've already been to
        .map(|n| {
            // assign the rest the map of what is left
            let mut e = exclude.to_owned();
            e.push(start.to_string());
            n.1 + create_shortest(map, n.0, e)
        })
        .min() // getting the minium
        .unwrap_or(0) // or 0 if at end of journey
}

#[aoc(day9, part1)]
fn part1(input: &HashMap<String, HashMap<String, u64>>) -> u64 {
    input
        .keys()
        .map(|node| create_shortest(input, node, vec![]))
        .min()
        .unwrap()
}

fn create_longest(
    map: &HashMap<String, HashMap<String, u64>>,
    start: &str,
    exclude: Vec<String>,
) -> u64 {
    // println!("{:?}", start);
    map.get(start)
        .unwrap()
        .iter()
        .filter(|n| !exclude.iter().contains(n.0))
        .map(|n| {
            let mut e = exclude.to_owned();
            e.push(start.to_string());
            n.1 + create_longest(map, n.0, e)
        })
        .max()
        .unwrap_or(0)
}

#[aoc(day9, part2)]
fn part2(input: &HashMap<String, HashMap<String, u64>>) -> u64 {
    input
        .keys()
        .map(|node| create_longest(input, node, vec![]))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 982);
    }
}
