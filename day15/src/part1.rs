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
    let mut iter = lines.iter();
    for line in iter.by_ref() {
        if line.trim().is_empty() {
            break;
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
