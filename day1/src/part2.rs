#[allow(unused_variables)]
fn parse(lines: Vec<&str>) -> (Vec<usize>, Vec<usize>) {
    let mut first_col = vec![];
    let mut second_col = vec![];
    for line in lines {
        let mut iter = line.split(" ");
        first_col.push(
            iter.next()
                .expect("No first value in input row")
                .parse::<usize>()
                .expect("First value was not a number"),
        );
        second_col.push(
            iter.find(|x| !x.is_empty())
                .expect("No second value in input row")
                .parse::<usize>()
                .expect("Second value was not a number"),
        );
    }
    first_col.sort();
    second_col.sort();
    (first_col, second_col)
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (col1, col2) = parse(lines);
    let mut sum = 0;
    for val in col1 {
        sum += val * col2.iter().filter(|x| &&val == x).count();
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
        assert_eq!(run(&example_content), 31);
    }
}
