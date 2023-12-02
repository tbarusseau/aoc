pub struct Day1;

crate::impl_day!("1", true);

fn process_input(input: &str) -> Vec<i32> {
    input.lines().flat_map(str::parse).collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut increased = 0;
    for slice in input.windows(2) {
        if let &[a, b] = slice {
            if b > a {
                increased += 1;
            }
        }
    }

    Box::new(increased)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut increased = 0;
    let mut previous = i32::MAX;

    for slice in input.windows(3) {
        if let &[a, b, c] = slice {
            let current = a + b + c;
            if current > previous {
                increased += 1;
            }
            previous = current;
        }
    }

    Box::new(increased)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        assert_eq!(7.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(5.to_string(), *solve_part2(INPUT).to_string());
    }
}
