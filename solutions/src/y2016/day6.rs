use std::str::Lines;

use crate::{
    solver::Solver,
    utils::{
        char_utils::{index_from_char, index_to_char},
        iterator_index::GetIndexOfMax,
    },
};

pub struct Day6;

crate::impl_day!("6", true);

fn process_input(input: &str) -> Lines {
    input.trim().lines()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let len = input.lines().next().unwrap().len();
    let lines = process_input(input);

    let mut counter: Vec<Vec<usize>> = vec![vec![0; 26]; len];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            let char_index = index_from_char(c);
            counter[i][char_index as usize] += 1;
        }
    }

    let res = counter
        .iter()
        .map(|column| {
            let max_index = column
                .iter()
                .get_index_of_max()
                .expect("no max index in column");
            let most_frequent_char = index_to_char(max_index as u8, false);

            most_frequent_char
        })
        .collect::<String>();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let len = input.lines().next().unwrap().len();
    let lines = process_input(input);

    let mut counter: Vec<Vec<usize>> = vec![vec![0; 26]; len];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            let char_index = index_from_char(c);
            counter[i][char_index as usize] += 1;
        }
    }

    let res = counter
        .iter()
        .map(|column| {
            let min_index = column
                .iter()
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .map(|(a, b)| (b, a))
                .min()
                .expect("no min index in column");
            let least_frequent_char = index_to_char(min_index.1 as u8, false);

            least_frequent_char
        })
        .collect::<String>();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
"#;

    #[test]
    fn test_part1() {
        assert_eq!("easter".to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!("advent".to_string(), *solve_part2(INPUT).to_string());
    }
}
