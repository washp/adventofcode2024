use std::collections::HashMap;

type Towels = Vec<String>;

#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Towels, Vec<String>) {
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

fn match_pattern(pattern: String, towels: &Towels, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(value) = cache.get(&pattern) {
        return *value;
    }
    if pattern.is_empty() {
        return 1;
    }

    let mut matches = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            matches += match_pattern(pattern[towel.len()..].to_string(), towels, cache);
        }
    }
    cache.insert(pattern, matches);
    matches
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (towels, patterns) = parse(lines);
    let mut sum = 0;
    for (i, pattern) in patterns.into_iter().enumerate() {
        let mut cache: HashMap<String, usize> = HashMap::new();
        sum += match_pattern(pattern, &towels, &mut cache);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 16);
    }
}
