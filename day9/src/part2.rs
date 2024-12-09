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
    for segment_to_move in segments.clone().into_iter().rev() {
        if segment_to_move.id.is_none() {
            continue;
        }
        for a in 0..segments.len() {
            let index = segments
                .clone()
                .iter()
                .position(|x| x.id == segment_to_move.id)
                .expect("Item not here?");
            if index <= a {
                continue;
            }
            if let Some(segment) = segments.get_mut(a) {
                if segment.id.is_none() {
                    if segment.length == segment_to_move.length {
                        segments.remove(index);
                        segments.insert(
                            index,
                            Segment {
                                id: None,
                                length: segment_to_move.length,
                            },
                        );
                        segments.remove(a);
                        segments.insert(a, segment_to_move);
                    } else if segment.length > segment_to_move.length {
                        segment.length -= segment_to_move.length;
                        segments.remove(index);
                        segments.insert(
                            index,
                            Segment {
                                id: None,
                                length: segment_to_move.length,
                            },
                        );
                        segments.insert(a, segment_to_move);
                    }
                }
            }
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
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 2858);
    }
}
