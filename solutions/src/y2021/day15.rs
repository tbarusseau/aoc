use pathfinding::prelude::dijkstra;

use crate::solver::Solver;

pub struct Day15;

crate::impl_day!("15", true);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, v: &[(Pos, usize)], width: i32, height: i32) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;

        let mut s = Vec::new();

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|&(xi, yi)| {
                let xf = x + xi;
                let yf = y + yi;

                if xf < 0 || xf >= width || yf < 0 || yf >= height {
                    return;
                }

                let target = (yf * width + xf) as usize;

                if let Some((p, w)) = v.get(target) {
                    s.push((p.clone(), *w));
                }
            });

        s
    }
}

fn process_input(input: &str) -> Vec<(Pos, usize)> {
    let input = input.trim();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    let w = c.to_digit(10).unwrap();

                    (Pos(x as i32, y as i32), w as usize)
                })
                .collect::<Vec<(Pos, usize)>>()
        })
        .collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let v = process_input(input);

    let (goal, _) = v.iter().max_by_key(|e| e.0.clone()).unwrap();
    let (_, result) = dijkstra(
        &Pos(0, 0),
        |p| p.successors(&v, goal.0 + 1, goal.1 + 1),
        |p| *p == *goal,
    )
    .unwrap();

    Box::new(result)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut v = process_input(input);
    let orig = v.clone();
    let (goal, _) = orig.iter().max_by_key(|e| e.0.clone()).unwrap();

    // Enlarge the cave!
    let &Pos(offset_x, offset_y) = goal;

    for y in 0..5 {
        for x in 0..5 {
            if x == 0 && y == 0 {
                continue;
            }

            // println!("[{x}, {y}]");

            for (p, w) in orig.iter() {
                let &Pos(xl, yl) = p;

                let new_pos = Pos(xl + x * (offset_x + 1), yl + y * (offset_y + 1));
                let mut new_weight = (*w as i32 + x + y - 1) % 9 + 1;

                println!(
                    "[{x}, {y}] new_pos: {:?}, new_weight: {}",
                    new_pos, new_weight
                );

                if new_weight > 9 {
                    new_weight = 1;
                }

                v.push((new_pos, new_weight as usize));
            }
        }
    }

    // println!("New map: {:?}", v);

    let (goal, _) = v.iter().max_by_key(|e| e.0.clone()).unwrap();

    // Process like before
    let (_, result) = dijkstra(
        &Pos(0, 0),
        |p| p.successors(&v, goal.0 + 1, goal.1 + 1),
        |p| *p == *goal,
    )
    .unwrap();

    Box::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn test_part1() {
        assert_eq!(40.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(315.to_string(), *solve_part2(INPUT).to_string());
    }
}
