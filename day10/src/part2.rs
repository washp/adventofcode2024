use std::collections::{HashMap, VecDeque};

use crate::utils::Vector2D;

type Coord = Vector2D<isize>;
type Map = HashMap<Coord, usize>;

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Map, Vec<Coord>) {
    let mut map: HashMap<Coord, usize> = HashMap::new();
    let mut start = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Coord::new(x as isize, y as isize);
            let height = char.to_digit(10).expect("Char is not a digit") as usize;
            if height == 0 {
                start.push(pos);
            }
            map.insert(pos, height);
        }
    }
    (map, start)
}

fn get_possible_steps(cur_pos: &Coord, map: &Map) -> Vec<Coord> {
    let cur_height = map.get(cur_pos).expect("No height at current pos");
    let dirs = vec![
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(-1, 0),
        Coord::new(0, -1),
    ];
    dirs.into_iter().fold(Vec::new(), |mut dirs, x| {
        if let Some(height) = map.get(&(*cur_pos + x)) {
            if *height == cur_height + 1 {
                dirs.push(*cur_pos + x)
            }
        }
        dirs
    })
}

fn find_trails(map: Map, start: Vec<Coord>) -> usize {
    let mut pos_paths: VecDeque<Vec<Coord>> = start.iter().map(|x| vec![*x]).collect();
    let mut trails = Vec::new();
    while let Some(path) = pos_paths.pop_front() {
        let cur_pos = path.last().expect("Path is empty!");
        for step in get_possible_steps(cur_pos, &map).iter_mut() {
            let height = map.get(step).expect("No height at pos");
            let mut new_path = path.clone();
            new_path.push(*step);
            if *height == 9 {
                trails.push(new_path);
            } else {
                pos_paths.push_back(new_path);
            }
        }
    }
    trails.len()
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (map, start) = parse(lines);
    find_trails(map, start)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 81);
    }
}
