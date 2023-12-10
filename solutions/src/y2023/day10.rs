use itertools::Itertools;

use crate::utils::direction::Direction;

pub struct Day10;

crate::impl_day!("10", true);

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim_end()
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec()
}

fn get_start_pos(input: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }

    unreachable!()
}

fn get_valid_directions(input: &[Vec<char>], pos: (usize, usize)) -> Vec<Direction> {
    let height = input.len();
    let width = input[0].len();

    let mut valid_directions = vec![];

    for dir in Direction::iterator() {
        let ipos = (pos.0 as isize, pos.1 as isize);
        let dir_ipos: (isize, isize) = (*dir).into();

        let neighbour_pos = (ipos.0 + dir_ipos.0, ipos.1 + dir_ipos.1);

        let (nx, ny) = neighbour_pos;

        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            continue;
        }

        let neighbour_pipe = input[ny as usize][nx as usize];

        match (dir_ipos, neighbour_pipe) {
            ((-1, 0), c) if c == '-' || c == 'L' || c == 'F' => {
                valid_directions.push(Direction::Left);
            }
            ((1, 0), c) if c == '-' || c == 'J' || c == '7' => {
                valid_directions.push(Direction::Right);
            }
            ((0, -1), c) if c == '|' || c == '7' || c == 'F' => {
                valid_directions.push(Direction::Up);
            }
            ((0, 1), c) if c == '|' || c == 'L' || c == 'J' => {
                valid_directions.push(Direction::Down);
            }
            _ => { /* Nothing to do */ }
        }
    }

    valid_directions
}

fn get_connected_pipes(
    input: &[Vec<char>],
    pos: (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    const OFFSETS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    let height = input.len();
    let width = input[0].len();

    let mut valid_neighbours = vec![];
    let current_pipe = input[pos.1][pos.0];

    OFFSETS
        .iter()
        .filter_map(|(offset_x, offset_y)| {
            let x_isize = pos.0 as isize + offset_x;
            let y_isize = pos.1 as isize + offset_y;

            if x_isize >= width as isize || y_isize >= height as isize {
                return None;
            }

            let x = std::convert::TryInto::<usize>::try_into(x_isize);
            let y = std::convert::TryInto::<usize>::try_into(y_isize);

            match (x, y) {
                (Ok(x), Ok(y)) => Some(((offset_x, offset_y), (x, y), input[y][x])),
                _ => None,
            }
        })
        .for_each(|((off_x, off_y), (pos_x, pos_y), c)| {
            let mut v = false;

            if current_pipe == '|' {
                if (*off_x, *off_y) == Direction::Up.into() {
                    if c == '7' || c == 'F' || c == '|' {
                        v = true;
                    }
                }
                if (*off_x, *off_y) == Direction::Down.into() {
                    if c == 'J' || c == 'L' || c == '|' {
                        v = true;
                    }
                }
            } else if current_pipe == '-' {
                if (*off_x, *off_y) == Direction::Left.into() {
                    if c == 'L' || c == 'F' || c == '-' {
                        v = true;
                    }
                }
                if (*off_x, *off_y) == Direction::Right.into() {
                    if c == 'J' || c == '7' || c == '-' {
                        v = true;
                    }
                }
            } else if current_pipe == 'L' {
                if (*off_x, *off_y) == Direction::Right.into() {
                    if c == 'J' || c == '7' || c == '-' {
                        v = true;
                    }
                }
                if (*off_x, *off_y) == Direction::Up.into() {
                    if c == 'F' || c == '7' || c == '|' {
                        v = true;
                    }
                }
            } else if current_pipe == 'J' {
                if (*off_x, *off_y) == Direction::Left.into() {
                    if c == 'L' || c == 'F' || c == '-' {
                        v = true;
                    }
                }
                if (*off_x, *off_y) == Direction::Up.into() {
                    if c == 'F' || c == '7' || c == '|' {
                        v = true;
                    }
                }
            } else if current_pipe == '7' {
                if (*off_x, *off_y) == Direction::Left.into() {
                    if c == 'F' || c == 'L' || c == '-' {
                        v = true;
                    }
                }
                if (*off_x, *off_y) == Direction::Down.into() {
                    if c == 'L' || c == 'J' || c == '|' {
                        v = true;
                    }
                }
            } else if current_pipe == 'F' {
                if (*off_x, *off_y) == Direction::Right.into() {
                    if c == '7' || c == 'J' || c == '-' {
                        v = true;
                    }
                }
                if (*off_x, *off_y) == Direction::Down.into() {
                    if c == 'L' || c == 'J' || c == '|' {
                        v = true;
                    }
                }
            }

            if v {
                valid_neighbours.push((pos_x, pos_y));
            }
        });

    println!("Valid neighbours for pos {pos: >3?}: {valid_neighbours:?}");

    valid_neighbours
        .into_iter()
        .collect_tuple()
        .expect("no two valid neighbours")
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let start_pos = get_start_pos(&input);

    let mut valid_directions = get_valid_directions(&input, start_pos);
    assert!(valid_directions.len() == 2);
    valid_directions.sort();

    let (d1, d2) = valid_directions
        .into_iter()
        .collect_tuple()
        .expect("no two initial valid directions");

    let start_pipe = match (d1, d2) {
        (Direction::Up, Direction::Down) => '|',
        (Direction::Right, Direction::Left) => '-',
        (Direction::Up, Direction::Right) => 'L',
        (Direction::Up, Direction::Left) => 'J',
        (Direction::Left, Direction::Down) => '7',
        (Direction::Right, Direction::Down) => 'F',
        _ => unreachable!(),
    };
    input[start_pos.1][start_pos.0] = start_pipe;

    let mut prev_pos_1 = start_pos;
    let mut prev_pos_2 = start_pos;
    let (mut curr_pos_1, mut curr_pos_2) = get_connected_pipes(&input, start_pos);
    let mut counter = 1;

    loop {
        let next_pos_1 = match get_connected_pipes(&input, curr_pos_1) {
            (a, v) if a == prev_pos_1 => v,
            (v, a) if a == prev_pos_1 => v,
            _ => unreachable!(),
        };

        prev_pos_1 = curr_pos_1;
        curr_pos_1 = next_pos_1;

        let next_pos_2 = match get_connected_pipes(&input, curr_pos_2) {
            (a, v) if a == prev_pos_2 => v,
            (v, a) if a == prev_pos_2 => v,
            _ => unreachable!(),
        };

        prev_pos_2 = curr_pos_2;
        curr_pos_2 = next_pos_2;

        counter += 1;

        if curr_pos_1 == curr_pos_2 {
            break;
        }
    }

    Box::new(counter)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = input
        .trim_end()
        .replace('L', "└")
        .replace('J', "┘")
        .replace('7', "┐")
        .replace('F', "┌")
        .replace('.', " ")
        .replace('-', "─")
        .replace('|', "│");

    println!("{input}");

    Box::new(0)
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
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
