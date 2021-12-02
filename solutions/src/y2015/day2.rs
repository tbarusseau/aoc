use crate::solver::Solver;

pub struct Day2;

crate::impl_day!("2", true);

fn process_input(input: &str) -> Vec<[i32; 3]> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split('x');

            let l: i32 = split.next().unwrap().parse().unwrap();
            let w: i32 = split.next().unwrap().parse().unwrap();
            let h: i32 = split.next().unwrap().parse().unwrap();

            [l, w, h]
        })
        .collect()
}

fn get_required_surface(l: i32, w: i32, h: i32) -> i32 {
    2 * l * w + 2 * w * h + 2 * h * l + (l * w).min((w * h).min(h * l))
}

fn get_required_ribbon_length(dimensions: [i32; 3]) -> i32 {
    let mut dim = dimensions;
    dim.sort_unstable();

    let [l, w, h] = dim;

    l + l + w + w + l * w * h
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .iter()
        .fold(0, |acc, &[l, w, h]| acc + get_required_surface(l, w, h));
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().fold(0, |acc, &dimensions| {
        acc + get_required_ribbon_length(dimensions)
    });
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(58.to_string(), *solve_part1("2x3x4").to_string());
        assert_eq!(43.to_string(), *solve_part1("1x1x10").to_string());
        assert_eq!(
            (58 + 43).to_string(),
            *solve_part1("2x3x4\n1x1x10").to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(34.to_string(), *solve_part2("2x3x4").to_string());
        assert_eq!(14.to_string(), *solve_part2("1x1x10").to_string());
        assert_eq!(
            (34 + 14).to_string(),
            *solve_part2("2x3x4\n1x1x10").to_string()
        );
    }
}
