use std::collections::HashSet;

use itertools::Itertools;
use tuple::T2;

use crate::utils::direction::Direction;

pub struct Day16;

crate::impl_day!("16", true);

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

type RaysStorage = Vec<(T2<isize, isize>, Direction)>;

fn try_get_next_pos(
    pos: T2<isize, isize>,
    dir: Direction,
    input: &[Vec<char>],
) -> Option<T2<isize, isize>> {
    let dir_offset = T2::from(dir);

    let new_pos = pos + dir_offset;

    let height = input.len();
    let width = input[0].len();

    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= width as isize || new_pos.1 >= height as isize
    {
        None
    } else {
        Some(new_pos)
    }
}

fn push_next(
    pos: T2<isize, isize>,
    dir: Direction,
    cache: &mut HashSet<(T2<isize, isize>, Direction)>,
    v: &mut RaysStorage,
    input: &[Vec<char>],
) {
    if let Some(new_pos) = try_get_next_pos(pos, dir, input) {
        let next_move = &(new_pos, dir);

        if cache.get(next_move).is_none() {
            v.push(*next_move);
            cache.insert(*next_move);
        }
    }
}

fn push_new_positions(
    input: &[Vec<char>],
    current: &(T2<isize, isize>, Direction),
    v: &mut RaysStorage,
    h: &mut HashSet<T2<isize, isize>>,
    cache: &mut HashSet<(T2<isize, isize>, Direction)>,
) {
    let (pos, dir) = *current;

    h.insert(pos);
    let c = input[pos.1 as usize][pos.0 as usize];

    match c {
        '.' => {
            push_next(pos, dir, cache, v, input);
        }
        '/' => {
            let new_dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Left,
            };

            push_next(pos, new_dir, cache, v, input);
        }
        '\\' => {
            let new_dir = match dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
            };

            push_next(pos, new_dir, cache, v, input);
        }
        '|' => match dir {
            Direction::Up | Direction::Down => {
                push_next(pos, dir, cache, v, input);
            }
            Direction::Right | Direction::Left => {
                push_next(pos, Direction::Up, cache, v, input);
                push_next(pos, Direction::Down, cache, v, input);
            }
        },
        '-' => match dir {
            Direction::Right | Direction::Left => {
                push_next(pos, dir, cache, v, input);
            }
            Direction::Up | Direction::Down => {
                push_next(pos, Direction::Right, cache, v, input);
                push_next(pos, Direction::Left, cache, v, input);
            }
        },
        _ => unreachable!(),
    }
}

fn solve_p1(input: &[Vec<char>], starting_conditions: (T2<isize, isize>, Direction)) -> usize {
    let mut h: HashSet<T2<isize, isize>> = HashSet::new();
    let mut v: RaysStorage = vec![starting_conditions];
    let mut cache: HashSet<(T2<isize, isize>, Direction)> = HashSet::new();

    loop {
        if v.is_empty() {
            break;
        }

        let mut new_v: RaysStorage = vec![];

        for light_ray in &v {
            push_new_positions(input, light_ray, &mut new_v, &mut h, &mut cache);
        }

        v = new_v;
    }

    h.len()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve_p1(
        &process_input(input),
        ((0, 0).into(), Direction::Right),
    ))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let height = input.len();
    let width = input[0].len();

    let mut max = usize::MIN;

    for y in 0..height {
        for x in 0..width {
            let start_position: T2<isize, isize> = (x as isize, y as isize).into();

            if x == 0 {
                max = max.max(solve_p1(&input, (start_position, Direction::Right)));
            }

            if x == width - 1 {
                max = max.max(solve_p1(&input, (start_position, Direction::Left)));
            }

            if y == 0 {
                max = max.max(solve_p1(&input, (start_position, Direction::Down)));
            }

            if y == height - 1 {
                max = max.max(solve_p1(&input, (start_position, Direction::Up)));
            }
        }
    }

    Box::new(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_part1() {
        assert_eq!(46.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(51.to_string(), *solve_part2(INPUT).to_string());
    }
}
