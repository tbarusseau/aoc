use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

pub struct Day17;

crate::impl_day!("17", true);

fn process_input(input: &str) -> Matrix<u32> {
    Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10))),
    )
    .expect("couldn't build pathfinding matrix")
}

fn solve(input: &Matrix<u32>, min_moves: usize, max_moves: usize) -> u32 {
    // State used: (pos_x, pos_y), (offset_x, offset_y), consecutive_straight_moves

    let goal_pos = (input.rows - 1, input.columns - 1);

    dijkstra(
        &((0, 0), (0, 0), 0),
        |&(pos, (offset_x, offset_y), consecutive_straight_moves)| {
            let mut next_neighbours = Vec::with_capacity(3);
            let mut extend_with_neighbour = |dir, consecutive_straight_moves| {
                next_neighbours.extend(&input.move_in_direction(pos, dir).map(|neighbour_pos| {
                    (
                        (neighbour_pos, dir, consecutive_straight_moves),
                        input[neighbour_pos],
                    )
                }));
            };

            if consecutive_straight_moves < max_moves && (offset_x != 0 || offset_y != 0) {
                // Keep moving in the same direction
                extend_with_neighbour((offset_x, offset_y), consecutive_straight_moves + 1);
            }

            if consecutive_straight_moves >= min_moves {
                // Allow to turn
                extend_with_neighbour((-offset_y, -offset_x), 1);
                extend_with_neighbour((offset_y, offset_x), 1);
            } else if consecutive_straight_moves == 0 {
                // Initial scenario: go both bottom and right.
                extend_with_neighbour((1, 0), 1);
                extend_with_neighbour((0, 1), 1);
            }

            next_neighbours
        },
        |&(pos, _, consecutive_straight_moves)| {
            pos == goal_pos && consecutive_straight_moves >= min_moves
        },
    )
    .expect("no path found")
    .1
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let res = solve(&process_input(input), 1, 3);

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let res = solve(&process_input(input), 4, 10);

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const INPUT2: &str = r"111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_part1() {
        assert_eq!(102.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(71.to_string(), *solve_part2(INPUT2).to_string());
        assert_eq!(94.to_string(), *solve_part2(INPUT).to_string());
    }
}
