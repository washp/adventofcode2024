use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::utils::Vector2D;

type Vector = Vector2D<i32>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Robot {
    pos: Vector,
    dir: Vector,
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    for line in lines {
        let re = Regex::new(r"[-\d]+").unwrap();
        let result: Vec<_> = re
            .find_iter(line)
            .map(|x| {
                x.as_str()
                    .parse::<i32>()
                    .expect("Not a number, wrong in regex")
            })
            .collect();
        robots.push(Robot {
            pos: Vector::new(result[0], result[1]),
            dir: Vector::new(result[2], result[3]),
        });
    }
    robots
}

fn walk_robots(mut robots: Vec<Robot>, steps: i32, size: (i32, i32)) -> Vec<Robot> {
    for robot in robots.iter_mut() {
        robot.pos.x = (robot.pos.x + robot.dir.x * steps) % size.0;
        robot.pos.y = (robot.pos.y + robot.dir.y * steps) % size.1;
        if robot.pos.x < 0 {
            robot.pos.x += size.0;
        }
        if robot.pos.y < 0 {
            robot.pos.y += size.1;
        }
    }
    robots
}

fn is_xmas_tree(robots: &Vec<Robot>, size: (i32, i32)) -> bool {
    let mut robot_pos: HashSet<Vector> = HashSet::new();
    let mut row_count: HashMap<usize, usize> = HashMap::new();
    for robot in robots {
        robot_pos.insert(robot.pos);
        if let Some(value) = row_count.get(&(robot.pos.y as usize)) {
            row_count.insert(robot.pos.y as usize, value + 1);
        } else {
            row_count.insert(robot.pos.y as usize, 0);
        }
    }
    for y in 0..size.1 {
        let count = row_count.get(&(y as usize));
        if count.is_none() || *count.unwrap() < 10 {
            continue;
        }
        let mut in_row = 0;
        for x in 0..size.0 {
            if robot_pos.contains(&Vector::new(x, y)) {
                in_row += 1;
                if in_row > 10 {
                    return true;
                }
            } else {
                in_row = 0;
            }
        }
    }
    false
}

fn print_tree(robots: &Vec<Robot>, size: (i32, i32)) {
    let mut robot_pos: HashSet<Vector> = HashSet::new();
    for robot in robots {
        robot_pos.insert(robot.pos);
    }
    for y in 0..size.1 {
        for x in 0..size.0 {
            if robot_pos.contains(&Vector::new(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let mut size = (101, 103);
    let lines = input.lines().collect::<Vec<_>>();
    let mut robots = parse(lines);
    if robots.len() < 15 {
        size = (11, 7)
    }
    let mut i = 0;
    while !is_xmas_tree(&robots, size) {
        robots = walk_robots(robots, 1, size);
        i += 1;
        if i > 10000 {
            println!("Timeout");
            break;
        }
    }
    print_tree(&robots, size);

    i
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 12);
    }
}
