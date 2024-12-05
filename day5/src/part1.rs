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

fn check_rules(rules: &Vec<(usize, usize)>, values: &Vec<Vec<usize>>) -> usize {
    let mut correct_sum = 0;
    'outer: for row in values {
        for rule in rules {
            if let Some(i0) = row.iter().position(|&x| x == rule.0) {
                if let Some(i1) = row.iter().position(|&x| x == rule.1) {
                    if i0 > i1 {
                        continue 'outer;
                    }
                }
            }
        }
        correct_sum += row[row.len() / 2];
    }
    correct_sum
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (rules, values) = parse(lines);
    check_rules(&rules, &values)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 143);
    }
}
