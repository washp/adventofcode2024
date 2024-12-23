struct State {
    a: u32,
    b: u32,
    c: u32,
    inst_point: u32,
    result: Vec<u32>,
}
type CmdFunc = fn(u32, &mut State) -> bool;

fn get_combo(op: u32, state: &State) -> u32 {
    match op {
        1..=3 => op,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => 0,
    }
}

fn adv(operand: u32, state: &mut State) -> bool {
    state.a /= 2u32.pow(get_combo(operand, state));
    true
}
fn bxl(operand: u32, state: &mut State) -> bool {
    state.b ^= operand;
    true
}
fn bst(operand: u32, state: &mut State) -> bool {
    state.b = get_combo(operand, state) % 8;
    true
}
fn jnz(operand: u32, state: &mut State) -> bool {
    if state.a != 0 {
        state.inst_point = operand / 2;
        return false;
    }
    true
}
fn bxc(_: u32, state: &mut State) -> bool {
    state.b ^= state.c;
    true
}
fn out(operand: u32, state: &mut State) -> bool {
    state.result.push(get_combo(operand, state) % 8);
    true
}
fn bdv(operand: u32, state: &mut State) -> bool {
    state.b = state.a / 2u32.pow(get_combo(operand, state));
    true
}
fn cdv(operand: u32, state: &mut State) -> bool {
    state.c = state.a / 2u32.pow(get_combo(operand, state));
    true
}

fn noop(_: u32, _: &mut State) -> bool {
    panic!("Unknown command!");
}

fn get_number(line: &str) -> u32 {
    line.split(":")
        .nth(1)
        .expect("No part after : on line")
        .trim()
        .parse::<u32>()
        .expect("Second part of line is not a number")
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (State, Vec<(CmdFunc, u32)>) {
    let mut iter = lines.iter();
    let a = get_number(iter.next().expect("No first row"));
    let b = get_number(iter.next().expect("No second row"));
    let c = get_number(iter.next().expect("No third row"));
    let reg = State {
        a,
        b,
        c,
        inst_point: 0,
        result: Vec::new(),
    };
    let prog_iter = iter
        .nth(1)
        .expect("No program row")
        .trim()
        .split(":")
        .nth(1)
        .expect("No second part of program row")
        .split(",")
        .map(|x| x.trim().parse::<u32>().expect("Command not a number"));
    let program = prog_iter
        .clone()
        .step_by(2)
        .zip(prog_iter.skip(1).step_by(2))
        .map(|x| match x.0 {
            0 => (adv as CmdFunc, x.1),
            1 => (bxl as CmdFunc, x.1),
            2 => (bst as CmdFunc, x.1),
            3 => (jnz as CmdFunc, x.1),
            4 => (bxc as CmdFunc, x.1),
            5 => (out as CmdFunc, x.1),
            6 => (bdv as CmdFunc, x.1),
            7 => (cdv as CmdFunc, x.1),
            _ => (noop as CmdFunc, 0),
        })
        .collect::<Vec<(CmdFunc, u32)>>();
    (reg, program)
}

#[allow(unused_variables)]
pub fn run(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let (mut state, program) = parse(lines);
    while let Some(cmd) = program.get(state.inst_point as usize) {
        if cmd.0(cmd.1, &mut state) {
            state.inst_point += 1;
        }
    }
    state
        .result
        .into_iter()
        .enumerate()
        .fold("".to_string(), |res, (i, x)| {
            if i == 0 {
                return res + format!("{}", x).as_str();
            }
            res + format!(",{}", x).as_str()
        })
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), "4,6,3,5,6,3,5,2,1,0");
    }
}
