#[allow(unused_variables)]
fn parse(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|x| x.parse::<usize>().expect("Value not a number"))
        .collect()
}

fn blink_stone(stone: usize, nr_of_blinks: usize) -> usize {
    let mut stones = vec![stone];
    for _ in 0..nr_of_blinks {
        let mut extra_stones = 0;
        for i in 0..stones.len() {
            let stone = stones.remove(i + extra_stones);
            match stone {
                0 => {
                    stones.insert(i, 1);
                }
                value if (value.ilog10() + 1) % 2 == 0 => {
                    let split_digits = (value.ilog10() + 1) / 2;
                    let upper = value / 10usize.pow(split_digits);
                    let lower = value - upper * 10usize.pow(split_digits);
                    stones.insert(i, upper);
                    stones.insert(i + 1, lower);
                    extra_stones += 1;
                }
                value => stones.insert(i, value * 2024),
            }
        }
    }
    stones.len()
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let stones = parse(input.trim());
    stones.iter().map(|x| blink_stone(*x, 25)).sum()
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 55312);
    }
}
