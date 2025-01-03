use std::collections::{HashMap, HashSet};

use crate::utils::Vector2D;

type Coord = Vector2D<i32>;
type Size = (i32, i32);

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

struct Map {
    start: Coord,
    goal: Coord,
    walls: HashSet<Coord>,
    size: Size,
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Map {
    let height = lines.len();
    let mut width = 0;
    let mut start = Coord::new(0, 0);
    let mut goal = Coord::new(0, 0);
    let mut walls: HashSet<Coord> = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            width = x + 1;
            let pos = Coord::new(x as i32, y as i32);
            match char {
                '#' => {
                    walls.insert(pos);
                }
                'S' => {
                    start = pos;
                }
                'E' => {
                    goal = pos;
                }
                _ => (),
            }
        }
    }
    Map {
        start,
        goal,
        walls,
        size: (width as i32, height as i32),
    }
}

fn successors(pos: &Coord, walls: &HashSet<Coord>, size: Size) -> Vec<Coord> {
    let mut successors: Vec<Coord> = Vec::new();
    for dir in [NORTH, EAST, SOUTH, WEST] {
        let new_pos = *pos + dir;
        if new_pos.x < 0 || new_pos.x > size.0 || new_pos.y < 0 || new_pos.y > size.1 {
            continue;
        }
        if walls.contains(&new_pos) {
            continue;
        }
        successors.push(new_pos);
    }
    successors
}

#[allow(dead_code)]
fn draw(visited: &HashSet<Coord>, walls: &HashSet<Coord>, size: Size) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let pos = Coord::new(x, y);
            if visited.contains(&pos) {
                print!("*");
            } else if walls.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn walk(
    pos: &Coord,
    map: &Map,
    mut visited: Vec<Coord>,
    result: &mut Vec<Vec<Coord>>,
    threshold: i32,
) -> i32 {
    if visited.contains(pos) || (threshold < visited.len() as i32 && threshold > 0) {
        return 0;
    }
    visited.push(*pos);
    if *pos == map.goal {
        result.push(visited);
        return 1;
    }
    let mut sum = 0;
    for new_pos in successors(pos, &map.walls, map.size) {
        sum += walk(&new_pos, map, visited.clone(), result, threshold);
    }
    sum
}

fn find_cheats(path: &[Coord], threshold: i32) -> Vec<i32> {
    let mut cheats = Vec::new();
    for (i, pos) in path.iter().enumerate() {
        for a in (i + 3)..path.len() {
            if let Some(new_pos) = path.get(a) {
                let diff = *new_pos - *pos;
                let dist = diff.x.abs() + diff.y.abs();
                if dist <= 20 {
                    let gain = a as i32 - i as i32 - dist;
                    if gain >= threshold {
                        cheats.push(gain);
                    }
                }
            }
        }
    }
    cheats
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let map = parse(lines);
    let mut result = Vec::new();
    walk(&map.start, &map, Vec::new(), &mut result, -1);
    let full_race = (result[0].len() - 1) as i32;
    let mut cache: HashMap<Coord, i32> = HashMap::new();
    for (i, pos) in result[0].clone().into_iter().rev().enumerate() {
        cache.insert(pos, i as i32);
    }
    let cheats = find_cheats(&result[0], 100);
    cheats.len()
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 10);
    }
}
