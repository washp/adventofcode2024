#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Segment {
    length: usize,
    id: Option<usize>,
}

#[allow(unused_variables)]
fn parse(line: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut iter = line.trim().chars().enumerate();
    while let Some(segment) = iter.next() {
        segments.push(Segment {
            id: Some(segment.0 / 2),
            length: segment
                .1
                .to_digit(10)
                .expect("No number in segment, failed to parse") as usize,
        });
        if let Some(segment) = iter.next() {
            segments.push(Segment {
                id: None,
                length: segment
                    .1
                    .to_digit(10)
                    .expect("No number in segment, failed to parse")
                    as usize,
            })
        };
    }
    segments
}

#[allow(clippy::mut_range_bound)]
fn calculate_checksum(segments: Vec<Segment>) -> usize {
    let mut last_i = 0;
    let mut sum = 0;
    for segment in segments {
        if let Some(id) = segment.id {
            for a in last_i..(segment.length + last_i) {
                sum += a * id;
                last_i = a + 1;
            }
        }
    }
    sum
}

fn defrag(mut segments: Vec<Segment>) -> Vec<Segment> {
    for i in 0..segments.len() {
        let mut last = segments.pop().expect("No last item?");
        if last.id.is_none() {
            last = segments.pop().expect("No last item?");
        }
        if let Some(segment) = segments.get_mut(i) {
            if segment.id.is_none() {
                match segment.length {
                    value if value > last.length => {
                        segment.length -= last.length;
                        segments.insert(i, last);
                    }
                    value if value == last.length => {
                        segments.remove(i);
                        segments.insert(i, last);
                    }
                    _ => {
                        last.length -= segment.length;
                        segment.id = last.id;
                        segments.push(last);
                    }
                }
            } else {
                segments.push(last);
            }
        } else {
            segments.push(last);
        }
    }
    segments
}

#[allow(unused_variables)]
pub fn run(input: &str) -> usize {
    let line = input;
    let mut segments = parse(line);
    segments = defrag(segments);
    calculate_checksum(segments)
}

#[cfg(test)]
mod tests {

    use super::run;
    use std::fs;

    #[test]
    fn test_part1_1() {
        let example_content = "12345";
        assert_eq!(run(&example_content), 60);
    }
    #[test]
    fn test_part1_2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 1928);
    }
}
