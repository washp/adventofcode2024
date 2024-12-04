use crate::utils::Vector2D;

type Coord = Vector2D<isize>;

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn in_bounds(pos: &Coord, size: (usize, usize)) -> bool {
    if pos.x <= 0 || pos.x + 1 >= size.0 as isize || pos.y <= 0 || pos.y + 1 >= size.1 as isize {
        return false;
    }
    true
}

fn find_xmas(pos: Coord, map: &[Vec<char>]) -> bool {
    let height = map.len();
    let width = map.first().expect("No first row!").len();
    if !in_bounds(&pos, (width, height)) {
        return false;
    }
    let ul = pos + Coord::new(-1, -1);
    let lr = pos + Coord::new(1, 1);
    let mas1 = (map[ul.y as usize][ul.x as usize] == 'M'
        && map[lr.y as usize][lr.x as usize] == 'S')
        || (map[ul.y as usize][ul.x as usize] == 'S' && map[lr.y as usize][lr.x as usize] == 'M');
    let ur = pos + Coord::new(1, -1);
    let ll = pos + Coord::new(-1, 1);
    let mas2 = (map[ur.y as usize][ur.x as usize] == 'M'
        && map[ll.y as usize][ll.x as usize] == 'S')
        || (map[ur.y as usize][ur.x as usize] == 'S' && map[ll.y as usize][ll.x as usize] == 'M');
    mas1 && mas2
}

fn traverse(chars: Vec<Vec<char>>) -> usize {
    let height = chars.len();
    let width = chars.first().expect("No first row!").len();
    let mut hits = 0;
    for row in 0..height {
        for col in 0..width {
            if chars[row][col] == 'A' && find_xmas(Coord::new(col as isize, row as isize), &chars) {
                hits += 1;
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
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 9);
    }
}
