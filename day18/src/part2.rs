use std::collections::HashSet;

use crate::utils::Vector2D;
use pathfinding::prelude::bfs;

type Coord = Vector2D<i32>;

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<Coord> {
    lines
        .iter()
        .map(|x| {
            let mut iter = x.split(",");
            let x = iter
                .next()
                .expect("No X value on line")
                .parse::<i32>()
                .expect("X not a number");
            let y = iter
                .next()
                .expect("No Y value on line")
                .parse::<i32>()
                .expect("Y not a number");
            Coord::new(x, y)
        })
        .collect()
}

fn successors(pos: &Coord, corrupted: &HashSet<Coord>, size: usize) -> Vec<Coord> {
    let mut successors: Vec<Coord> = Vec::new();
    for dir in [NORTH, EAST, SOUTH, WEST] {
        let new_pos = *pos + dir;
        if new_pos.x < 0 || new_pos.x > size as i32 || new_pos.y < 0 || new_pos.y > size as i32 {
            continue;
        }
        if corrupted.contains(&new_pos) {
            continue;
        }
        successors.push(new_pos);
    }
    successors
}

#[allow(unused_variables)]
pub fn run(input: &str, size: usize) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let mut all_bytes = parse(lines);
    let remaining = all_bytes.split_off(12);
    let mut corrupted = HashSet::from_iter(all_bytes);
    let start = Coord::new(0, 0);
    let goal = Coord::new(size as i32, size as i32);
    let result = bfs(
        &start,
        |pos| successors(pos, &corrupted, size),
        |pos| *pos == goal,
    );
    let mut path: HashSet<Coord> = HashSet::from_iter(result.expect("No path"));
    for pos in remaining {
        corrupted.insert(pos);
        if !path.contains(&pos) {
            continue;
        }
        let result = bfs(
            &start,
            |pos| successors(pos, &corrupted, size),
            |pos| *pos == goal,
        );
        if let Some(new_path) = result {
            path = HashSet::from_iter(new_path);
        } else {
            return format!("({},{})", pos.x, pos.y);
        }
    }
    format!("({},{})", 0, 0)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content, 6), "(6,1)");
    }
}
