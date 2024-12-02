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

fn is_row_safe(row: &[usize], allow_recursion: bool) -> bool {
    let mut row_iter = row.iter();
    let mut is_dir_up: Option<bool> = None;
    let mut last_val = row_iter.next().expect("No first number on row");
    let mut result = true;
    for value in row_iter {
        if is_dir_up.is_none() {
            is_dir_up = Some(value > last_val)
        }
        result = match value {
            _ if value == last_val => false,
            _ if is_dir_up == Some(true) && value < last_val => false,
            _ if is_dir_up == Some(false) && value > last_val => false,
            _ if (*value as isize - *last_val as isize).abs() > 3 => false,
            &_ => true,
        };
        if !result {
            if allow_recursion {
                for i in 0..row.len() {
                    let mut new_row = row.to_vec();
                    new_row.remove(i);
                    result = is_row_safe(&new_row, false);
                    if result {
                        break;
                    }
                }
            }
            break;
        }
        last_val = value;
    }
    result
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let rows = parse(lines);
    let mut safe_rows = 0;
    for row in rows {
        if is_row_safe(&row, true) {
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
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 4);
    }
}
