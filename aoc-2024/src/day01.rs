use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> i64 {
    // Create two lists and fill them up
    let mut list_a: Vec<i64> = Vec::new();
    let mut list_b: Vec<i64> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let x: Vec<&str> = line.split("   ").collect();

        let num_a = x[0]
            .parse::<i64>()
            .expect("Failed to convert number 1 to number");
        let num_b = x[1]
            .parse::<i64>()
            .expect("Failed to convert number 2 to number");

        list_a.push(num_a);
        list_b.push(num_b);
    }

    // sort the lists
    list_a.sort();
    list_b.sort();

    // Make a new list with the difference of all the values
    let mut diff: Vec<i64> = Vec::new();

    for (index, value) in list_a.iter().enumerate() {
        diff.push((value - list_b[index]).abs());
    }

    diff.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u64 {
    // Once again split the data up
    let mut list_a: Vec<u64> = Vec::new();
    let mut list_b: Vec<u64> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let x: Vec<&str> = line.split("   ").collect();

        let num_a = x[0]
            .parse::<u64>()
            .expect("Failed to convert number 1 to number");
        let num_b = x[1]
            .parse::<u64>()
            .expect("Failed to convert number 2 to number");

        list_a.push(num_a);
        list_b.push(num_b);
    }

    let mut list_c: Vec<u64> = Vec::new();
    // let list_d: Vec<u64> = Vec::new();

    // Count how many times the value in the first list appears in the second list.
    for value in list_a.iter() {
        let mut count: u64 = 0;
        if list_b.contains(value) {
            for value_2 in list_b.iter() {
                if value_2 == value {
                    count += 1;
                }
            }
        }
        list_c.push(count);
        // list_d.push(count * value);
    }

    list_a
        .iter()
        .enumerate()
        .map(|(index, value)| value * list_c[index])
        .sum()

    // println!("{:#?}", list_c);
    // println!("{:#?}", list_d);

    // list_d.iter().sum()
    // list_c.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 31);
    }
}
