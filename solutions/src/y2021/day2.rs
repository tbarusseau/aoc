use pest_consume::{match_nodes, Error, Parser};
type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

pub struct Day2;

crate::impl_day!("2", true);

enum Direction {
    Forward,
    Down,
    Up,
}

struct Instruction {
    direction: Direction,
    value: i32,
}

#[derive(Parser)]
#[grammar = "y2021/grammars/day2.pest"]
struct InstructionsParser;

#[allow(clippy::unnecessary_wraps)]
#[pest_consume::parser]
impl InstructionsParser {
    fn EOI(input: Node) -> Result<()> {
        Ok(())
    }

    fn direction(input: Node) -> Result<Direction> {
        use Direction::{Down, Forward, Up};

        match input.as_str() {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => panic!(),
        }
    }

    fn value(input: Node) -> Result<i32> {
        input.as_str().parse::<i32>().map_err(|e| input.error(e))
    }

    fn instruction(input: Node) -> Result<Instruction> {
        Ok(match_nodes!(input.into_children();
            [direction(d), value(v)] => Instruction { direction: d, value: v },
        ))
    }

    fn instructions_set(input: Node) -> Result<Vec<Instruction>> {
        Ok(match_nodes!(input.into_children();
            [instruction(i).., EOI(())] => i.collect(),
        ))
    }
}

#[allow(clippy::result_large_err)]
fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    let inputs = InstructionsParser::parse(Rule::instructions_set, input).unwrap();
    let input = inputs.single().unwrap();

    InstructionsParser::instructions_set(input)
}

fn process_input(input: &str) -> Vec<Instruction> {
    parse_input(input).unwrap()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut pos = (0, 0);
    input
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Forward => pos.0 += instruction.value,
            Direction::Down => pos.1 += instruction.value,
            Direction::Up => pos.1 -= instruction.value,
        });

    Box::new(pos.0 * pos.1)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut aim = 0;
    let mut pos = (0, 0);
    input
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Forward => {
                pos.0 += instruction.value;
                pos.1 += aim * instruction.value;
            }
            Direction::Down => aim += instruction.value,
            Direction::Up => aim -= instruction.value,
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
