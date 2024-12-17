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

fn count_robots(robots: &Vec<Robot>, size: (i32, i32)) -> usize {
    let mut results = [0, 0, 0, 0];
    let x_border = (size.0) / 2;
    let y_border = (size.1) / 2;
    for robot in robots {
        if robot.pos.x == x_border || robot.pos.y == y_border {
            continue;
        }
        if robot.pos.x < x_border {
            if robot.pos.y < y_border {
                results[0] += 1;
            } else {
                results[2] += 1;
            }
        } else if robot.pos.y < y_border {
            results[1] += 1;
        } else {
            results[3] += 1;
        }
    }
    results
        .into_iter()
        .reduce(|a, b| a * b)
        .expect("No value from quadrants")
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let mut size = (101, 103);
    let steps = 100;
    let lines = input.lines().collect::<Vec<_>>();
    let mut robots = parse(lines);
    if robots.len() < 15 {
        size = (11, 7)
    }
    robots = walk_robots(robots, steps, size);
    count_robots(&robots, size)
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
