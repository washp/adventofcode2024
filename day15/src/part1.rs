use std::collections::HashSet;

use crate::utils::Vector2D;

type Coord = Vector2D<i32>;

struct Map {
    boxes: HashSet<Coord>,
    robot: Coord,
    walls: HashSet<Coord>,
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) {
    let mut map = Map {
        boxes: HashSet::new(),
        walls: HashSet::new(),
        robot: Coord::new(0, 0),
    };
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
        println!("{}", line);
    }
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    parse(lines);
    0
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

    #[test]
    fn test_big() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 10092);
    }
}
