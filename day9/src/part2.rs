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
        for a in last_i..(segment.length + last_i) {
            if let Some(id) = segment.id {
                sum += a * id;
            }
            last_i = a + 1;
        }
    }
    sum
}

fn defrag(mut segments: Vec<Segment>) -> Vec<Segment> {
    for i_seg in (0..segments.len()).rev() {
        let segment = segments.remove(i_seg);
        let mut found_void = false;
        if segment.id.is_none() {
            segments.insert(i_seg, segment);
            continue;
        }
        for i_void in 0..i_seg {
            let void = segments
                .get_mut(i_void)
                .expect("No value at index, weird...");
            if void.id.is_some() {
                continue;
            }
            if void.length >= segment.length {
                void.length -= segment.length;
                segments.insert(i_void, segment);
                segments.insert(
                    i_seg,
                    Segment {
                        id: None,
                        length: segment.length,
                    },
                );
                found_void = true;
                break;
            }
        }
        if !found_void {
            segments.insert(i_seg, segment);
        }
    }
    segments.iter().filter(|x| x.length > 0).cloned().collect()
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
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 2858);
    }
}
