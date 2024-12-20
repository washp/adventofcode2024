use std::collections::{HashMap, HashSet};

use crate::utils::Vector2D;

type Coord = Vector2D<isize>;
type Map = HashMap<Coord, char>;

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Result {
    area: usize,
    fence: usize,
}

fn get_4_dirs() -> Vec<Coord> {
    vec![NORTH, EAST, SOUTH, WEST]
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Map, (usize, usize)) {
    let mut map: Map = HashMap::new();
    let height = lines.len();
    let width = lines.first().expect("No first line!").len();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Coord::new(x as isize, y as isize);
            map.insert(pos, char);
        }
    }
    (map, (width, height))
}

fn walk_pos(pos: &Coord, map: &Map, processed: &mut HashSet<Coord>, mut result: Result) -> Result {
    let cur_char = map
        .get(pos)
        .expect("No char at pos, something is not right with the position");
    if processed.contains(pos) {
        return result;
    }
    result.area += 1;
    processed.insert(*pos);
    for dir in get_4_dirs() {
        let new_pos = *pos + dir;
        let new_char = map.get(&new_pos);
        if new_char.is_some() && new_char.unwrap() == cur_char {
            result = walk_pos(&new_pos, map, processed, result);
        } else {
            result.fence += 1;
        }
    }
    result
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (map, size) = parse(lines);
    let mut processed: HashSet<Coord> = HashSet::new();
    let mut results = Vec::new();
    for y in 0..size.1 {
        for x in 0..size.0 {
            let pos = Coord::new(x as isize, y as isize);
            if processed.contains(&pos) {
                continue;
            }
            let result = walk_pos(&pos, &map, &mut processed, Result { area: 0, fence: 0 });
            results.push(result);
        }
    }
    results.iter().map(|x| x.area * x.fence).sum()
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_example1() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 140);
    }

    #[test]
    fn test_example2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 1930);
    }
}
