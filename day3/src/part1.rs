#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> Vec<(usize, usize)> {
    let mut multiplications: Vec<(usize, usize)> = vec![];
    for line in lines {
        let mut iter = line.split("mul(").peekable();
        if iter.peek() != Some(&"mul(") {
            iter.next();
        }
        let potential_mul: Vec<(usize, usize)> = iter
            .map(|x| {
                x.split(")")
                    .next()
                    .expect("No first value")
                    .split(",")
                    .collect()
            })
            .filter(|x: &Vec<&str>| x.len() == 2)
            .map(|x| {
                let mut iter = x.iter();
                (
                    iter.next().unwrap_or(&"0").parse::<usize>().unwrap_or(0),
                    iter.next().unwrap_or(&"0").parse::<usize>().unwrap_or(0),
                )
            })
            .collect();
        multiplications.extend(potential_mul)
    }
    multiplications
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let mul = parse(lines);
    let mut sum = 0;
    for pair in mul {
        sum += pair.0 * pair.1;
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
        assert_eq!(run(&example_content), 161);
    }
}
