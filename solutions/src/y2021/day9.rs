use crate::{solver::Solver, utils::grid::Grid};

pub struct Day9;

crate::impl_day!("9", true);

fn process_input(input: &str) -> Grid<u32> {
    let input = input.trim();

    let mut grid = Grid::new();
    let lines = input.lines();

    for line in lines {
        grid.push_row(line.chars().flat_map(|c| char::to_digit(c, 10)).collect())
    }

    grid
}

fn get_low_points(grid: &Grid<u32>) -> Vec<u32> {
    let mut low_points = vec![];

    for (v, neighbours) in grid.iter_neighbours_ortho() {
        if neighbours.iter().count() == 0 || neighbours.iter().any(|&n| n <= v) {
            continue;
        }

        low_points.push(*v);
    }

    low_points
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = get_low_points(&input).iter().fold(0, |acc, n| acc + n + 1);
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(15.to_string(), *solve_part1(INPUT).to_string());
        assert_eq!(
            (6 * 4).to_string(),
            *solve_part1(
                "
5995
9999
9999
5995"
            )
            .to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
