use num::abs;

pub struct Day1;

crate::impl_day!("1", true);

fn process_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l_left = vec![];
    let mut l_right = vec![];

    input
        .trim()
        .lines()
        .map(|l| l.split("   "))
        .for_each(|mut s| {
            l_left.push(s.next().unwrap().parse::<i32>().unwrap());
            l_right.push(s.next().unwrap().parse::<i32>().unwrap());
        });

    (l_left, l_right)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (mut left, mut right) = process_input(input);

    left.sort_unstable();
    right.sort_unstable();

    let res: i32 = left.iter().zip(&right).map(|(a, b)| abs(b - a)).sum();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (left, right) = process_input(input);

    let res: i32 = left
        .iter()
        .map(|&a| a * right.iter().filter(|&&b| b == a).count() as i32)
        .sum();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(11.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(31.to_string(), *solve_part2(INPUT).to_string());
    }
}
