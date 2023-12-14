use std::collections::HashMap;

use itertools::Itertools;

pub struct Day14;

crate::impl_day!("14", true);

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn rotate_left<T>(input: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Copy,
{
    let height = input.len();

    (0..height)
        .map(|y| {
            input
                .iter()
                .flat_map(|l| {
                    l.iter()
                        .rev()
                        .copied()
                        .enumerate()
                        .filter_map(|(i, c)| if i == y { Some(c) } else { None })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}

fn roll_stones_left(input: &mut [Vec<char>]) -> Vec<Vec<char>> {
    input
        .iter_mut()
        .map(|l| {
            l.split_mut(|v| *v == '#')
                .map(|split| {
                    split.sort_unstable();
                    split.reverse();
                    split.iter().collect::<String>()
                })
                .join("#")
                .chars()
                .collect::<Vec<char>>()
        })
        .collect_vec()
}

fn rotate_right(input: &[Vec<char>]) -> Vec<Vec<char>> {
    // Yes
    rotate_left(&rotate_left(&rotate_left(input)))
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let width = input.len();

    let mut rotated_left = rotate_left(&input);
    let rolled = roll_stones_left(&mut rotated_left);

    let res: usize = rolled
        .iter()
        .map(|l| {
            l.iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == 'O' { Some(width - i) } else { None })
                .sum::<usize>()
        })
        .sum();

    Box::new(res)
}

type Cache = HashMap<Vec<Vec<char>>, Vec<Vec<char>>>;

fn tilt(input: &[Vec<char>], h: Option<&mut Cache>) -> Vec<Vec<char>> {
    if let Some(ref h) = h {
        if let Some(v) = h.get(input) {
            return v.to_owned();
        }
    }

    let mut r = input.to_owned();

    r = rotate_left(&r);

    for _ in 0..4 {
        r = roll_stones_left(&mut r);
        r = rotate_right(&r);
    }

    r = rotate_right(&r);

    if let Some(h) = h {
        h.insert(input.to_owned(), r.clone());
    }

    r
}

fn find_cycle(current_value: &Vec<Vec<char>>, h: &Cache) -> Option<usize> {
    if h.contains_key(current_value) {
        let mut i = 0;
        let mut current = current_value;

        while let Some(next) = h.get(current) {
            i += 1;

            current = next;
            if next == current_value {
                return Some(i);
            }
        }
    }

    None
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    const RECURSION_AMOUNT: i32 = 1_000_000_000;

    let input = process_input(input);
    let width = input.len();
    let mut rotated_input = input;
    let mut h = HashMap::new();

    let mut i = 0;

    while i < RECURSION_AMOUNT {
        if let Some(cycle_length) = find_cycle(&rotated_input, &h) {
            // Found a cycle! Just compress what's left with a modulo of the cycle
            // and then finish the remaining iterations.
            let remaining = (RECURSION_AMOUNT - i) % cycle_length as i32;

            (0..remaining).for_each(|_| {
                rotated_input = tilt(&rotated_input, None);
            });

            break;
        }

        rotated_input = tilt(&rotated_input, Some(&mut h));
        i += 1;
    }

    let res: usize = rotate_left(&rotated_input)
        .iter()
        .map(|l| {
            l.iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == 'O' { Some(width - i) } else { None })
                .sum::<usize>()
        })
        .sum();

    Box::new(res)
}

#[allow(dead_code)]
fn print_arrays(input: &[Vec<char>]) {
    for l in input {
        println!("{}", l.iter().collect::<String>());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part1() {
        assert_eq!(136.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(64.to_string(), *solve_part2(INPUT).to_string());
    }

    #[test]
    fn utils_tests() {
        let v1 = vec![vec![1, 2], vec![3, 4]];
        let v2 = vec![vec![2, 4], vec![1, 3]];

        assert_eq!(rotate_left(&v1), v2);
    }

    #[test]
    fn test_tilts() {
        let one_cycle = process_input(
            r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        );

        let two_cycle = process_input(
            r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        );

        let three_cycle = process_input(
            r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        );

        let orig = process_input(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );

        let tilted_once = tilt(&orig, None);
        let tilted_twice = tilt(&tilted_once, None);
        let tilted_thrice = tilt(&tilted_twice, None);

        println!("orig:");
        print_arrays(&orig);
        println!();
        println!("tilted once:");
        print_arrays(&tilted_once);
        println!("should be:");
        print_arrays(&one_cycle);
        println!();
        println!("tilted twice:");
        print_arrays(&tilted_twice);
        println!("should be:");
        print_arrays(&two_cycle);
        println!();
        println!("tilted thrice:");
        print_arrays(&tilted_thrice);
        println!("should be:");
        print_arrays(&three_cycle);

        assert_eq!(one_cycle, tilted_once);
        assert_eq!(two_cycle, tilted_twice);
        assert_eq!(three_cycle, tilted_thrice);
    }
}
