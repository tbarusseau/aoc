pub struct Day4;

crate::impl_day!("4", true);

use colored::Colorize;
use grid::Grid;

#[derive(Debug)]
struct DrawOrder(Vec<i32>);

struct Board(Grid<(i32, bool)>);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for i in 0..self.0.rows() {
            self.0.iter_row(i).for_each(|(a, b)| {
                if *b {
                    out.push_str(&format!("{} ", a.to_string().blue()));
                } else {
                    out.push_str(&format!("{a} "));
                }
            });

            out.push('\n');
        }

        write!(f, "{out}")
    }
}

fn process_input(input: &str) -> (DrawOrder, Vec<Board>) {
    let mut lines = input.lines();
    let mut grid: Grid<(i32, bool)> = Grid::new(5, 5);
    grid.clear();
    let mut row_index = 0;
    let mut boards = vec![];

    let draw_order = DrawOrder(
        lines
            .next()
            .unwrap()
            .split(',')
            .flat_map(str::parse)
            .collect(),
    );
    lines.next();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let numbers = line.split(' ').flat_map(str::parse).collect::<Vec<i32>>();
        grid.push_row(numbers.iter().map(|&n| (n, false)).collect());

        row_index += 1;

        if row_index == 5 {
            boards.push(Board(grid.clone()));
            grid.clear();

            row_index = 0;
        }
    }

    (draw_order, boards)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (ref mut draw_order, ref mut boards) = process_input(input);

    let mut winning_board = None;
    let mut last_draw = 0;

    'outer: for draw in &draw_order.0 {
        last_draw = *draw;

        for b in boards.iter_mut() {
            b.0.iter_mut().for_each(|(n, b)| {
                if n == draw {
                    *b = true;
                }
            });
        }

        for board in boards.iter() {
            for i in 0..5 {
                if board.0.iter_row(i).all(|(_, b)| *b) || board.0.iter_col(i).all(|(_, b)| *b) {
                    winning_board = Some(board.0.clone());
                    break 'outer;
                }
            }
        }
    }

    let sum_unmarked_numbers =
        winning_board
            .unwrap()
            .iter()
            .fold(0, |acc, (n, b)| if *b { acc } else { acc + n });

    let res = sum_unmarked_numbers * last_draw;
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (ref mut draw_order, ref mut boards) = process_input(input);

    let mut winning_boards = vec![];
    let mut last_winning_board = None;
    let mut last_draw = 0;

    'outer: for draw in &draw_order.0 {
        last_draw = *draw;

        for b in boards.iter_mut() {
            b.0.iter_mut().for_each(|(n, b)| {
                if n == draw {
                    *b = true;
                }
            });
        }

        for (index, board) in boards.iter().enumerate() {
            for i in 0..5 {
                if board.0.iter_row(i).all(|(_, b)| *b) || board.0.iter_col(i).all(|(_, b)| *b) {
                    if !winning_boards.contains(&index) {
                        winning_boards.push(index);
                    }

                    if winning_boards.len() == boards.len() {
                        last_winning_board = Some(board.0.clone());
                        break 'outer;
                    }
                }
            }
        }
    }

    let sum_unmarked_numbers =
        last_winning_board
            .unwrap()
            .iter()
            .fold(0, |acc, (n, b)| if *b { acc } else { acc + n });

    let res = sum_unmarked_numbers * last_draw;
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn test_part1() {
        assert_eq!(4512.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1924.to_string(), *solve_part2(INPUT).to_string());
    }
}
