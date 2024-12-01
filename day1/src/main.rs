mod part1;
mod part2;
mod utils;
use aoc_client::{AocClient, AocResult};
use std::{env, fs, path::Path};

const YEAR: i32 = 2024;
const INPUT_FILE: &str = "./input.txt";

fn get_input_from_aoc(day: &usize) -> AocResult<String> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(YEAR)?
        .day(*day as u32)?
        .build()?;

    println!("Getting input from AoC site...");
    let input: String = client.get_input()?;
    Ok(input)
}

fn get_input(day: &usize) -> String {
    match Path::new(INPUT_FILE).exists() {
        true => fs::read_to_string(INPUT_FILE).unwrap(),
        false => {
            println!(
                "No local input.txt found for day {}, attempting to download from AoC...",
                day
            );
            match get_input_from_aoc(day) {
                Ok(input) => {
                    fs::write(INPUT_FILE, &input).expect("Failed to write input file!");
                    input
                }
                Err(e) => panic!("AoC client error: {}", e),
            }
        }
    }
}

fn get_day() -> usize {
    let cur_path = env::current_dir();
    if let Ok(path) = cur_path {
        if let Some(dir_name) = path.as_path().iter().last() {
            if let Some(day) = dir_name.to_str().unwrap().split("day").last() {
                return day.parse::<usize>().unwrap();
            }
        }
    }
    panic!(
        "Failed to figure out day number, please verify that you are running in a correct directory, ie 'day2'"
    );
}

fn main() {
    let input = get_input(&get_day());
    println!("Processing part1...");
    let part1_result = part1::run(&input);
    println!("Processing part2...");
    let part2_result = part2::run(&input);
    println!("Part1: {}", part1_result);
    println!("Part2: {}", part2_result);
}
