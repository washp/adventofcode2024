#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().expect("Not a number"))
                .collect()
        })
        .collect()
}

fn is_row_safe(row: &[usize]) -> bool {
    let mut row_iter = row.iter();
    let mut is_dir_up: Option<bool> = None;
    let mut last_val = row_iter.next().expect("No first number on row");
    for value in row_iter {
        if is_dir_up.is_none() {
            is_dir_up = Some(value > last_val)
        }
        match value {
            _ if value == last_val => return false,
            _ if is_dir_up == Some(true) && value < last_val => return false,
            _ if is_dir_up == Some(false) && value > last_val => return false,
            _ if (*value as isize - *last_val as isize).abs() > 3 => return false,
            &_ => (),
        }
        last_val = value;
    }
    true
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let rows = parse(lines);
    let mut safe_rows = 0;
    for row in rows {
        if is_row_safe(&row) {
            safe_rows += 1;
        }
    }
    safe_rows
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 2);
    }
}
