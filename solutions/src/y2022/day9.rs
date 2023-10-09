use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day9;

crate::impl_day!("9", true);

fn dir_to_vec(dir: &str) -> (isize, isize) {
    match dir {
        "R" => (1, 0),
        "U" => (0, 1),
        "L" => (-1, 0),
        "D" => (0, -1),
        _ => panic!(),
    }
}

fn should_move(target: (isize, isize), knot: (isize, isize)) -> bool {
    if target.0 == knot.0 {
        isize::abs(target.1 - knot.1) > 1
    } else if target.1 == knot.1 {
        isize::abs(target.0 - knot.0) > 1
    } else {
        let mdist = isize::abs(target.0 - knot.0) + isize::abs(target.1 - knot.1);

        mdist > 2
    }
}

fn move_towards_target(target: (isize, isize), knot: (isize, isize)) -> (isize, isize) {
    if target.0 == knot.0 {
        (0, if target.1 > knot.1 { 1 } else { -1 })
    } else if target.1 == knot.1 {
        (if target.0 > knot.0 { 1 } else { -1 }, 0)
    } else {
        if target.0 > knot.0 && target.1 > knot.1 {
            (1, 1)
        } else if target.0 > knot.0 && target.1 < knot.1 {
            (1, -1)
        } else if target.0 < knot.0 && target.1 > knot.1 {
            (-1, 1)
        } else if target.0 < knot.0 && target.1 < knot.1 {
            (-1, -1)
        } else {
            unreachable!()
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    let mut head_pos: (isize, isize) = (0, 0);
    let mut tail_pos: (isize, isize) = (0, 0);

    visited_pos.insert((0, 0));

    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap();
        let count = isize::from_str_radix(split.next().unwrap(), 10).unwrap();
        let dirvec = dir_to_vec(dir);

        for _ in 1..=count {
            head_pos.0 += dirvec.0;
            head_pos.1 += dirvec.1;

            if should_move(head_pos, tail_pos) {
                let move_tail = move_towards_target(head_pos, tail_pos);
                tail_pos.0 += move_tail.0;
                tail_pos.1 += move_tail.1;

                visited_pos.insert(tail_pos);
            }
        }
    }

    Box::new(visited_pos.len())
}

#[allow(unused)]
fn display(set: &HashSet<(isize, isize)>) {
    for y in -20..20 {
        for x in -20..20 {
            if x == 0 && y == 0 {
                print!("s ");
                continue;
            }

            print!("{} ", if set.contains(&(x, -y)) { "#" } else { "." });
        }

        println!();
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); 10];

    visited_pos.insert((0, 0));

    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap();
        let count = isize::from_str_radix(split.next().unwrap(), 10).unwrap();
        let dirvec = dir_to_vec(dir);

        for _ in 0..count {
            let head = knots.get_mut(0).unwrap();
            head.0 += dirvec.0;
            head.1 += dirvec.1;

            windows_mut_each(&mut knots, 2, |pair| {
                let target = pair[0];
                let mut knot = pair[1];

                if should_move(target, knot) {
                    let move_amount = move_towards_target(target, knot);
                    knot.0 += move_amount.0;
                    knot.1 += move_amount.1;
                }
            });

            if !visited_pos.contains(&knots[9]) {
                println!("Inserting pos {:?}", knots[9]);
                visited_pos.insert(knots[9]);
            }
        }
    }

    Box::new(visited_pos.len())
}

fn windows_mut_each<T>(v: &mut [T], n: usize, mut f: impl FnMut(&mut [T])) {
    let mut start = 0;
    let mut end = n;
    while end <= v.len() {
        f(&mut v[start..end]);
        start += 1;
        end += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn test_part1() {
        assert_eq!(13.to_string(), *solve_part1(INPUT).to_string());
    }

    const INPUT2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn test_part2() {
        assert_eq!(36.to_string(), *solve_part2(INPUT2).to_string());
    }
}
