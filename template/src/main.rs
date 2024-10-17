mod part1;
mod part2;
mod utils;
use aoc_client::{AocClient, AocResult};
use std::{fs, path::Path};

const INPUT_FILE: &str = "./input.txt";

fn get_input_from_aoc() -> AocResult<String> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2023)?
        .day(1)?
        .build()?;

    println!("Getting input from AoC site...");
    let input: String = client.get_input()?;
    Ok(input)
}

fn get_input() -> String {
    match Path::new(INPUT_FILE).exists() {
        true => fs::read_to_string(INPUT_FILE).unwrap(),
        false => match get_input_from_aoc() {
            Ok(input) => {
                fs::write(INPUT_FILE, &input).expect("Failed to write input file!");
                input
            }
            Err(e) => panic!("AoC client error: {}", e),
        },
    }
}

fn main() {
    let input = get_input();
    println!("Input: {}", input);
    println!("Processing part1...");
    let part1_result = part1::run(&input);
    println!("Processing part2...");
    let part2_result = part2::run(&input);
    println!("Part1: {}", part1_result);
    println!("Part2: {}", part2_result);
}
