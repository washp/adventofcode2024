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
fn parse(lines: Vec<&str>) -> (Map, Vec<Coord>, (usize, usize)) {
    let mut map = Map {
        boxes: HashSet::new(),
        walls: HashSet::new(),
        robot: Coord::new(0, 0),
    };
    let mut dirs: Vec<Coord> = Vec::new();
    let mut height = 0;
    let mut width = 0;

    let mut iter = lines.iter();
    for (y, line) in iter.by_ref().enumerate() {
        if y == 0 && line.trim().is_empty() {
            continue;
        }
        if line.trim().is_empty() {
            break;
        }
        height = y;
        for (x, char) in line.chars().enumerate() {
            let pos = Coord::new((x * 2) as i32, y as i32);
            match char {
                '@' => {
                    map.robot = pos;
                }
                '#' => {
                    map.walls.insert(pos);
                    map.walls.insert(pos + EAST);
                }

                'O' => {
                    map.boxes.insert(pos);
                }
                _ => (),
            }
            width = x;
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
    (map, dirs, ((width + 1) * 2, height + 1))
}

fn get_box_pos(pos: &Coord, boxes: &HashSet<Coord>) -> Result<Coord, ()> {
    if boxes.contains(pos) {
        return Ok(*pos);
    }
    let new_pos = *pos + WEST;
    if boxes.contains(&new_pos) {
        return Ok(new_pos);
    }
    Err(())
}

fn move_item(pos: &Coord, dir: Coord, map: &mut Map, dry_run: bool) -> Result<(), ()> {
    if map.walls.contains(pos) {
        return Err(());
    }
    let is_box = get_box_pos(pos, &map.boxes).is_ok();
    if !is_box && map.robot != *pos {
        return Ok(());
    }
    let mut new_pos = *pos + dir;
    if dir == EAST && is_box {
        new_pos = new_pos + EAST;
    }
    let mut new_box_res = get_box_pos(&new_pos, &map.boxes);
    let mut second_box: Result<Coord, ()> = Err(());
    if is_box && (dir == NORTH || dir == SOUTH) {
        if map.walls.contains(&new_pos) || map.walls.contains(&(new_pos + EAST)) {
            return Err(());
        }
        if new_box_res.is_err() {
            new_box_res = get_box_pos(&(new_pos + EAST), &map.boxes);
        } else if new_box_res.unwrap() != new_pos {
            second_box = get_box_pos(&(new_pos + EAST), &map.boxes);
        }
    }
    let mut second_box_ok = false;
    if let Ok(pos2) = second_box {
        move_item(&pos2, dir, map, true)?;
        second_box_ok = true;
    }
    if let Ok(new_box_pos) = new_box_res {
        move_item(&new_box_pos, dir, map, dry_run)?;
        if second_box_ok {
            move_item(&second_box.unwrap(), dir, map, dry_run)?;
        }
    } else {
        move_item(&new_pos, dir, map, dry_run)?;
    }
    if !dry_run {
        if *pos == map.robot {
            map.robot = new_pos;
        } else {
            map.boxes.remove(pos);
            map.boxes.insert(*pos + dir);
        }
    }
    Ok(())
}

fn draw_map(map: &Map, dir: &Coord, (width, height): (usize, usize)) {
    for y in 0..height {
        let mut box_started = false;
        for x in 0..width {
            let pos = Coord::new(x as i32, y as i32);
            if box_started {
                print!("]");
                box_started = false;
                continue;
            }

            if map.robot == pos {
                match *dir {
                    NORTH => print!("^"),
                    EAST => print!(">"),
                    WEST => print!("<"),
                    SOUTH => print!("v"),
                    _ => print!("@"),
                };
            } else if map.walls.contains(&pos) {
                print!("#");
            } else if map.boxes.contains(&pos) {
                print!("[");
                box_started = true;
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(unused_variables)]
pub fn run(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let (mut map, dirs, size) = parse(lines);
    println!("dirs: {}", dirs.len());
    for (i, dir) in dirs.iter().enumerate() {
        let res = move_item(&(map.robot.clone()), *dir, &mut map, false);
    }
    draw_map(&map, &Coord::new(0, 0), size);
    map.boxes.iter().map(|x| x.y * 100 + x.x).sum()
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[ignore]
    #[test]
    fn test_small() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 2028);
    }

    #[ignore]
    #[test]
    fn test_big() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 9021);
    }

    #[ignore]
    #[test]
    fn test_mid() {
        let example_content = fs::read_to_string("example_input3.txt").unwrap();
        assert_eq!(run(&example_content), 10092);
    }

    //#[ignore]
    #[test]
    fn test_up_pyramid() {
        let example_content = "
######
#....#
##O..#
#.OO@#
#.O..#
#....#
######

<vv<<^
";
        run(example_content);
    }
    #[ignore]
    #[test]
    fn test_down_pyramid() {
        let example_content = "
#######
#.....#
#.....#
#..O@.#
#..OO.#
#..#..#
#######

>>v<^^<<v
";
        run(example_content);
    }
}
