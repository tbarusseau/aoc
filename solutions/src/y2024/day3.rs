use regex::Regex;

pub struct Day3;

crate::impl_day!("3", true);

fn process_input(input: &str) -> &str {
    input.trim()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let res: i32 = re
        .captures_iter(input)
        .map(|cap| {
            let first = &cap["first"];
            let second = &cap["second"];

            first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
        })
        .sum();

    Box::new(res)
}

fn build_dos_donts_index(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut input_ref = input;

    let mut v = vec![];
    let mut w = vec![];
    let mut offset = 0;

    // Find dos
    v.push(0);
    while let Some(found) = input_ref.find("do()") {
        v.push(found + offset);
        offset += found + 1;
        input_ref = &input_ref[found + 1..];
    }

    input_ref = input;
    offset = 0;

    // Find donts
    while let Some(found) = input_ref.find("don't()") {
        w.push(found + offset);
        offset += found + 1;
        input_ref = &input_ref[found + 1..];
    }

    (v, w)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let (v, w) = build_dos_donts_index(input);

    let res: i32 = re
        .captures_iter(input)
        .map(|cap| {
            let index_in_string = cap.get(1).unwrap().start() - 4;

            let index_of_last_do = *v
                .iter()
                .filter(|&n| *n < index_in_string)
                .last()
                .unwrap_or(&0);
            let index_of_last_dont = *w
                .iter()
                .filter(|&n| *n < index_in_string)
                .last()
                .unwrap_or(&0);

            if index_of_last_dont > index_of_last_do {
                return 0;
            }

            let first = &cap["first"];
            let second = &cap["second"];

            first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
        })
        .sum();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str =
            r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        const INPUT: &str =
            r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48.to_string(), *solve_part2(INPUT).to_string());
    }
}
