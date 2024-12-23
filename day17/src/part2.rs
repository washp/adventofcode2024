#[derive(Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    inst_point: u64,
    result: Vec<u64>,
}
type CmdFunc = fn(u64, &mut State) -> bool;

fn get_combo(op: u64, state: &State) -> u64 {
    match op {
        1..=3 => op,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => 0,
    }
}

fn adv(operand: u64, state: &mut State) -> bool {
    state.a /= 2u64.pow(get_combo(operand, state) as u32);
    true
}
fn bxl(operand: u64, state: &mut State) -> bool {
    state.b ^= operand;
    true
}
fn bst(operand: u64, state: &mut State) -> bool {
    state.b = get_combo(operand, state) % 8;
    true
}
fn jnz(operand: u64, state: &mut State) -> bool {
    if state.a != 0 {
        state.inst_point = operand / 2;
        return false;
    }
    true
}
fn bxc(_: u64, state: &mut State) -> bool {
    state.b ^= state.c;
    true
}
fn out(operand: u64, state: &mut State) -> bool {
    //println!("A: {:o}", state.a);
    state.result.push(get_combo(operand, state) % 8);
    true
}
fn bdv(operand: u64, state: &mut State) -> bool {
    state.b = state.a / 2u64.pow((get_combo(operand, state)) as u32);
    true
}
fn cdv(operand: u64, state: &mut State) -> bool {
    state.c = state.a / 2u64.pow((get_combo(operand, state)) as u32);
    true
}

fn noop(_: u64, _: &mut State) -> bool {
    panic!("Unknown command!");
}

fn get_number(line: &str) -> u64 {
    line.split(":")
        .nth(1)
        .expect("No part after : on line")
        .trim()
        .parse::<u64>()
        .expect("Second part of line is not a number")
}

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (State, Vec<(CmdFunc, u64)>, Vec<u64>) {
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
    let expected: Vec<u64> = iter
        .nth(1)
        .expect("No program row")
        .trim()
        .split(":")
        .nth(1)
        .expect("No second part of program row")
        .split(",")
        .map(|x| x.trim().parse::<u64>().expect("Command not a number"))
        .collect();
    let program = expected
        .iter()
        .step_by(2)
        .zip(expected.iter().skip(1).step_by(2))
        .map(|x| match x.0 {
            0 => (adv as CmdFunc, *x.1),
            1 => (bxl as CmdFunc, *x.1),
            2 => (bst as CmdFunc, *x.1),
            3 => (jnz as CmdFunc, *x.1),
            4 => (bxc as CmdFunc, *x.1),
            5 => (out as CmdFunc, *x.1),
            6 => (bdv as CmdFunc, *x.1),
            7 => (cdv as CmdFunc, *x.1),
            _ => (noop as CmdFunc, 0),
        })
        .collect::<Vec<(CmdFunc, u64)>>();
    (reg, program, expected)
}

#[allow(unused_variables)]
pub fn run(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let (org_state, program, expected) = parse(lines);
    let mut rev_exp: Vec<u64> = Vec::new();
    let mut new_res_list: Vec<u64> = vec![0];
    for exp in expected.iter().rev() {
        rev_exp.insert(0, *exp);
        let res_list = new_res_list;
        new_res_list = Vec::new();
        for res in res_list {
            for i in 0..8 {
                let mut state = org_state.clone();
                let a_start = res + i;

                state.a = a_start;
                while let Some(cmd) = program.get(state.inst_point as usize) {
                    if cmd.0(cmd.1, &mut state) {
                        state.inst_point += 1;
                    }
                }
                if rev_exp == state.result {
                    new_res_list.push(a_start << 3);
                }
            }
        }
    }
    new_res_list.iter().min().expect("No result values") >> 3
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 117440);
    }
}
