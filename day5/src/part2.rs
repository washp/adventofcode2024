use itertools::Itertools;

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut line_iter = lines.iter();
    let rules: Vec<(usize, usize)> = line_iter
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            x.split("|")
                .map(|x| x.parse::<usize>().expect("rules value is not a number!"))
                .collect_tuple()
                .expect("No 2 values on rules line, weird...")
        })
        .collect();
    let values: Vec<Vec<usize>> = line_iter
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<usize>().expect("rules value is not a number!"))
                .collect()
        })
        .collect();
    (rules, values)
}

fn fix_row(mut row: Vec<usize>, rules: &Vec<(usize, usize)>) -> usize {
    let mut correct = false;
    'outer: while !correct {
        for rule in rules {
            if let Some(low_i) = row.iter().position(|&x| x == rule.0) {
                if let Some(high_i) = row.iter().position(|&x| x == rule.1) {
                    if low_i > high_i {
                        let low_val = row.remove(low_i);
                        row.insert(high_i, low_val);
                        continue 'outer;
                    }
                }
            }
        }
        correct = true;
    }
    row[row.len() / 2]
}

fn check_rules(rules: &Vec<(usize, usize)>, mut values: Vec<Vec<usize>>) -> usize {
    let mut correct_sum = 0;
    'outer: for row in values.iter_mut() {
        for rule in rules {
            if let Some(low_i) = row.iter().position(|&x| x == rule.0) {
                if let Some(high_i) = row.iter().position(|&x| x == rule.1) {
                    if low_i > high_i {
                        correct_sum += fix_row(row.clone(), rules);
                        continue 'outer;
                    }
                }
            }
        }
    }
    correct_sum
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (rules, values) = parse(lines);
    check_rules(&rules, values)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 123);
    }
}
