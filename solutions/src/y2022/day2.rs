use crate::solver::Solver;

pub struct Day2;

crate::impl_day!("2", true);

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(
        input
            .lines()
            .map(|l| {
                let mut split = l.split(" ");
                let l = split.next().unwrap();
                let r = split.next().unwrap();

                let mut score = 0;
                match r {
                    "X" => {
                        score += 1;

                        match l {
                            "A" => score += 3,
                            "B" => score += 0,
                            "C" => score += 6,
                            _ => panic!(),
                        }
                    }
                    "Y" => {
                        score += 2;

                        match l {
                            "A" => score += 6,
                            "B" => score += 3,
                            "C" => score += 0,
                            _ => panic!(),
                        }
                    }
                    "Z" => {
                        score += 3;

                        match l {
                            "A" => score += 0,
                            "B" => score += 6,
                            "C" => score += 3,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }

                score
            })
            .sum::<i32>(),
    )
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(
        input
            .lines()
            .map(|l| {
                let mut split = l.split(" ");
                let l = split.next().unwrap();
                let r = split.next().unwrap();

                let mut score = 0;
                match r {
                    // Lose
                    "X" => {
                        score += 0;

                        match l {
                            "A" => score += 3,
                            "B" => score += 1,
                            "C" => score += 2,
                            _ => panic!(),
                        }
                    }
                    // Draw
                    "Y" => {
                        score += 3;

                        match l {
                            "A" => score += 1,
                            "B" => score += 2,
                            "C" => score += 3,
                            _ => panic!(),
                        }
                    }
                    // Win
                    "Z" => {
                        score += 6;

                        match l {
                            "A" => score += 2,
                            "B" => score += 3,
                            "C" => score += 1,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }

                score
            })
            .sum::<i32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn test_part1() {
        assert_eq!(15.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(12.to_string(), *solve_part2(INPUT).to_string());
    }
}
