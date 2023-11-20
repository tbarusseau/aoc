use crate::solver::Solver;

pub struct Day9;

crate::impl_day!("9", true);

fn process_input(input: &str) -> String {
    input.trim_end().replace(char::is_whitespace, "")
}

fn parse_usize<'a, T: Into<&'a str>>(s: T) -> usize {
    s.into().parse().unwrap()
}

fn decompress(s: &str) -> usize {
    let mut out = 0;
    let mut current_slice = s;

    loop {
        match current_slice.find('(') {
            Some(pos) => {
                // Advance to next '('
                out += pos;
                current_slice = &current_slice[pos..];

                match current_slice.find(')') {
                    Some(pos) => {
                        // Extract the marker slice, i.e. the ({COUNT}x{REPETITIONS})
                        let marker = &current_slice[1..pos];

                        // Advance the current slice to after the marker
                        current_slice = &current_slice[pos + 1..];

                        // Extract count and repetitions
                        let x_pos = marker.find('x').expect("couldn't find a 'x'");
                        let count = parse_usize(&marker[0..x_pos]);
                        let repetitions = parse_usize(&marker[x_pos + 1..]);

                        // Recursively decompress the next `count` characters
                        let repetition_len = decompress(&current_slice[0..count]);

                        // This will account for {REPETITIONS} * {REPETITION_LEN} chars
                        out += repetitions * repetition_len;

                        current_slice = &current_slice[count..];
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                // We reached the end of the input
                out += current_slice.len();
                break;
            }
        }
    }

    out
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = decompress(&input);
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6.to_string(), *solve_part1("ADVENT").to_string());
        assert_eq!(7.to_string(), *solve_part1("A(1x5)BC").to_string());
        assert_eq!(9.to_string(), *solve_part1("(3x3)XYZ").to_string());
        assert_eq!(
            11.to_string(),
            *solve_part1("A(2x2)BCD(2x2)EFG").to_string()
        );
        assert_eq!(6.to_string(), *solve_part1("(6x1)(1x3)A").to_string());
        assert_eq!(18.to_string(), *solve_part1("X(8x2)(3x3)ABCY").to_string());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
