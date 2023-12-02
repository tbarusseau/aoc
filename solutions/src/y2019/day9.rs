pub struct Day9;

crate::impl_day!("9", true);

pub struct DayResult {
    result: Vec<i64>,
}

impl std::fmt::Display for DayResult {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self.result)
    }
}

fn process_input(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let c = crate::y2019::intcode_computer::IntcodeComputer::from(&input, vec![1]);
    let res = c.into_iter().collect::<Vec<i64>>().pop().unwrap();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let c = crate::y2019::intcode_computer::IntcodeComputer::from(&input, vec![2]);
    let res = DayResult {
        result: c.into_iter().collect::<Vec<i64>>(),
    };

    Box::new(res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        use crate::y2019::intcode_computer::{IntcodeComputer, State::*};

        let c = IntcodeComputer::from(
            &[
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            vec![],
        );
        assert_eq!(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            c.into_iter().collect::<Vec<i64>>()
        );
        let mut c = IntcodeComputer::from(&[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], vec![]);
        match c.process() {
            GaveOutput(r) => assert_eq!(16, r.to_string().chars().as_str().len()),
            _ => panic!("Computer gave no output"),
        }
        let mut c = IntcodeComputer::from(&[104, 1_125_899_906_842_624, 99], vec![]);
        match c.process() {
            GaveOutput(r) => assert_eq!(1_125_899_906_842_624, r),
            _ => panic!("Computer gave no output"),
        }
    }
}
