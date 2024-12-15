use crate::utils::Vector2D;

type Coord = Vector2D<u32>;

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
    Coord::new(x as u32, y as u32)
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
pub fn run(input: &str) -> u32 {
    let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let machines = parse(lines);
    let mut wins: Vec<u32> = Vec::new();
    for machine in machines {
        let mut cur_vec = Coord::new(0, 0);
        let mut pos_switch: Vec<Coord> = Vec::new();
        let mut i = 0;
        while cur_vec.x <= machine.target.x && cur_vec.y <= machine.target.y {
            if i > 100 {
                break;
            }
            let target_remain = machine.target - cur_vec;
            if target_remain.x % machine.btn_b.x == 0 && target_remain.y % machine.btn_b.y == 0 {
                pos_switch.push(cur_vec);
            }
            cur_vec = cur_vec + machine.btn_a;
            i += 1;
        }
        let mut least_tokens = 0;
        for switch in pos_switch.iter() {
            let a_press = switch.x / machine.btn_a.x;
            let b_press = (machine.target.x - switch.x) / machine.btn_b.x;
            if b_press > 100
                || a_press * machine.btn_a.y + b_press * machine.btn_b.y != machine.target.y
            {
                continue;
            }
            let tokens = a_press * 3 + b_press;
            if least_tokens == 0 || least_tokens > tokens {
                least_tokens = tokens;
            }
        }
        wins.push(least_tokens);
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
}
