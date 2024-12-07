#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<(usize, Vec<usize>)> {
    let mut result = Vec::new();
    for line in lines {
        let mut iter = line.split(":");
        let row_result = iter
            .next()
            .expect("no first value on row")
            .parse::<usize>()
            .expect("result could not be parsed to number!");
        let values = iter
            .next()
            .expect("no list of values on row")
            .split_whitespace()
            .map(|x| {
                x.parse::<usize>()
                    .expect("failed to convert list item to usize!")
            })
            .collect();
        result.push((row_result, values));
    }
    result
}

fn find_solvable_lines(lines: Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;
    for line in lines {
        let mut val_iter = line.1.iter();
        let mut result_list = vec![*val_iter.next().expect("No first value!")];
        for val in val_iter {
            result_list = result_list.iter().fold(Vec::new(), |mut list, x| {
                list.push(*x + val);
                list.push(*x * val);
                list.push((x * 10_usize.pow(val.checked_ilog10().unwrap_or(0) + 1)) + val);
                list
            })
        }
        if result_list.iter().any(|x| *x == line.0) {
            result += line.0;
        }
    }
    result
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    find_solvable_lines(parse(lines))
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 11387);
    }
}
