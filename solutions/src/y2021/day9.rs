use itertools::Itertools;

use crate::utils::grid::Grid;

pub struct Day9;

crate::impl_day!("9", true);

fn process_input(input: &str) -> Grid<u32> {
    let input = input.trim();

    let mut grid = Grid::new();
    let lines = input.lines();

    for line in lines {
        grid.push_row(line.chars().filter_map(|c| char::to_digit(c, 10)).collect());
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

fn get_low_point_positions<T>(grid: &Grid<T>) -> Vec<usize>
where
    T: PartialOrd,
{
    let mut low_point_positions = vec![];

    for (index, (v, neighbours)) in grid.iter_neighbours_ortho().enumerate() {
        if neighbours.iter().count() != 0 && neighbours.iter().all(|&n| n > v) {
            low_point_positions.push(index);
        }
    }

    low_point_positions
}

fn iter_neighbours(
    grid: &Grid<u32>,
    index: usize,
    size: &mut usize,
    visited_indices: &mut Vec<usize>,
) {
    if visited_indices.contains(&index) {
        return;
    }

    visited_indices.push(index);

    let x = index % grid.cols();
    let y = index / grid.cols();
    let v = *grid.get(x, y).unwrap();

    if v == 9 {
        return;
    }

    *size += 1;

    let neighbours = grid.get_ortho_neighbours(x, y).unwrap();
    if let Some(up) = neighbours.up {
        if *up > v {
            iter_neighbours(grid, (y - 1) * grid.cols() + x, size, visited_indices);
        }
    }
    if let Some(right) = neighbours.right {
        if *right > v {
            iter_neighbours(grid, y * grid.cols() + x + 1, size, visited_indices);
        }
    }
    if let Some(down) = neighbours.down {
        if *down > v {
            iter_neighbours(grid, (y + 1) * grid.cols() + x, size, visited_indices);
        }
    }
    if let Some(left) = neighbours.left {
        if *left > v {
            iter_neighbours(grid, y * grid.cols() + x - 1, size, visited_indices);
        }
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let low_point_positions = get_low_point_positions(&input);

    let mut basin_sizes = vec![];
    for index in low_point_positions {
        let mut size = 0;
        let mut visited_indices = vec![];
        iter_neighbours(&input, index, &mut size, &mut visited_indices);

        if size > 0 {
            basin_sizes.push(size);
        }
    }

    let res = basin_sizes
        .iter()
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>();
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
        assert_eq!(1134.to_string(), *solve_part2(INPUT).to_string());
    }
}
