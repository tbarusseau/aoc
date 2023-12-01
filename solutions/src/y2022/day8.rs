pub struct Day8;

crate::impl_day!("8", true);

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10))
                .flatten()
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let forest = process_input(input);
    let height = forest.len();
    let width = forest[0].len();

    let mut res = 0;

    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                res += 1;

                continue;
            }

            let tree = forest[y][x];
            for dir in DIRECTIONS {
                if is_shorter_rec(&forest, tree, (x, y), *dir, (width, height)) {
                    res += 1;
                    break;
                }
            }
        }
    }

    Box::new(res)
}

fn is_shorter_rec(
    forest: &[Vec<u32>],
    tree: u32,
    pos: (usize, usize),
    dir: (isize, isize),
    size: (usize, usize),
) -> bool {
    let next_x = dir.0 + pos.0 as isize;
    let next_y = dir.1 + pos.1 as isize;

    if next_x < 0 || next_x >= size.0 as isize || next_y < 0 || next_y >= size.1 as isize {
        return true;
    }

    let current_height = forest[next_y as usize][next_x as usize];
    if current_height >= tree {
        return false;
    }

    return is_shorter_rec(forest, tree, (next_x as usize, next_y as usize), dir, size);
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut res = 0;

    let forest = process_input(input);
    let height = forest.len();
    let width = forest[0].len();

    for y in 0..height {
        for x in 0..width {
            let mut score = 1;
            for dir in DIRECTIONS {
                let tree = forest[y][x];
                let viewing_distance =
                    get_viewing_distance_rec(&forest, tree, (x, y), *dir, (width, height));
                score *= viewing_distance;
            }

            if score > res {
                res = score;
            }
        }
    }

    Box::new(res)
}

fn get_viewing_distance_rec(
    forest: &[Vec<u32>],
    tree: u32,
    pos: (usize, usize),
    dir: (isize, isize),
    size: (usize, usize),
) -> i32 {
    let next_x = dir.0 + pos.0 as isize;
    let next_y = dir.1 + pos.1 as isize;

    if next_x < 0 || next_x >= size.0 as isize || next_y < 0 || next_y >= size.1 as isize {
        0
    } else {
        let next_height = forest[next_y as usize][next_x as usize];
        if next_height >= tree {
            1
        } else {
            1 + get_viewing_distance_rec(
                forest,
                tree,
                (next_x as usize, next_y as usize),
                dir,
                size,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part1() {
        assert_eq!(21.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(8.to_string(), *solve_part2(INPUT).to_string());
    }
}
