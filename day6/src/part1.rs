use std::collections::HashSet;

use crate::utils::Vector2D;

type Coord = Vector2D<isize>;

const NORTH: Coord = Coord::new(0, -1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, 1);
const WEST: Coord = Coord::new(-1, 0);

struct ParseResult(Coord, HashSet<Coord>, (usize, usize));

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> ParseResult {
    let mut obstacles = HashSet::new();
    let mut start_pos = Coord::new(0, 0);
    let height = lines.len();
    let width = lines.first().expect("No first row...").len();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                obstacles.insert(Coord::new(x as isize, y as isize));
            } else if c == '^' {
                start_pos = Coord::new(x as isize, y as isize);
            }
        }
    }
    ParseResult(start_pos, obstacles, (width, height))
}

fn is_on_map(pos: &Coord, (width, height): &(usize, usize)) -> bool {
    pos.x >= 0 && pos.x < *width as isize && pos.y >= 0 && pos.y < *height as isize
}

fn walk(start_pos: &Coord, obstacles: &HashSet<Coord>, map_size: &(usize, usize)) -> usize {
    let mut cur_dir = NORTH;
    let mut cur_pos = *start_pos;
    let mut steps = HashSet::new();
    while is_on_map(&cur_pos, map_size) {
        steps.insert(cur_pos);
        let new_pos = cur_pos + cur_dir;
        if obstacles.contains(&new_pos) {
            cur_dir = match cur_dir {
                NORTH => EAST,
                EAST => SOUTH,
                SOUTH => WEST,
                WEST => NORTH,
                _ => NORTH,
            };
        } else {
            cur_pos = new_pos;
        }
    }
    steps.len()
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let ParseResult(start_pos, obstacles, map_size) = parse(lines);
    walk(&start_pos, &obstacles, &map_size)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 41);
    }
}
