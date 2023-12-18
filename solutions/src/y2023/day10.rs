use std::collections::HashMap;

use grid::Grid;
use itertools::Itertools;

use crate::utils::{direction::Direction, to_usize_tuple::to_usize_tuple};

pub struct Day10;

crate::impl_day!("10", true);

fn process_input(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();

    Grid::from_vec(
        input
            .trim_end()
            .lines()
            .flat_map(|s| s.chars().collect_vec())
            .collect_vec(),
        width,
    )
}

fn get_start_pos(input: &Grid<char>) -> (usize, usize) {
    for (y, row) in input.iter_rows().enumerate() {
        for (x, c) in row.enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }

    unreachable!()
}

const HAS_DOWN: &[char] = &['|', '7', 'F'];
const HAS_UP: &[char] = &['|', 'L', 'J'];
const HAS_RIGHT: &[char] = &['-', 'L', 'F'];
const HAS_LEFT: &[char] = &['-', 'J', '7'];

fn get_connected_pipes(input: &Grid<char>, pos: (usize, usize)) -> (Direction, Direction) {
    Direction::iterator()
        .filter_map(|&dir| {
            let dir_offset: (isize, isize) = dir.into();
            let neighbour_pos = (pos.0 as isize + dir_offset.0, pos.1 as isize + dir_offset.1);

            to_usize_tuple(neighbour_pos, input.cols(), input.rows())
                .and_then(|pos| input.get(pos.1, pos.0))
                .filter(|&c| match dir {
                    Direction::Up => HAS_DOWN.contains(c),
                    Direction::Right => HAS_LEFT.contains(c),
                    Direction::Left => HAS_RIGHT.contains(c),
                    Direction::Down => HAS_UP.contains(c),
                })
                .map(|_| dir)
        })
        .sorted()
        .collect_tuple()
        .expect("no two connected pipes")
}

fn connected_pipes_to_pipe(directions: (Direction, Direction)) -> char {
    match directions {
        (Direction::Up, Direction::Right) => 'L',
        (Direction::Right, Direction::Down) => 'F',
        (Direction::Down, Direction::Left) => '7',
        (Direction::Left, Direction::Up) => 'J',
        (Direction::Up, Direction::Down) => '|',
        (Direction::Right, Direction::Left) => '-',
        _ => unreachable!("directions: {:?}", directions),
    }
}

fn get_pipe_dir(pipe: char, dir: Direction) -> Direction {
    match dir {
        Direction::Up => match pipe {
            '|' => Direction::Up,
            '7' => Direction::Left,
            'F' => Direction::Right,
            _ => panic!(),
        },
        Direction::Right => match pipe {
            '-' => Direction::Right,
            '7' => Direction::Down,
            'J' => Direction::Up,
            _ => panic!(),
        },
        Direction::Left => match pipe {
            '-' => Direction::Left,
            'L' => Direction::Up,
            'F' => Direction::Down,
            _ => panic!(),
        },
        Direction::Down => match pipe {
            '|' => Direction::Down,
            'L' => Direction::Right,
            'J' => Direction::Left,
            _ => panic!(),
        },
    }
}

fn build_pipe_loop(input: &mut Grid<char>) -> HashMap<(usize, usize), char> {
    let mut h = HashMap::new();

    let start_pos = get_start_pos(input);

    let connected_pipes = get_connected_pipes(input, start_pos);
    let start_pipe = connected_pipes_to_pipe(connected_pipes);

    // Patch the start pipe
    input[(start_pos.1, start_pos.0)] = start_pipe;

    let mut pos = start_pos;
    let mut dir = connected_pipes.0;

    loop {
        let pipe = input[(pos.1, pos.0)];
        h.insert(pos, pipe);

        let dir_offset: (isize, isize) = dir.into();

        pos = (
            (pos.0 as isize + dir_offset.0) as usize,
            (pos.1 as isize + dir_offset.1) as usize,
        );
        let pipe = input[(pos.1, pos.0)];
        dir = get_pipe_dir(pipe, dir);

        if pos == start_pos {
            break;
        }
    }

    h
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let h = build_pipe_loop(&mut input);

    Box::new(h.len() / 2)
}

fn get_min_max<I: Iterator<Item = (usize, usize)>>(input: I) -> ((usize, usize), (usize, usize)) {
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for pos in input {
        min_x = min_x.min(pos.0);
        min_y = min_y.min(pos.1);
        max_x = max_x.max(pos.0);
        max_y = max_y.max(pos.1);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let h = build_pipe_loop(&mut input);
    let ((min_x, min_y), (max_x, max_y)) = get_min_max(h.keys().copied());
    let mut cells_count = 0;

    for y in min_y..=max_y {
        let mut is_in_loop = h.get(&(0, y)).is_some();
        for x in min_x..=max_x {
            let current = input[(y, x)];
            let current_is_on_loop = h.get(&(x, y)).is_some();

            if current == '.' {
                if is_in_loop {
                    cells_count += 1;
                }
            } else if current_is_on_loop {
                if is_in_loop {
                    match current {
                        '|' | '7' | 'J' => {
                            is_in_loop = !is_in_loop;
                        }
                        _ => {}
                    }
                } else {
                    match current {
                        '|' | 'F' | 'L' => {
                            is_in_loop = !is_in_loop;
                        }
                        _ => {}
                    }
                }
            }

            println!(
                "[{: >3?}] {current}: {is_in_loop} (count: {cells_count})",
                (x, y)
            );
        }
    }

    Box::new(cells_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    #[test]
    fn test_part1() {
        assert_eq!(8.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        const INPUT1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        const INPUT2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        const INPUT3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

        // INPUT2 solved
        // OF----7F7F7F7F-7OOOO
        // O|F--7||||||||FJOOOO
        // O||OFJ||||||||L7OOOO
        // FJL7L7LJLJ||LJIL-7OO
        // L--JOL7IIILJS7F-7L7O
        // OOOOF-JIIF7FJ|L7L7L7
        // OOOOL7IF7||L7|IL7L7|
        // OOOOO|FJLJ|FJ|F7|OLJ
        // OOOOFJL-7O||O||||OOO
        // OOOOL---JOLJOLJLJOOO

        // INPUT3 solved
        // FF7FSF7F7F7F7F7F---7
        // L|LJ||||||||||||F--J
        // FL-7LJLJ||||||LJL-77
        // F--JF--7||LJLJIF7FJ-
        // L---JF-JLJIIIIFJLJJ7
        // |F|F-JF---7IIIL7L|7|
        // |FFJF7L7F-JF7IIL---7
        // 7-L-JL7||F7|L7F-7F7|
        // L.L7LFJ|||||FJL7||LJ
        // L7JLJL-JLJLJL--JLJ.L

        // assert_eq!(4.to_string(), *solve_part2(INPUT1).to_string());
        assert_eq!(8.to_string(), *solve_part2(INPUT2).to_string());
        // assert_eq!(10.to_string(), *solve_part2(INPUT3).to_string());
    }
}
