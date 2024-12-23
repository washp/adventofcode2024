use std::collections::HashSet;

use crate::utils::Vector2D;
use pathfinding::prelude::bfs;

type Coord = Vector2D<i32>;

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

#[allow(unused_variables)]
fn parse(lines: Vec<&str>, limit: usize) -> HashSet<Coord> {
    lines[..limit]
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
pub fn run(input: &str, size: usize) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let corrupted = parse(lines, 1024);
    let start = Coord::new(0, 0);
    let goal = Coord::new(size as i32, size as i32);
    let result = bfs(
        &start,
        |pos| successors(pos, &corrupted, size),
        |pos| *pos == goal,
    );
    result.expect("No path found!").len() - 1
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content, 6), 22);
    }
}
