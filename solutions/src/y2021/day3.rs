use std::{cmp::Ordering, collections::BTreeMap};

pub struct Day3;

crate::impl_day!("3", true);

fn get_gamma(input: &str) -> i32 {
    let mut counts = BTreeMap::new();

    for entry in input.lines() {
        for (pos, c) in entry.chars().enumerate() {
            *counts.entry(pos).or_insert(0) += if c == '0' { -1 } else { 1 };
        }
    }

    let mut gamma = String::new();

    for &v in counts.values() {
        match v.cmp(&0) {
            Ordering::Less => gamma.push('0'),
            Ordering::Equal => panic!("Equal count"),
            Ordering::Greater => gamma.push('1'),
        }
    }

    i32::from_str_radix(&gamma, 2).unwrap()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let gamma = get_gamma(input);
    let mask = 2_i32.pow(format!("{:b}", gamma).len() as u32) - 1;

    let epsilon = gamma ^ mask;

    Box::new(gamma * epsilon)
}

fn partition(values: &[i32], bit_index: usize) -> (Vec<i32>, Vec<i32>) {
    values.iter().partition(|&v| (v & 1 << bit_index) > 0)
}

fn criteria<F>(values: &[i32], bit_len: usize, cmp: F) -> i32
where
    F: Fn(usize, usize) -> bool,
{
    let mut values = values.to_vec();
    for bit in (0..bit_len).rev() {
        let (ones, zeros) = partition(&values, bit);
        if cmp(ones.len(), zeros.len()) {
            values = ones;
        } else {
            values = zeros;
        }
        if let [value] = *values {
            return value;
        }
    }
    unreachable!()
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let bit_len = input.lines().next().unwrap().len();
    let input: Vec<i32> = input
        .lines()
        .flat_map(|l| i32::from_str_radix(l, 2))
        .collect();

    let o2 = criteria(&input, bit_len, |ones, zeroes| ones >= zeroes);
    let co2 = criteria(&input, bit_len, |ones, zeroes| ones < zeroes);

    Box::new(o2 * co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        assert_eq!(198.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(230.to_string(), *solve_part2(INPUT).to_string());

        let s = "111111
111110
111100
111000
110000
100000
011110
001111
        ";
        assert_eq!(945.to_string(), *solve_part2(s).to_string());

        let s = "1111
1110
1101
0110
1001
0010
0001";
        assert_eq!(90.to_string(), *solve_part2(s).to_string());
    }
}
