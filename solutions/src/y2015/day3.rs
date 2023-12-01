use std::collections::HashMap;

pub struct Day3;

crate::impl_day!("3", true);

fn process_input(input: &str) -> &str {
    input
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut pos = (0, 0);
    let mut h: HashMap<(i32, i32), i32> = HashMap::new();

    h.insert((0, 0), 1);

    for c in input.chars() {
        match c {
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            _ => {}
        }

        *h.entry(pos).or_insert(0) += 1;
    }

    let res = h
        .iter()
        .fold(0, |acc, e| if *e.1 >= 1 { acc + 1 } else { acc });
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);
    let mut robot = false;
    let mut houses = HashMap::new();

    houses.insert((0, 0), 2);

    for c in input.chars() {
        let pos = if robot {
            &mut robot_pos
        } else {
            &mut santa_pos
        };

        match c {
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            _ => {}
        }

        *houses.entry(*pos).or_insert(0) += 1;
        robot = !robot;
    }

    let res = houses
        .iter()
        .fold(0, |acc, e| if *e.1 >= 1 { acc + 1 } else { acc });
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2.to_string(), *solve_part1(">").to_string());
        assert_eq!(4.to_string(), *solve_part1("^>v<").to_string());
        assert_eq!(2.to_string(), *solve_part1("^v^v^v^v^v").to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3.to_string(), *solve_part2("^v").to_string());
        assert_eq!(3.to_string(), *solve_part2("^>v<").to_string());
        assert_eq!(11.to_string(), *solve_part2("^v^v^v^v^v").to_string());
    }
}
