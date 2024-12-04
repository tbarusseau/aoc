pub struct Day4;

crate::impl_day!("4", true);

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

fn count_occurrences(input: &[Vec<char>]) -> usize {
    let mut count = 0;
    let target = ['X', 'M', 'A', 'S'];
    let height = input.len();
    let width = input[0].len();

    // Helper function to check if XMAS starts at given position and direction
    let check_xmas = |row: isize, col: isize, row_dir: isize, col_dir: isize| -> bool {
        for i in 0..4 {
            let r = row + i * row_dir;
            let c = col + i * col_dir;
            if r < 0 || r >= height as isize || c < 0 || c >= width as isize {
                return false;
            }
            if input[r as usize][c as usize] != target[i as usize] {
                return false;
            }
        }
        true
    };

    // Check all positions and directions
    for row in 0..height {
        for col in 0..width {
            let current = input[row][col];
            if current != 'X' && current != 'S' {
                continue;
            }

            // Right
            if check_xmas(row as isize, col as isize, 0, 1) {
                count += 1;
            }
            // Left
            if check_xmas(row as isize, col as isize, 0, -1) {
                count += 1;
            }
            // Down
            if check_xmas(row as isize, col as isize, 1, 0) {
                count += 1;
            }
            // Up
            if check_xmas(row as isize, col as isize, -1, 0) {
                count += 1;
            }
            // Diagonal down-right
            if check_xmas(row as isize, col as isize, 1, 1) {
                count += 1;
            }
            // Diagonal down-left
            if check_xmas(row as isize, col as isize, 1, -1) {
                count += 1;
            }
            // Diagonal up-right
            if check_xmas(row as isize, col as isize, -1, 1) {
                count += 1;
            }
            // Diagonal up-left
            if check_xmas(row as isize, col as isize, -1, -1) {
                count += 1;
            }
        }
    }

    count
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = count_occurrences(&input);
    Box::new(res)
}

fn count_x_mas(input: &[Vec<char>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut count = 0;

    // Helper to check if position is in bounds and matches expected char
    let is_valid = |row: isize, col: isize, expected: char| -> bool {
        if row < 0 || row >= height as isize || col < 0 || col >= width as isize {
            return false;
        }
        input[row as usize][col as usize] == expected
    };

    // Check each starting position
    for (row, current_row) in input.iter().enumerate().take(height) {
        for (col, current_char) in current_row.iter().enumerate().take(width) {
            if *current_char != 'A' {
                continue;
            }

            let row = row as isize;
            let col = col as isize;

            if ((is_valid(row - 1, col - 1, 'M') && is_valid(row + 1, col + 1, 'S'))
                || (is_valid(row - 1, col - 1, 'S') && is_valid(row + 1, col + 1, 'M')))
                && ((is_valid(row + 1, col - 1, 'M') && is_valid(row - 1, col + 1, 'S'))
                    || (is_valid(row + 1, col - 1, 'S') && is_valid(row - 1, col + 1, 'M')))
            {
                count += 1;
            }
        }
    }

    count
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = count_x_mas(&input);
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(18.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!(9.to_string(), *solve_part2(INPUT).to_string());
    }
}
