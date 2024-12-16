use crate::utils::Vector2D;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

type Coord = Vector2D<u64>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Machine {
    btn_a: Coord,
    btn_b: Coord,
    target: Coord,
}

fn parse_line(line: &str, is_target: bool) -> Coord {
    let mut btn_iter = line
        .split(":")
        .nth(1)
        .expect("No text right of colon on first line in segment")
        .split(",");
    let divider = match is_target {
        false => "+",
        true => "=",
    };
    let x = btn_iter
        .next()
        .expect("No text for value 1")
        .split(divider)
        .nth(1)
        .expect("No value for value 1")
        .parse::<usize>()
        .expect("value 1 is not a number");
    let y = btn_iter
        .next()
        .expect("No text for value 2")
        .split(divider)
        .nth(1)
        .expect("No value for value 2")
        .parse::<usize>()
        .expect("value 2 is not a number");
    Coord::new(x as u64, y as u64)
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<Machine> {
    let mut iter = lines.iter();
    let mut machines: Vec<Machine> = Vec::new();
    while let Some(line) = iter.next() {
        let btn_a = parse_line(line, false);
        let btn_b = parse_line(iter.next().expect("No line for button B"), false);
        let target = parse_line(iter.next().expect("No line for Target"), true);
        machines.push(Machine {
            btn_a,
            btn_b,
            target,
        });
    }
    machines
}

#[allow(unused_variables)]
pub fn run(input: &str) -> u64 {
    let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let machines = parse(lines);
    let mut wins = Vec::new();
    for machine in machines {
        let a: Array2<f64> = array![
            [machine.btn_a.x as f64, machine.btn_b.x as f64],
            [machine.btn_a.y as f64, machine.btn_b.y as f64]
        ];
        let b: Array1<f64> = array![machine.target.x as f64, machine.target.y as f64];
        let x = a.solve_into(b);
        if let Ok(values) = x {
            let mut num_x = (values[0] * 1000.).round() as u64;
            num_x = match num_x % 1000 {
                0 => num_x / 1000,
                _ => 0,
            };
            let mut num_y = (values[1] * 1000.).round() as u64;
            num_y = match num_y % 1000 {
                0 => num_y / 1000,
                _ => 0,
            };

            let tokens = num_x * 3 + num_y;
            if num_x < 100 && num_y < 100 {
                let tokens = num_x * 3 + num_y;
                wins.push(num_x * 3 + num_y)
            } else {
                wins.push(0);
            }
        }
    }
    wins.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 480);
    }

    #[test]
    fn test_test() {
        let example_content = fs::read_to_string("example_input3.txt").unwrap();
        assert_eq!(run(&example_content), 357);
    }
    #[test]
    fn test_test2() {
        let example_content = fs::read_to_string("example_input4.txt").unwrap();
        assert_eq!(run(&example_content), 134);
    }
}
