use itertools::Itertools;
use num::abs;

pub struct Day10;

crate::impl_day!("10", true);

#[derive(Clone, Debug)]
enum Operation {
    Addx(i32),
    Noop,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value == "noop" {
            Operation::Noop
        } else if value.starts_with("addx") {
            let v = value.split(' ').nth(1).unwrap();

            Operation::Addx(i32::from_str_radix(v, 10).unwrap())
        } else {
            panic!("Unknown operation: {}", value);
        }
    }
}

impl Operation {
    pub fn duration(&self) -> i32 {
        match self {
            Operation::Addx(_) => 2,
            Operation::Noop => 1,
        }
    }
}

fn process_input(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect()
}

struct State {
    pub register: i32,
    pub cycle_count: i32,
    current_op: Option<Operation>,
    current_op_cycle_count: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            register: 1,
            cycle_count: 0,
            current_op: None,
            current_op_cycle_count: 0,
        }
    }
}

impl State {
    pub fn feed(&mut self, operation: &Operation) {
        self.current_op = Some(operation.clone());
        self.current_op_cycle_count = 0;
    }

    pub fn tick(&mut self) {
        self.cycle_count += 1;
        self.current_op_cycle_count += 1;
    }

    pub fn finished(&mut self) -> bool {
        let res = self.current_op.as_ref().unwrap().duration() == self.current_op_cycle_count;

        if res {
            match self.current_op.as_ref().unwrap() {
                Operation::Addx(val) => self.register += val,
                Operation::Noop => { /* Nothing to do. */ }
            }
        }

        res
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let operations = process_input(input);
    let mut state = State::default();
    let mut signal_strength = 0;

    for op in operations {
        // println!("[{}] Feed: {:?}", state.cycle_count, op);
        state.feed(&op);
        while !state.finished() {
            // println!("[{}] Tick", state.cycle_count);
            state.tick();

            if (state.cycle_count + 20) % 40 == 0 {
                // Sample the signal strength.
                println!(
                    "[{}] Sampling: {} * {} = {}",
                    state.cycle_count,
                    state.cycle_count,
                    state.register,
                    state.cycle_count * state.register
                );
                signal_strength += state.cycle_count * state.register;
            }
        }

        // println!("[{}] Finished", state.cycle_count);
    }

    Box::new(signal_strength)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let operations = process_input(input);
    let mut state = State::default();
    let mut pixels: Vec<char> = Vec::with_capacity(40 * 6);

    for op in operations {
        state.feed(&op);
        while !state.finished() {
            state.tick();

            let State {
                register: middle_cursor_pos,
                cycle_count,
                ..
            } = state;

            let x = cycle_count % 40;
            let dist = abs(x - middle_cursor_pos - 1);

            if dist <= 1 {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
        }
    }

    let initial_string_collect = pixels.iter().collect::<String>();
    let chunks = initial_string_collect.chars().chunks(40);
    let mut s = String::with_capacity(41 * 6 + 1);

    s.push('\n');

    for chunk in &chunks {
        for c in chunk {
            s.push(c);
        }

        s.push('\n');
    }

    Box::new(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn test_part1() {
        assert_eq!(13140.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            String::from(
                r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######.....
"#
            ),
            *solve_part2(INPUT).to_string()
        );
    }
}
