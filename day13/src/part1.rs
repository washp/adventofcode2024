#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Machine {
    btn_a: (usize, usize),
    btn_b: (usize, usize),
    target: (usize, usize),
}

fn parse_line(line: &str, is_target: bool) -> (usize, usize) {
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
    (x, y)
}

fn sort_options(machine: &Machine) -> ((usize, usize), (usize, usize)) {
    if machine.btn_a.0 + machine.btn_a.1 > machine.btn_b.0 + machine.btn_b.1 {
        return (machine.btn_a, machine.btn_b);
    }
    (machine.btn_b, machine.btn_a)
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
pub fn run(input: &str) -> usize {
    let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let machines = parse(lines);
    for machine in machines {
        println!("{:?}", machine);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 10);
    }
}
