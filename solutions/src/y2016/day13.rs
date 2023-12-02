use std::collections::{HashMap, HashSet};

use colored::Colorize;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

pub struct Day13;

crate::impl_day!("13", true);

fn process_input(input: &str) -> i32 {
    input.trim().parse().unwrap()
}

fn is_wall(x: i32, y: i32, number: i32) -> bool {
    if x < 0 || y < 0 {
        return true;
    }

    let formula_result = x * x + 3 * x + 2 * x * y + y + y * y;
    let with_number = formula_result + number;

    with_number.count_ones() % 2 == 1
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    solve_p1_with_target(input, (31, 39))
}

fn compute_valid_successors(pos: &(i32, i32), favorite_number: i32) -> Vec<(i32, i32)> {
    let mut v = vec![];

    for pos_offset in [
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ] {
        if !is_wall(pos_offset.0, pos_offset.1, favorite_number) {
            v.push(pos_offset);
        }
    }

    v
}

fn solve_p1_with_target(input: &str, target: (i32, i32)) -> Box<dyn std::fmt::Display> {
    let favorite_number = process_input(input);

    let result = dijkstra(
        &(1, 1),
        |p| {
            compute_valid_successors(p, favorite_number)
                .into_iter()
                .map(|v| (v, 1))
                .collect_vec()
        },
        |p| *p == target,
    )
    .unwrap();

    Box::new(result.1)
}

fn recursively_visit_valid_neighbours(
    visited_pos: &mut HashMap<(i32, i32), i32>,
    current_pos: &(i32, i32),
    favorite_number: i32,
    depth: i32,
    max_depth: i32,
) {
    // This position was already handled by another call of the recursive function.
    // Do not count it twice, stop the recursion here.
    // if visited_pos.contains(current_pos) {
    //     return;
    // }

    visited_pos.insert(*current_pos, depth);

    let valid_neighbours = compute_valid_successors(current_pos, favorite_number)
        .into_iter()
        .filter(|p| visited_pos.get(p).map_or(true, |v| depth < *v))
        .collect_vec();

    // println!(
    //     "[{depth: >2}] Pos: {current_pos:?}, valid neighbours: {:?}",
    //     valid_neighbours.clone().collect_vec()
    // );

    if depth < max_depth {
        // Count current pos + other valid neighbours
        valid_neighbours.iter().for_each(|p| {
            recursively_visit_valid_neighbours(
                visited_pos,
                p,
                favorite_number,
                depth + 1,
                max_depth,
            )
        })
    }
}

#[allow(dead_code)]
fn print_maze(favorite_number: i32, side_length: i32, visited_pos: &HashSet<(i32, i32)>) {
    let upper_digit = (side_length / 10 + 1) * 10;

    println!("   {}", (0..=(upper_digit / 10)).join("         "));
    for y in 0..=upper_digit {
        print!("{: >2} ", y);
        for x in 0..=upper_digit {
            if is_wall(x, y, favorite_number) {
                print!("{}", "#".dimmed());
            } else if x == 1 && y == 1 {
                print!("{}", "x".bold())
            } else if visited_pos.contains(&(x, y)) {
                print!("{}", "o".bold().green())
            } else {
                print!("{}", ".".dimmed());
            }
        }

        println!();
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut visited_pos: HashMap<(i32, i32), i32> = HashMap::new();
    let current_pos = (1, 1);
    let favorite_number = process_input(input);
    let depth = 0;

    const MAX_DEPTH: i32 = 50;

    recursively_visit_valid_neighbours(
        &mut visited_pos,
        &current_pos,
        favorite_number,
        depth,
        MAX_DEPTH,
    );

    // print_maze(favorite_number, MAX_DEPTH, &visited_pos);

    Box::new(visited_pos.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"10"#;

    #[test]
    fn test_part1() {
        assert_eq!(
            11.to_string(),
            *solve_p1_with_target(INPUT, (7, 4)).to_string()
        );
    }
}
