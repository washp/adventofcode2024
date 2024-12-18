use std::collections::HashSet;

use crate::utils::Vector2D;

type Coord = Vector2D<i32>;

struct Map {
    boxes: HashSet<Coord>,
    robot: Coord,
    walls: HashSet<Coord>,
}

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Map, Vec<Coord>) {
    let mut map = Map {
        boxes: HashSet::new(),
        walls: HashSet::new(),
        robot: Coord::new(0, 0),
    };
    let mut dirs: Vec<Coord> = Vec::new();

    let mut iter = lines.iter();
    for (y, line) in iter.by_ref().enumerate() {
        if line.trim().is_empty() {
            break;
        }
        for (x, char) in line.chars().enumerate() {
            let pos = Coord::new(x as i32, y as i32);
            match char {
                '@' => map.robot = pos,
                '#' => _ = map.walls.insert(pos),
                'O' => _ = map.boxes.insert(pos),
                _ => (),
            }
        }
    }
    for line in iter {
        for char in line.chars() {
            let dir = match char {
                '^' => NORTH,
                '>' => EAST,
                '<' => WEST,
                'v' => SOUTH,
                _ => Coord::new(0, 0),
            };
            dirs.push(dir);
        }
    }
    (map, dirs)
}

fn move_item(pos: Coord, dir: Coord, map: &mut Map) -> Result<(), ()> {
    if map.walls.contains(&pos) {
        return Err(());
    }
    if !map.boxes.contains(&pos) && pos != map.robot {
        return Ok(());
    }
    let new_pos = pos + dir;
    move_item(new_pos, dir, map)?;
    if pos == map.robot {
        map.robot = new_pos;
    } else {
        map.boxes.remove(&pos);
        map.boxes.insert(new_pos);
    }
    Ok(())
}

#[allow(unused_variables)]
pub fn run(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let (mut map, dirs) = parse(lines);
    for dir in dirs {
        let res = move_item(map.robot, dir, &mut map);
    }
    map.boxes.iter().map(|x| x.y * 100 + x.x).sum()
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_small() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 2028);
    }

    //#[ignore]
    #[test]
    fn test_big() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 10092);
    }
}
