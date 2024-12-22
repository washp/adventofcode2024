use std::collections::{HashMap, HashSet};

use crate::utils::Vector2D;

type Coord = Vector2D<i32>;
type Paths = HashSet<Vector2D<i32>>;

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Paths, (Coord, Coord)) {
    let mut paths = HashSet::new();
    let mut start = Coord::new(0, 0);
    let mut goal = Coord::new(0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Coord::new(x as i32, y as i32);
            match char {
                '.' => {
                    paths.insert(pos);
                }
                'S' => {
                    start = pos;
                    paths.insert(pos);
                }
                'E' => {
                    paths.insert(pos);
                    goal = pos;
                }
                _ => (),
            };
            if char == '.' {}
        }
    }
    (paths, (start, goal))
}

#[allow(dead_code)]
fn draw(paths: &Paths, trail: &HashSet<Coord>) {
    for y in 0..15 {
        for x in 0..15 {
            let pos = Coord::new(x, y);
            if !paths.contains(&pos) {
                print!("#");
            } else if trail.contains(&pos) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn walk(start: Coord, start_dir: Coord, goal: Coord, valid: &Paths) -> usize {
    let mut start_trail = HashSet::new();
    start_trail.insert(start);
    let initial = (start, start_dir, start_trail, 0);
    let mut paths: Vec<(Coord, Coord, HashSet<Coord>, u32)> = vec![initial];
    let mut visited: HashMap<(Coord, Coord), u32> = HashMap::new();
    let mut optimal_paths: Vec<HashSet<Coord>> = Vec::new();
    let mut optimal_score = 0;
    while let Some(path) = paths.pop() {
        let (pos, dir, trail, score) = path;
        if optimal_score > score {
            continue;
        }
        if pos == goal {
            optimal_score = score;
            optimal_paths.push(trail.clone());
        }
        let vis = visited.get(&(dir, pos));
        if vis.unwrap_or(&102489) < &score {
            continue;
        }
        visited.insert((dir, pos), score);
        for new_dir in [NORTH, EAST, SOUTH, WEST] {
            if new_dir == Coord::new(-dir.x, -dir.y) {
                continue;
            }
            let new_pos = pos + new_dir;
            if trail.contains(&(new_pos)) || !valid.contains(&new_pos) {
                continue;
            }
            let mut new_trail = trail.clone();
            new_trail.insert(new_pos);
            let mut new_score = score;
            if new_dir == dir {
                new_score += 1;
            } else {
                new_score += 1001;
            }
            paths.push((new_pos, new_dir, new_trail, new_score))
        }
        paths.sort_by(|a, b| b.3.cmp(&a.3));
    }
    let mut combined: HashSet<Coord> = HashSet::new();
    for path in optimal_paths.iter() {
        combined.extend(path);
    }
    combined.len()
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let start_dir = EAST;
    let trail: HashSet<Coord> = HashSet::new();
    let (paths, (start, goal)) = parse(lines);
    walk(start, start_dir, goal, &paths)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_small() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 45);
    }

    #[test]
    fn test_big() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 64);
    }
}
