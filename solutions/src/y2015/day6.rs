pub struct Day6;

crate::impl_day!("6", true);

#[derive(Debug)]
enum Operation {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    start: (i32, i32),
    end: (i32, i32),
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|l| {
            let operation = if l.starts_with("turn off") {
                Operation::TurnOff
            } else if l.starts_with("turn on") {
                Operation::TurnOn
            } else if l.starts_with("toggle") {
                Operation::Toggle
            } else {
                panic!("unknown operation: {}", input);
            };

            let mut split = l.split(" through ");
            let mut start_str = split.next().unwrap().split(' ').last().unwrap().split(',');
            let mut end_str = split.next().unwrap().split(',');

            let start = (
                start_str.next().unwrap().parse().unwrap(),
                start_str.next().unwrap().parse().unwrap(),
            );
            let end = (
                end_str.next().unwrap().parse().unwrap(),
                end_str.next().unwrap().parse().unwrap(),
            );

            Instruction {
                operation,
                start,
                end,
            }
        })
        .collect()
}

fn step(op: &Instruction, lights: &mut [i32]) {
    let Instruction {
        operation,
        start: (xs, ys),
        end: (xe, ye),
    } = op;

    for y in *ys..=*ye {
        for x in *xs..=*xe {
            let index = (y * 1000 + x) as usize;

            match operation {
                Operation::TurnOff => lights[index] = 0,
                Operation::TurnOn => lights[index] = 1,
                Operation::Toggle => lights[index] = 1 - lights[index],
            };
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut lights = vec![0; 1000 * 1000];

    for op in input {
        step(&op, &mut lights);
    }

    let res: i32 = lights.iter().sum();
    Box::new(res)
}

fn step2(op: &Instruction, lights: &mut [i32]) {
    let Instruction {
        operation,
        start: (xs, ys),
        end: (xe, ye),
    } = op;

    for y in *ys..=*ye {
        for x in *xs..=*xe {
            let index = (y * 1000 + x) as usize;

            match operation {
                Operation::TurnOn => lights[index] += 1,
                Operation::TurnOff => {
                    if lights[index] > 0 {
                        lights[index] -= 1;
                    }
                }
                Operation::Toggle => lights[index] += 2,
            };
        }
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut lights = vec![0; 1000 * 1000];

    for op in input {
        step2(&op, &mut lights);
    }

    let res: i32 = lights.iter().sum();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";

    #[test]
    fn test_part1() {
        assert_eq!((999_000 - 4).to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
