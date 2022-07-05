use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day13;

crate::impl_day!("13", true);

struct Instructions(Vec<(isize, bool)>);

fn process_input(input: &str) -> (HashSet<(isize, isize)>, Instructions) {
    let mut set = HashSet::new();
    let mut lines = input.trim().lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut s = line.split(',');
        let x = s.next().unwrap().parse::<isize>().unwrap();
        let y = s.next().unwrap().parse::<isize>().unwrap();

        set.insert((x, y));
    }

    let mut instructions = Vec::new();

    for line in lines {
        let index: isize = (line.chars().collect::<Vec<_>>()[13..])
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        let horizontal = line.starts_with("fold along x=");

        instructions.push((index, horizontal));
    }

    (set, Instructions(instructions))
}

fn fold(set: HashSet<(isize, isize)>, index: isize, horizontal: bool) -> HashSet<(isize, isize)> {
    set.iter()
        .map(|&(x, y)| {
            if horizontal {
                if x > index {
                    (2 * index - x, y)
                } else {
                    (x, y)
                }
            } else {
                if y > index {
                    (x, 2 * index - y)
                } else {
                    (x, y)
                }
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (set, instructions) = process_input(input);
    let (index, horizontal) = instructions.0[0];

    let set = fold(set, index, horizontal);

    let res = set.iter().count();
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (mut set, instructions) = process_input(input);
    let mut ins = instructions.0.iter();

    while let Some(ins) = ins.next() {
        let &(index, horizontal) = ins;

        set = fold(set, index, horizontal);
    }

    let mut max_x = 0;
    let mut max_y = 0;

    for &(x, y) in set.iter() {
        if x > max_x {
            max_x = x + 1;
        }

        if y > max_y {
            max_y = y + 1;
        }
    }

    let mut v = vec![format!("{}", " ".repeat(max_x as usize)); max_y as usize];

    set.iter().for_each(|&(x, y)| {
        v[y as usize].replace_range((x as usize)..(x as usize + 1), "X");
    });

    Box::new(format!("\n{}", v.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn test_part1() {
        assert_eq!(17.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
