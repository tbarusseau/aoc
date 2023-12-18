use itertools::Itertools;

use crate::utils::direction::Direction;

pub struct Day18;

crate::impl_day!("18", true);

struct Instruction(Direction, usize, String);

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .trim_end()
        .lines()
        .filter_map(|l| {
            if let Ok(r) = sscanf::sscanf!(l, "{char} {usize} (#{String})") {
                Some(Instruction(Direction::from_char(r.0), r.1, r.2))
            } else {
                None
            }
        })
        .collect_vec()
}

fn solve(instructions: &[Instruction]) -> usize {
    // Use Gauss's area formula to compute total area based
    // on polygon coordinates + perimeter length

    let mut pos: (isize, isize) = (0, 0);
    let mut corners = vec![];
    let mut trench_length = 0;
    corners.push(pos);

    for inst in instructions {
        let Instruction(dir, length, ..) = *inst;
        let dir_offset: (isize, isize) = dir.into();
        trench_length += length;

        pos = (
            pos.0 + dir_offset.0 * length as isize,
            pos.1 + dir_offset.1 * length as isize,
        );
        corners.push(pos);
    }

    corners
        .iter()
        .tuple_windows()
        .map(|((x0, y0), (x1, y1))| x0 * y1 - x1 * y0)
        .sum::<isize>()
        .unsigned_abs()
        / 2
        + trench_length / 2
        + 1
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);

    Box::new(solve(&instructions))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input)
        .iter()
        .map(|Instruction(_, _, s)| {
            let length = usize::from_str_radix(&s[0..5], 16).expect("invalid length");
            let dir = match (s[5..6]).parse::<usize>().expect("invalid dir") {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                v => panic!("invalid dir encoding: {}", v),
            };

            Instruction(dir, length, String::new())
        })
        .collect_vec();

    Box::new(solve(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part1() {
        assert_eq!(62.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            952_408_144_115_usize.to_string(),
            *solve_part2(INPUT).to_string()
        );
    }
}
