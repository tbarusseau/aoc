use crate::solver::Solver;

pub struct Day7;

crate::impl_day!("7", false);

#[derive(Debug)]
enum IpSequence {
    Regular(String),
    Hypernet(String),
}

fn process_input(input: &str) -> Vec<Vec<IpSequence>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut v = vec![];
            let mut s = String::new();

            for ch in l.chars() {
                match ch {
                    '[' => {
                        v.push(IpSequence::Regular(s.clone()));
                        s.clear();
                    }
                    ']' => {
                        v.push(IpSequence::Hypernet(s.clone()));
                        s.clear();
                    }
                    'a'..='z' => s.push(ch),
                    _ => panic!(),
                }
            }

            v.push(IpSequence::Regular(s));

            v
        })
        .collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    println!("{:?}", input);

    let res = "Part 1 not done";
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

    const INPUT: &str = r#""#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
