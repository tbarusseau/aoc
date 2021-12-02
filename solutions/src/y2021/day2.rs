use crate::solver::Solver;

pub struct Day2;

crate::impl_day!("2", true);

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn process_input(input: &str) -> Vec<Instruction> {
    use Instruction::*;

    input
        .lines()
        .flat_map(|line| {
            let mut split = line.split(' ');
            let instruction = split.next().unwrap();
            let value = split.next().unwrap().parse().unwrap();

            match instruction {
                "forward" => Some(Forward(value)),
                "down" => Some(Down(value)),
                "up" => Some(Up(value)),
                _ => None,
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut pos = (0, 0);
    input.iter().for_each(|instruction| match instruction {
        Instruction::Forward(v) => pos.0 += v,
        Instruction::Down(v) => pos.1 += v,
        Instruction::Up(v) => pos.1 -= v,
    });

    Box::new(pos.0 * pos.1)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut aim = 0;
    let mut pos = (0, 0);
    input.iter().for_each(|instruction| match instruction {
        Instruction::Forward(v) => {
            pos.0 += v;
            pos.1 += aim * v;
        }
        Instruction::Down(v) => aim += v,
        Instruction::Up(v) => aim -= v,
    });

    Box::new(pos.0 * pos.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_part1() {
        assert_eq!(150.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(900.to_string(), *solve_part2(INPUT).to_string());
    }
}
