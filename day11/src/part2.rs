use std::collections::HashMap;

const NR_OF_BLINKS: usize = 75;
type Cache = HashMap<(usize, usize), usize>;

#[allow(unused_variables)]
fn parse(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|x| x.parse::<usize>().expect("Value not a number"))
        .collect()
}

fn blink_stone(stone: usize, nr_of_blinks: usize, mut cache: Cache) -> (usize, Cache) {
    let mut sum = 0;
    if nr_of_blinks == 0 {
        return (1, cache);
    }
    if let Some(result) = cache.get(&(stone, nr_of_blinks)) {
        return (*result, cache);
    }
    match stone {
        0 => {
            let (result, new_cache) = blink_stone(1, nr_of_blinks - 1, cache);
            sum += result;
            cache = new_cache;
        }
        value if (value.ilog10() + 1) % 2 == 0 => {
            let split_digits = (value.ilog10() + 1) / 2;
            let upper = value / 10usize.pow(split_digits);
            let lower = value - upper * 10usize.pow(split_digits);
            let (result1, new_cache) = blink_stone(upper, nr_of_blinks - 1, cache);
            cache = new_cache;
            let (result2, new_cache) = blink_stone(lower, nr_of_blinks - 1, cache);
            sum += result1 + result2;
            cache = new_cache;
        }
        value => {
            let (result, new_cache) = blink_stone(value * 2024, nr_of_blinks - 1, cache);
            sum += result;
            cache = new_cache;
        }
    }
    cache.insert((stone, nr_of_blinks), sum);
    (sum, cache)
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let stones = parse(input.trim());
    let mut cache: Cache = HashMap::new();
    let mut sum = 0;
    for stone in stones {
        let (stone_len, new_cache) = blink_stone(stone, NR_OF_BLINKS, cache);
        cache = new_cache;
        sum += stone_len;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input2.txt").unwrap();
        assert_eq!(run(&example_content), 55312);
    }
}
