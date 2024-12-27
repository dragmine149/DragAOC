use aoc_runner_derive::aoc;
use md5::{Digest, Md5};

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    // println!("{}", format!("{}{}", input.trim(), 609043));
    // let h = Md5::new_with_prefix(format!("{}{}", input.trim(), 609043).as_bytes());
    // let f = h.finalize();
    // println!("{:#x}", f);
    // return 0;

    let mut index = 1;
    loop {
        let hasher = Md5::new_with_prefix(format!("{}{}", input.trim(), index).as_bytes());
        let fin = hasher.finalize();
        let hex = format!("{:#x}", fin);
        if hex.starts_with("00000") {
            break index;
        }
        index += 1;
    }
}

// #[aoc(day4, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("abcdef"), 609043);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
