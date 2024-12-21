use aoc_runner_derive::{aoc, aoc_generator};
use std::{char, fmt};

enum NumberKeys {
    Zero,
    Accept,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl fmt::Debug for NumberKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Accept => 'A',
                Self::Zero => '0',
                Self::One => '1',
                Self::Two => '2',
                Self::Three => '3',
                Self::Four => '4',
                Self::Five => '5',
                Self::Six => '6',
                Self::Seven => '7',
                Self::Eight => '8',
                Self::Nine => '9',
            }
        )
    }
}

impl NumberKeys {
    // hard code values to translate which buttons need to be pressed to press a button
    fn path(&self, destination: &Self) -> Vec<DirectionalKeys> {
        let mut keys = match self {
            Self::Zero => match destination {
                Self::Accept => vec![DirectionalKeys::Right],
                Self::Zero => vec![],
                Self::One => vec![DirectionalKeys::Up, DirectionalKeys::Left],
                Self::Two => vec![DirectionalKeys::Up],
                Self::Three => vec![DirectionalKeys::Up, DirectionalKeys::Right],
                Self::Four => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Left,
                ],
                Self::Five => vec![DirectionalKeys::Up, DirectionalKeys::Up],
                Self::Six => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                ],
                Self::Seven => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Left,
                ],
                Self::Eight => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
                Self::Nine => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                ],
            },
            Self::Accept => match destination {
                Self::Zero => vec![DirectionalKeys::Left],
                Self::Accept => vec![],
                Self::One => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                ],
                Self::Two => vec![DirectionalKeys::Left, DirectionalKeys::Up],
                Self::Three => vec![DirectionalKeys::Up],
                Self::Four => vec![
                    // < A A v < A A > > ^ A -> < v < A > > ^ A A < v A < A > > A v A A < ^ A > A
                    // < A v < A A > ^ A > A -> < v < A > > ^ A < v A < A > > ^ A A v A < ^ A > A v A ^ A
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                ],
                Self::Five => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
                Self::Six => vec![DirectionalKeys::Up, DirectionalKeys::Up],
                Self::Seven => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                ],
                Self::Eight => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
                Self::Nine => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
            },
            Self::One => match destination {
                Self::Zero => vec![DirectionalKeys::Right, DirectionalKeys::Down],
                Self::Accept => vec![
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                    DirectionalKeys::Down,
                ],
                Self::One => vec![],
                Self::Two => vec![DirectionalKeys::Right],
                Self::Three => vec![DirectionalKeys::Right, DirectionalKeys::Right],
                Self::Four => vec![DirectionalKeys::Up],
                Self::Five => vec![DirectionalKeys::Up, DirectionalKeys::Right],
                Self::Six => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
                Self::Seven => vec![DirectionalKeys::Up, DirectionalKeys::Up],
                Self::Eight => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                ],
                Self::Nine => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
            },
            Self::Two => match destination {
                Self::Zero => vec![DirectionalKeys::Down],
                Self::Accept => vec![DirectionalKeys::Down, DirectionalKeys::Right],
                Self::One => vec![DirectionalKeys::Left],
                Self::Two => vec![],
                Self::Three => vec![DirectionalKeys::Right],
                Self::Four => vec![DirectionalKeys::Left, DirectionalKeys::Up],
                Self::Five => vec![DirectionalKeys::Up],
                Self::Six => vec![DirectionalKeys::Up, DirectionalKeys::Right],
                Self::Seven => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
                Self::Eight => vec![DirectionalKeys::Up, DirectionalKeys::Up],
                Self::Nine => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                ],
            },
            Self::Three => match destination {
                Self::Zero => vec![DirectionalKeys::Left, DirectionalKeys::Down],
                Self::Accept => vec![DirectionalKeys::Down],
                Self::One => vec![DirectionalKeys::Left, DirectionalKeys::Left],
                Self::Two => vec![DirectionalKeys::Left],
                Self::Three => vec![],
                Self::Four => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                ],
                Self::Five => vec![DirectionalKeys::Left, DirectionalKeys::Up],
                Self::Six => vec![DirectionalKeys::Up],
                Self::Seven => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
                Self::Eight => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                    DirectionalKeys::Up,
                ],
                Self::Nine => vec![DirectionalKeys::Up, DirectionalKeys::Up],
            },
            Self::Four => match destination {
                Self::Zero => vec![
                    DirectionalKeys::Right,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Accept => vec![
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::One => vec![DirectionalKeys::Down],
                Self::Two => vec![DirectionalKeys::Down, DirectionalKeys::Right],
                Self::Three => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
                Self::Four => vec![],
                Self::Five => vec![DirectionalKeys::Right],
                Self::Six => vec![DirectionalKeys::Right, DirectionalKeys::Right],
                Self::Seven => vec![DirectionalKeys::Up],
                Self::Eight => vec![DirectionalKeys::Up, DirectionalKeys::Right],
                Self::Nine => vec![
                    DirectionalKeys::Up,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
            },
            Self::Five => match destination {
                Self::Zero => vec![DirectionalKeys::Down, DirectionalKeys::Down],
                Self::Accept => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                ],
                Self::One => vec![DirectionalKeys::Left, DirectionalKeys::Down],
                Self::Two => vec![DirectionalKeys::Down],
                Self::Three => vec![DirectionalKeys::Down, DirectionalKeys::Right],
                Self::Four => vec![DirectionalKeys::Left],
                Self::Five => vec![],
                Self::Six => vec![DirectionalKeys::Right],
                Self::Seven => vec![DirectionalKeys::Left, DirectionalKeys::Up],
                Self::Eight => vec![DirectionalKeys::Up],
                Self::Nine => vec![DirectionalKeys::Up, DirectionalKeys::Right],
            },
            Self::Six => match destination {
                Self::Zero => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Accept => vec![DirectionalKeys::Down, DirectionalKeys::Down],
                Self::One => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                ],
                Self::Two => vec![DirectionalKeys::Left, DirectionalKeys::Down],
                Self::Three => vec![DirectionalKeys::Down],
                Self::Four => vec![DirectionalKeys::Left, DirectionalKeys::Left],
                Self::Five => vec![DirectionalKeys::Left],
                Self::Six => vec![],
                Self::Seven => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                    DirectionalKeys::Up,
                ],
                Self::Eight => vec![DirectionalKeys::Left, DirectionalKeys::Up],
                Self::Nine => vec![DirectionalKeys::Up],
            },
            Self::Seven => match destination {
                Self::Zero => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                ],
                Self::Accept => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
                Self::One => vec![DirectionalKeys::Down, DirectionalKeys::Down],
                Self::Two => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                ],
                Self::Three => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
                Self::Four => vec![DirectionalKeys::Down],
                Self::Five => vec![DirectionalKeys::Down, DirectionalKeys::Right],
                Self::Six => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                ],
                Self::Seven => vec![],
                Self::Eight => vec![DirectionalKeys::Right],
                Self::Nine => vec![DirectionalKeys::Right, DirectionalKeys::Right],
            },
            Self::Eight => match destination {
                Self::Zero => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Accept => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                ],
                Self::One => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Two => vec![DirectionalKeys::Down, DirectionalKeys::Down],
                Self::Three => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Right,
                ],
                Self::Four => vec![DirectionalKeys::Left, DirectionalKeys::Down],
                Self::Five => vec![DirectionalKeys::Down],
                Self::Six => vec![DirectionalKeys::Down, DirectionalKeys::Right],
                Self::Seven => vec![DirectionalKeys::Left],
                Self::Eight => vec![],
                Self::Nine => vec![DirectionalKeys::Right],
            },
            Self::Nine => match destination {
                Self::Zero => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Accept => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::One => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Two => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                    DirectionalKeys::Down,
                ],
                Self::Three => vec![DirectionalKeys::Down, DirectionalKeys::Down],
                Self::Four => vec![
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                    DirectionalKeys::Down,
                ],
                Self::Five => vec![DirectionalKeys::Left, DirectionalKeys::Down],
                Self::Six => vec![DirectionalKeys::Down],
                Self::Seven => vec![DirectionalKeys::Left, DirectionalKeys::Left],
                Self::Eight => vec![DirectionalKeys::Left],
                Self::Nine => vec![],
            },
        };
        keys.push(DirectionalKeys::Accept);
        keys
    }
}

impl From<char> for NumberKeys {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::Accept,
            _ => panic!("Invalid number key!"),
        }
    }
}

enum DirectionalKeys {
    Up,
    Accept,
    Left,
    Down,
    Right,
}

impl fmt::Debug for DirectionalKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Accept => 'A',
                Self::Left => '<',
                Self::Right => '>',
                Self::Up => '^',
                Self::Down => 'v',
            }
        )
    }
}

impl DirectionalKeys {
    // hard code values to translate which buttons need to be pressed to press a button
    fn path(&self, destination: &Self) -> Vec<DirectionalKeys> {
        let mut keys = match self {
            Self::Up => match destination {
                Self::Up => vec![],
                Self::Accept => vec![DirectionalKeys::Right],
                Self::Left => vec![DirectionalKeys::Down, DirectionalKeys::Left],
                Self::Down => vec![DirectionalKeys::Down],
                Self::Right => vec![DirectionalKeys::Down, DirectionalKeys::Right],
            },
            Self::Accept => match destination {
                Self::Up => vec![DirectionalKeys::Left],
                Self::Accept => vec![],
                Self::Left => vec![
                    DirectionalKeys::Down,
                    DirectionalKeys::Left,
                    DirectionalKeys::Left,
                ],
                Self::Down => vec![DirectionalKeys::Left, DirectionalKeys::Down],
                Self::Right => vec![DirectionalKeys::Down],
            },
            Self::Left => match destination {
                Self::Up => vec![DirectionalKeys::Right, DirectionalKeys::Up],
                Self::Accept => vec![
                    DirectionalKeys::Right,
                    DirectionalKeys::Right,
                    DirectionalKeys::Up,
                ],
                Self::Left => vec![],
                Self::Down => vec![DirectionalKeys::Right],
                Self::Right => vec![DirectionalKeys::Right, DirectionalKeys::Right],
            },
            Self::Down => match destination {
                Self::Up => vec![DirectionalKeys::Up],
                Self::Accept => vec![DirectionalKeys::Up, DirectionalKeys::Right],
                Self::Left => vec![DirectionalKeys::Left],
                Self::Down => vec![],
                Self::Right => vec![DirectionalKeys::Right],
            },
            Self::Right => match destination {
                Self::Up => vec![DirectionalKeys::Left, DirectionalKeys::Up],
                Self::Accept => vec![DirectionalKeys::Up],
                Self::Left => vec![DirectionalKeys::Left, DirectionalKeys::Left],
                Self::Down => vec![DirectionalKeys::Left],
                Self::Right => vec![],
            },
        };
        // keys.insert(0, DirectionalKeys::Left);
        keys.push(DirectionalKeys::Accept);
        keys
    }

    fn index(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Accept => 1,
            Self::Left => 2,
            Self::Down => 3,
            Self::Right => 4,
        }
    }
}

impl From<usize> for DirectionalKeys {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Accept,
            2 => Self::Left,
            3 => Self::Down,
            4 => Self::Right,
            _ => panic!("Invalid number"),
        }
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<(Vec<NumberKeys>, u64)> {
    input
        .lines()
        .map(|line| {
            let keys = line
                .trim()
                .chars()
                .map(NumberKeys::from)
                .collect::<Vec<NumberKeys>>();

            let num = line
                .trim()
                .replace("A", "")
                .parse::<u64>()
                .expect("Failed to parse number somehow...");

            (keys, num)
        })
        .collect::<Vec<(Vec<NumberKeys>, u64)>>()
}

// translate the num pad into directional keys
fn get_number_combination(input: &[NumberKeys]) -> Vec<DirectionalKeys> {
    let mut pos = &NumberKeys::Accept;
    let mut directions = vec![];
    for destination in input {
        directions.append(&mut pos.path(destination));
        pos = destination;
    }
    directions
}
// translate the direction keys into more directional keys
fn get_directional_combination(
    start: &DirectionalKeys,
    input: &[DirectionalKeys],
) -> Vec<DirectionalKeys> {
    let mut pos = start;
    let mut directions = vec![];
    for destination in input {
        // let mut path = pos.path(destination);
        // println!(
        //     "Path from Pos({:?}) to Des({:?}) is {:?}",
        //     pos, destination, path
        // );
        directions.append(&mut pos.path(destination));
        pos = destination;
    }
    directions
}
// translates the directional keys into numbers. Use of last one to save memory.
fn get_directional_count(start: &DirectionalKeys, input: &[DirectionalKeys]) -> u64 {
    let mut pos = start;
    let mut count: u64 = 0;
    for destination in input {
        count += pos.path(destination).len() as u64;
        pos = destination;
    }
    count
}

// simple version
#[aoc(day21, part1)]
fn part1(input: &[(Vec<NumberKeys>, u64)]) -> u64 {
    input
        .iter()
        .map(|code| {
            let depressurized = &code.0;
            // println!("{:?}", depressurized);
            let radiation = get_number_combination(depressurized);
            // println!("{:?}", radiation);
            let cold = get_directional_combination(&DirectionalKeys::Accept, &radiation);
            // println!("{:?}", cold);
            let you = get_directional_combination(&DirectionalKeys::Accept, &cold);
            // println!("{:?}", you);

            // println!("Num: {:?}. You: {:?} ({:?})", code.1, you, you.len());
            code.1 * you.len() as u64
        })
        .sum()
}

// NOTE: In order to run this part, you need 48 odd minutes and (recommended) 10GiB of RAM.
#[aoc(day21, part2)]
fn part2(input: &[(Vec<NumberKeys>, u64)]) -> u64 {
    // < A
    // v < < A
    // < v A < A A > > ^ A
    // v < < A > A < A > > ^ A v A A < ^ A > A
    // < v A < A A > > ^ A v A ^ A v < < A > > ^ A v A A < ^ A > A < v A ^ > A A v < < A > ^ A > A v A ^ A

    // ^, ^, ^, A, v, A, <, ^, A, v, v, v, >, A
    //
    // v, <, <, A, >, >, ^, A, A, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, <, A, v, >, A, ^, A, <, v, A, <, A, A, >, >, ^, A, v, A, <, ^, A, >, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, A, A, v, A, ^, A, <, A, >, A
    // v, <, <, A, >, >, ^, A, v, <, <, A, >, >, ^, A, v, <, <, A, >, >, ^, A, A, v, <, <, A, >, A, ^, >, A, A, <, v, A, <, A, A, >, >, ^, A, v, <, <, A, >, >, ^, A, A, v, <, <, A, >, A, ^, >, A, v, <, <, A, >, A, ^, >, A, v, <, <, A, >, A, ^, >, A, <, v, A, ^, >, A, A

    // v, <, <, A, >, >, ^, A, > A, > A, v, >, A, ^, A, v, <, <, A, >, A, ^, >, A, ^, A, v, >, A, ^, A, <, v, A, <, A, A, >, >, ^, A, >, >, A, <, ^, A, >, A, v, >, A, ^, A, v, <, <, A, >, A, ^, >, A, ^, >, A, ^, >, A, >, A, ^, A, <, ^, A, >, A

    // ^, ^, ^, A, v, A, <, ^, A, v, v, v, >, A
    // <, A, A, A, >, A, <, v, A, ^, >, A, v, <, <, A, >, ^, A, >, A, <, v, A, A, A, >, A, ^, A
    // <, A, A, A, >, A, <, v, A, ^, >, A, v, <, <, A, >, ^, A, >, A, <, v, A, A, A, >, A, ^, A
    // v, <, <, A, >, >, ^, A, A, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, <, A, v, >, A, ^, A, <, v, A, <, A, A, >, >, ^, A, v, A, <, ^, A, >, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, A, A, v, A, ^, A, <, A, >, A
    // v, <, <, A, >, >, ^, A, >, A, >, A, v, >, A, ^, A, v, <, <, A, >, A, ^, >, A, ^, A, v, >, A, ^, A, <, v, A, <, A, A, >, >, ^, A, >, >, A, <, ^, A, >, A, v, >, A, ^, A, v, <, <, A, >, A, ^, >, A, ^, >, A, ^, >, A, >, A, ^, A, <, ^, A, >, A
    // v, <, <, A, >, >, ^, A, A, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, <, A, v, >, A, ^, A, <, v, A, <, A, A, >, >, ^, A, v, A, <, ^, A, >, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, A, A, v, A, ^, A, <, A, >, A

    // v, <, <, A, >, >, ^, A, A, v, A, ^, A, v, <, <, A, >, A, ^, >, A, <, A, v, >, A, ^, A, <, v, A, <, A, A, >, >, ^, A, v, A, <, ^, A, >, A, <, v, A, ^, >, A, A, v, A, ^, A, <, A, >, A

    let mut cache = [u64::MAX; 25];

    // for each input
    input
        .iter()
        // .par_iter()
        .map(|code| {
            // for each char in the input
            println!("Computing: {:?}", code);
            let keypad = &code.0;
            // println!("{:?}", keypad);
            // calculate the first set of directional keys
            let combo_1 = get_number_combination(keypad);
            // println!("{:?}", combo_1);
            // println!("1");
            // ^ A < v >
            let mut sum: u64 = 0;
            let mut previous = &DirectionalKeys::Accept;
            // let mut a = vec![];
            // split up the work load, one char at a time
            combo_1.iter().for_each(|char| {
                // println!("Calculating: prev({:?}) to next({:?})", previous, char);
                let c = char.to_owned();
                let pos = c.index();
                let cache_pos = pos + (previous.index() * 5);
                // check if we haven't come across this combination before
                if cache[cache_pos] != u64::MAX {
                    sum += cache[cache_pos];
                    previous = char;
                    println!("Cache!");
                    // println!(
                    //     "Used cache as cacluated prev({:?}) to next({:?}) before score ({:?})",
                    //     previous, c, cache[cache_pos]
                    // );
                    return;
                }

                // calculate all the combinations

                // println!("Prev: {:?} Char: {:?}", previous, char);
                let combo_2 = get_directional_combination(previous, &[DirectionalKeys::from(pos)]);
                // println!("2");
                // println!("{:?}", combo_2);
                let combo_3 = get_directional_combination(&DirectionalKeys::Accept, &combo_2);
                // println!("3");
                // let combo_3 = get_directional_count(&DirectionalKeys::Accept, &combo_2);
                previous = char; // doesn't actuall matter where this happens, as long as it's after combo_2

                // for i in combo_3.iter() {
                //     a.push(DirectionalKeys::from(i.index()));
                // }

                // cache[cache_pos] = combo_3;
                // sum += combo_3;

                let combo_4 = get_directional_combination(&DirectionalKeys::Accept, &combo_3);
                println!("4");
                let combo_5 = get_directional_combination(&DirectionalKeys::Accept, &combo_4);
                println!("5");
                let combo_6 = get_directional_combination(&DirectionalKeys::Accept, &combo_5);
                println!("6");
                let combo_7 = get_directional_combination(&DirectionalKeys::Accept, &combo_6);
                println!("7");
                let combo_8 = get_directional_combination(&DirectionalKeys::Accept, &combo_7);
                println!("8");
                let combo_9 = get_directional_combination(&DirectionalKeys::Accept, &combo_8);
                println!("9");
                let combo_10 = get_directional_combination(&DirectionalKeys::Accept, &combo_9);
                println!("10");
                let combo_11 = get_directional_combination(&DirectionalKeys::Accept, &combo_10);
                println!("11");
                let combo_12 = get_directional_combination(&DirectionalKeys::Accept, &combo_11);
                println!("12");
                let combo_13 = get_directional_combination(&DirectionalKeys::Accept, &combo_12);
                println!("13");
                let combo_14 = get_directional_combination(&DirectionalKeys::Accept, &combo_13);
                println!("14");
                let combo_15 = get_directional_combination(&DirectionalKeys::Accept, &combo_14);
                println!("15");
                let combo_16 = get_directional_combination(&DirectionalKeys::Accept, &combo_15);
                println!("16");
                let combo_17 = get_directional_combination(&DirectionalKeys::Accept, &combo_16);
                println!("17");
                let combo_18 = get_directional_combination(&DirectionalKeys::Accept, &combo_17);
                println!("18");
                let combo_19 = get_directional_combination(&DirectionalKeys::Accept, &combo_18);
                println!("19");
                let combo_20 = get_directional_combination(&DirectionalKeys::Accept, &combo_19);
                println!("20");
                let combo_21 = get_directional_combination(&DirectionalKeys::Accept, &combo_20);
                println!("21");
                let combo_22 = get_directional_combination(&DirectionalKeys::Accept, &combo_21);
                println!("22");
                let combo_23 = get_directional_combination(&DirectionalKeys::Accept, &combo_22);
                println!("23");
                let combo_24 = get_directional_combination(&DirectionalKeys::Accept, &combo_23);
                println!("24");
                let combo_25 = get_directional_combination(&DirectionalKeys::Accept, &combo_24);
                println!("25");
                // saves memory by reducing the list. And well, we don't also need a big long list here.
                let combo_26 = get_directional_count(&DirectionalKeys::Accept, &combo_25);

                // add to cache and sum
                cache[cache_pos] = combo_26;
                sum += combo_26;
            });
            // println!(
            //     "{:?}",
            //     a.iter()
            //         .map(|c| format!("{:?}", c))
            //         .collect::<Vec<String>>()
            //         .join(", ")
            // );

            // println!("{:?}", cache);
            code.1 * sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "029A
980A
179A
456A
379A
";

    // https://www.reddit.com/r/adventofcode/comments/1hj6o1j/comment/m347iul/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    const COMMUNITY_1: &str = "159A
375A
613A
894A
080A
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 126384);
    }

    #[test]
    fn part1_community() {
        assert_eq!(part1(&parse(COMMUNITY_1)), 151826);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
