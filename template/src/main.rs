mod part1;
mod part2;
mod utils;
use aoc_client::{AocClient, AocResult};

fn get_aoc_input() -> AocResult<String> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(1)?
        .build()?;

    let input: String = client.get_input()?;
    Ok(input)
}

fn main() {
    // this should probably be written to disk, so I don't grab it everytime...
    let input = match get_aoc_input() {
        Ok(input) => input,
        Err(e) => panic!("AoC client error: {}", e),
    };
    println!("Processing part1...");
    let part1_result = part1::run(&input);
    println!("Processing part2...");
    let part2_result = part2::run(&input);
    println!("Part1: {}", part1_result);
    println!("Part2: {}", part2_result);
}
