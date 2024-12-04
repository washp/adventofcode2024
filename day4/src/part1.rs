use crate::utils::Vector2D;

type Coord = Vector2D<isize>;

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn in_bounds(pos: &Coord, size: (usize, usize)) -> bool {
    pos.x >= 0 && pos.x < size.0 as isize && pos.y >= 0 && pos.y < size.1 as isize
}

fn find_xmas(mut pos: Coord, dir: Vector2D<isize>, map: &[Vec<char>]) -> bool {
    let height = map.len();
    let width = map.first().expect("No first row!").len();
    if map[pos.y as usize][pos.x as usize] != 'X' {
        return false;
    }
    pos = pos + dir;
    if !in_bounds(&pos, (width, height)) || map[pos.y as usize][pos.x as usize] != 'M' {
        return false;
    }
    pos = pos + dir;
    if !in_bounds(&pos, (width, height)) || map[pos.y as usize][pos.x as usize] != 'A' {
        return false;
    }
    pos = pos + dir;
    if !in_bounds(&pos, (width, height)) || map[pos.y as usize][pos.x as usize] != 'S' {
        return false;
    }
    true
}

fn traverse(chars: Vec<Vec<char>>) -> usize {
    let height = chars.len();
    let width = chars.first().expect("No first row!").len();
    let mut hits = 0;
    let all_dirs = [
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(0, 1),
        Coord::new(-1, 0),
        Coord::new(-1, -1),
        Coord::new(0, -1),
        Coord::new(-1, 1),
        Coord::new(1, -1),
    ];
    for row in 0..height {
        for col in 0..width {
            if chars[row][col] == 'X' {
                for dir in all_dirs {
                    if find_xmas(Coord::new(col as isize, row as isize), dir, &chars) {
                        hits += 1;
                    }
                }
            }
        }
    }
    hits
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let chars = parse(lines);
    traverse(chars)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 18);
    }
}
