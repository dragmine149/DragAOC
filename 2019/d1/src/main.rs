use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn caluclate_fuel_for_mass(mass: f32) -> i32 {
    let div3: f32 = mass / 3.0;
    let round: f32 = div3.floor();
    let sub2: f32 = round - 2.0;
    sub2 as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];
    let debug = &args[3];

    let file = File::open(file_path).expect("File should be able to be read");
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass = line.expect("Failed to read line");

        if mass.is_empty() {
            continue;
        }

        let fuel_for_mass = caluclate_fuel_for_mass(mass.parse::<f32>().unwrap());
        total_fuel += fuel_for_mass;

        if debug == "1" {
            println!("The fuel required for a mass of '{mass}' is '{fuel_for_mass}'");
        }
    }

    println!("The total fuel required is: {total_fuel}");
}
