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

fn successors(pos: &Coord, walls: &HashSet<Coord>, size: Size, can_cheat: bool) -> Vec<Coord> {
    let mut successors: Vec<Coord> = Vec::new();
    for dir in [NORTH, EAST, SOUTH, WEST] {
        let new_pos = *pos + dir;
        if new_pos.x < 0 || new_pos.x > size.0 || new_pos.y < 0 || new_pos.y > size.1 {
            continue;
        }
        if walls.contains(&new_pos) && !can_cheat {
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
    can_cheat: bool,
    threshold: i32,
    cache: &Option<HashMap<Coord, i32>>,
) -> i32 {
    if !can_cheat {
        if let Some(cache_val) = cache {
            if let Some(value) = cache_val.get(pos) {
                let length = visited.len() as i32 - 1 + *value;
                if threshold >= length {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
    }
    if visited.contains(pos) || (threshold < visited.len() as i32 && threshold > 0) {
        return 0;
    }
    visited.push(*pos);
    if *pos == map.goal {
        result.push(visited);
        return 1;
    }
    let mut sum = 0;
    for new_pos in successors(pos, &map.walls, map.size, can_cheat) {
        if map.walls.contains(&new_pos) {
            sum += walk(
                &new_pos,
                map,
                visited.clone(),
                result,
                false,
                threshold,
                cache,
            );
        } else {
            sum += walk(
                &new_pos,
                map,
                visited.clone(),
                result,
                can_cheat,
                threshold,
                cache,
            );
        }
    }
    sum
}

fn get_diamond() -> Vec<Coord> {
    let mut positions = Vec::new();
    for x in -20..21i32 {
        for y in 0..(21 - x.abs()) {
            positions.push(Coord::new(x, y));
            positions.push(Coord::new(x, -y));
        }
    }
    positions
}

fn is_in_map(pos: &Coord, map: &Map) -> bool {
    if pos.x < 0 || pos.x >= map.size.0 || pos.y < 0 || pos.y >= map.size.1 {
        return false;
    }
    true
}

fn find_cheats(path: HashMap<Coord, i32>, map: &Map, threshold: i32) -> Vec<i32> {
    let mut cheats = Vec::new();
    let diamond = get_diamond();
    for (pos, value) in path.iter() {
        for diamond_pos in diamond.iter() {
            let new_pos = *pos + *diamond_pos;
            if !is_in_map(&new_pos, map) {
                continue;
            }
            if let Some(new_value) = path.get(&new_pos) {
                if new_value < value {
                    continue;
                }
                let score = new_value - value - (diamond_pos.x.abs() + diamond_pos.y.abs());
                if score >= threshold {
                    cheats.push(score);
                }
            } else {
                continue;
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
    walk(&map.start, &map, Vec::new(), &mut result, false, -1, &None);
    let full_race = (result[0].len() - 1) as i32;
    let mut cache: HashMap<Coord, i32> = HashMap::new();
    for (i, pos) in result[0].clone().into_iter().rev().enumerate() {
        cache.insert(pos, i as i32);
    }
    let cheats = find_cheats(cache, &map, 100);
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
