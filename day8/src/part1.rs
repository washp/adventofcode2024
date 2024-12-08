use std::collections::{HashMap, HashSet};

use crate::utils::Vector2D;

type Int = isize;
type Coord = Vector2D<Int>;
type SignalType = HashMap<char, Vec<Coord>>;
type Size = (usize, usize);

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (SignalType, Size) {
    let mut signals: SignalType = HashMap::new();
    let height = lines.len();
    let width = lines.first().expect("No first row in data").len();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                let pos = Coord::new(x as Int, y as Int);
                signals.entry(char).or_default().push(pos);
            }
        }
    }
    (signals, (width, height))
}

fn is_on_map(pos: &Coord, (width, height): &Size) -> bool {
    pos.x >= 0 && pos.x < *width as isize && pos.y >= 0 && pos.y < *height as isize
}

fn calculate_antinodes(sig1: &Coord, sig2: &Coord, size: &Size) -> HashSet<Coord> {
    let mut antinodes: HashSet<Coord> = HashSet::new();
    let node_vec = *sig2 - *sig1;
    let node1 = *sig1 - node_vec;
    let node2 = *sig2 + node_vec;
    if is_on_map(&node1, size) {
        antinodes.insert(node1);
    }
    if is_on_map(&node2, size) {
        antinodes.insert(node2);
    }
    antinodes
}

fn find_nr_antinodes(signals: SignalType, size: Size) -> usize {
    let mut unique_antinodes: HashSet<Coord> = HashSet::new();
    for (_, locs) in signals {
        for (i, loc1) in locs.iter().enumerate() {
            for loc2 in locs.iter().skip(i + 1) {
                unique_antinodes.extend(calculate_antinodes(loc1, loc2, &size));
            }
        }
    }
    unique_antinodes.len()
}
#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (signals, size) = parse(lines);
    find_nr_antinodes(signals, size)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 14);
    }
}
