use std::collections::HashMap;

use pathfinding::prelude::dijkstra;

pub struct Day15;

crate::impl_day!("15", true);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, h: &HashMap<Pos, usize>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;

        let mut s = Vec::new();

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|&(xi, yi)| {
                let xf = x + xi;
                let yf = y + yi;

                let target = Pos(xf, yf);

                if let Some(w) = h.get(&target) {
                    s.push((target, *w));
                }
            });

        s
    }
}

fn process_input(input: &str) -> HashMap<Pos, usize> {
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
    let (_, result) = dijkstra(&Pos(0, 0), |p| p.successors(&v), |p| *p == *goal).unwrap();

    Box::new(result)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut v = process_input(input);
    let orig = v.clone();
    let (goal, _) = orig.iter().max_by_key(|e| e.0.clone()).unwrap();

    // Enlarge the cave!
    let offset_x = goal.0 + 1;
    let offset_y = goal.1 + 1;

    for y in 0..5 {
        for x in 0..5 {
            if x == 0 && y == 0 {
                continue;
            }

            for (p, w) in orig.iter() {
                let &Pos(xl, yl) = p;

                let new_pos = Pos(xl + x * offset_x, yl + y * offset_y);
                let mut new_weight = (*w as i32 + x + y - 1) % 9 + 1;

                if new_weight > 9 {
                    new_weight = 1;
                }

                v.insert(new_pos, new_weight as usize);
            }
        }
    }

    let (goal, _) = v.iter().max_by_key(|e| e.0.clone()).unwrap();

    // Process like before
    let (_, result) = dijkstra(&Pos(0, 0), |p| p.successors(&v), |p| *p == *goal).unwrap();

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
