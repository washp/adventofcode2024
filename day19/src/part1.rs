#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Vec<String>, Vec<String>) {
    let mut iter = lines.iter();
    let towels: Vec<String> = iter
        .next()
        .expect("No towel row!")
        .split(",")
        .map(|x| x.trim().to_string())
        .collect();
    let patterns: Vec<String> = iter.skip(1).map(|x| x.trim().to_string()).collect();
    (towels, patterns)
}

fn match_pattern(pattern: String, towels: &[String]) -> bool {
    let mut matches = towels.to_owned();
    while let Some(tow_match) = matches.pop() {
        let mut partial_pattern = String::from("");
        for char in pattern.chars() {
            partial_pattern.push(char);
            if partial_pattern.len() != tow_match.len() {
                continue;
            }
            if partial_pattern == *tow_match {
                if partial_pattern.len() == pattern.len() {
                    return true;
                }
                matches.extend(
                    towels
                        .iter()
                        .map(|x| (tow_match.clone() + x))
                        .collect::<Vec<String>>(),
                );
                break;
            }
        }
    }
    false
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (towels, patterns) = parse(lines);
    let mut sum = 0;
    for pattern in patterns {
        if match_pattern(pattern, &towels) {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 6);
    }
}
