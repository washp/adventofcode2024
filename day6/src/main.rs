mod part1;
mod part2;
mod utils;
use aoc_client::{AocClient, AocResult};
use std::{env, fs, path::Path, time::Instant};

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
    cur_path
        .expect("Failed to read current path!")
        .as_path()
        .iter()
        .last()
        .expect("Failed to get last element of path!")
        .to_str()
        .unwrap()
        .split("day")
        .last()
        .expect("Could not split directory name on 'day', please make sure directory name follows correct format!")
        .parse::<usize>()
        .expect("Could not convert day number to uint!")
}

fn main() {
    let input = get_input(&get_day());
    let part1_timer = Instant::now();
    println!("Processing part1...");
    let part1_result = part1::run(&input);
    println!("Part1: {} ({:?})", part1_result, part1_timer.elapsed());
    println!("Processing part2...");
    let part2_timer = Instant::now();
    let part2_result = part2::run(&input);
    println!("Part2: {} ({:?})", part2_result, part2_timer.elapsed());
}
