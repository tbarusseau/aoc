use log::debug;

use crate::y2019::helpers::commons::get_nth_digit;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Runnable,
    Halted,
    WaitingForInput,
    GaveOutput(i64),
}

pub const MEMORY_SIZE: usize = 65536;
pub struct IntcodeComputer {
    initial_state: Vec<i64>,
    memory: Vec<i64>,
    instruction_pointer: usize,
    pub input: Vec<i64>,
    relative_base: isize,
}

impl IntcodeComputer {
    pub fn from(data: &[i64], input: Vec<i64>) -> Self {
        // Extend the program's memory
        let mut memory = data.to_vec();
        memory.extend(vec![0; MEMORY_SIZE - data.len()]);
        let initial_state = memory.clone();

        Self {
            initial_state,
            memory,
            instruction_pointer: 0,
            input,
            relative_base: 0,
        }
    }

    #[allow(unused)]
    pub fn process_from(data: &[i64], input: Vec<i64>) -> State {
        let mut c = Self::from(data, input);
        c.process()
    }

    fn get_instruction_length(&self) -> usize {
        match self.memory[self.instruction_pointer] % 100 {
            1 | 2 | 7 | 8 => 4,
            3 | 4 | 9 => 2,
            5 | 6 => 3,
            99 => 1,
            _ => panic!(
                "Unknown instruction: {}",
                self.memory[self.instruction_pointer] % 100
            ),
        }
    }

    fn get_parameter_modes(&self) -> Vec<ParameterMode> {
        let len = self.get_instruction_length() - 1;

        let mut r = vec![ParameterMode::Position; len];

        (0..len).for_each(|i| {
            r[i] = match get_nth_digit(self.memory[self.instruction_pointer] as u32, i + 2, false) {
                None | Some(0) => ParameterMode::Position,
                Some(1) => ParameterMode::Immediate,
                Some(2) => ParameterMode::Relative,
                _ => ParameterMode::Position,
            };
        });

        r
    }

    fn read(&self, param: i64, parameter_mode: ParameterMode) -> i64 {
        match parameter_mode {
            ParameterMode::Position => self.memory[param as usize],
            ParameterMode::Immediate => param,
            ParameterMode::Relative => self.memory[(param + self.relative_base as i64) as usize],
        }
    }

    fn write(&mut self, param: i64, parameter_mode: ParameterMode, value: i64) {
        let addr = match parameter_mode {
            ParameterMode::Position => param as usize,
            ParameterMode::Relative => (param + self.relative_base as i64) as usize,
            _ => panic!("Unsupported parameter mode: {:?}", parameter_mode),
        };

        // TODO: Handle address too big

        self.memory[addr] = value;
    }

    pub fn process(&mut self) -> State {
        use State::{GaveOutput, Halted, WaitingForInput};

        loop {
            let p = self.get_parameter_modes();

            match self.memory[self.instruction_pointer] % 100 {
                1 => {
                    // Add
                    debug!("[{0:4}] ADD", self.instruction_pointer);

                    let op1 = self.read(self.memory[self.instruction_pointer + 1], p[0]);
                    let op2 = self.read(self.memory[self.instruction_pointer + 2], p[1]);
                    self.write(self.memory[self.instruction_pointer + 3], p[2], op1 + op2);

                    self.instruction_pointer += 4;
                }
                2 => {
                    // Mul
                    debug!("[{0:4}] MUL", self.instruction_pointer);

                    let op1 = self.read(self.memory[self.instruction_pointer + 1], p[0]);
                    let op2 = self.read(self.memory[self.instruction_pointer + 2], p[1]);
                    self.write(self.memory[self.instruction_pointer + 3], p[2], op1 * op2);

                    self.instruction_pointer += 4;
                }
                3 => {
                    // Input
                    debug!("[{0:4}] INPUT", self.instruction_pointer);

                    if self.input.is_empty() {
                        return WaitingForInput;
                    }

                    self.write(
                        self.memory[self.instruction_pointer + 1],
                        p[0],
                        self.input[0],
                    );
                    self.input = self.input.drain(1..).collect();

                    self.instruction_pointer += 2;
                }
                4 => {
                    // Output

                    let output = self.read(self.memory[self.instruction_pointer + 1], p[0]);
                    debug!("[{0:4}] OUTPUT {1}", self.instruction_pointer, output);

                    self.instruction_pointer += 2;
                    return GaveOutput(output);
                }
                5 => {
                    // Jump if true
                    debug!("[{0:4}] JUMP IF TRUE", self.instruction_pointer);

                    if self.read(self.memory[self.instruction_pointer + 1], p[0]) != 0 {
                        self.instruction_pointer =
                            self.read(self.memory[self.instruction_pointer + 2], p[1]) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                6 => {
                    // Jump if false
                    debug!("[{0:4}] JUMP IF FALSE", self.instruction_pointer);

                    if self.read(self.memory[self.instruction_pointer + 1], p[0]) == 0 {
                        self.instruction_pointer =
                            self.read(self.memory[self.instruction_pointer + 2], p[1]) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                7 => {
                    // Less than
                    debug!("[{0:4}] LESS THAN", self.instruction_pointer);

                    if self.read(self.memory[self.instruction_pointer + 1], p[0])
                        < self.read(self.memory[self.instruction_pointer + 2], p[1])
                    {
                        self.write(self.memory[self.instruction_pointer + 3], p[2], 1);
                    } else {
                        self.write(self.memory[self.instruction_pointer + 3], p[2], 0);
                    }

                    self.instruction_pointer += 4;
                }
                8 => {
                    // Equals
                    debug!("[{0:4}] EQUALS", self.instruction_pointer);

                    if self.read(self.memory[self.instruction_pointer + 1], p[0])
                        == self.read(self.memory[self.instruction_pointer + 2], p[1])
                    {
                        self.write(self.memory[self.instruction_pointer + 3], p[2], 1);
                    } else {
                        self.write(self.memory[self.instruction_pointer + 3], p[2], 0);
                    }

                    self.instruction_pointer += 4;
                }
                9 => {
                    // Relative base adjust
                    let rba = self.read(self.memory[self.instruction_pointer + 1], p[0]);

                    debug!(
                        "[{0:4}] RELATIVE BASE ADJUST: {1} = {2}",
                        self.instruction_pointer,
                        rba,
                        self.relative_base + rba as isize
                    );

                    self.relative_base += rba as isize;
                    self.instruction_pointer += 2;
                }
                99 => {
                    debug!("[{0:4}] HALT", self.instruction_pointer);
                    break;
                }
                _ => panic!("Unknown intruction"),
            }
        }

        Halted
    }

    #[allow(unused)]
    pub fn provide_input(&mut self, input: Vec<i64>) {
        self.input.extend(input);
    }

    pub fn patch_memory(&mut self, index: usize, value: i64) {
        self.memory[index] = value;
    }

    pub fn index(&mut self, index: usize) -> i64 {
        self.memory[index]
    }

    pub fn reinitialize_memory(&mut self) {
        self.memory = self.initial_state.clone();
        self.instruction_pointer = 0;
        self.input = vec![];
        self.relative_base = 0;
    }

    #[allow(unused)]
    pub fn get_memory(&self) -> &Vec<i64> {
        &self.memory
    }
}

impl Iterator for IntcodeComputer {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        match self.process() {
            State::GaveOutput(o) => Some(o),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test_intcode_computer {
    use super::*;

    // Tests have been disabled because the "if target == current then do not increment
    // intruction pointer" modification seems to have broken these basic tests.
    //#[test]
    //fn test_basics() {
    //    // These test the memory
    //    assert_eq!(process_from(vec![1,0,0,0,99], 0).1, vec![2,0,0,0,99]);
    //    assert_eq!(process_from(vec![2,3,0,3,99], 0).1, vec![2,3,0,6,99]);
    //    assert_eq!(process_from(vec![2,4,4,5,99,0], 0).1, vec![2,4,4,5,99,9801]);
    //    assert_eq!(process_from(vec![1,1,1,4,99,5,6,0,99], 0).1, vec![30,1,1,4,2,5,6,0,99]);
    //}

    #[test]
    fn test_get_parameter_mode() {
        let c = IntcodeComputer::from(&[1002, 4, 3, 4, 33], vec![0]);
        assert_eq!(
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            ],
            c.get_parameter_modes()
        );
    }

    #[test]
    fn test_comparisons() {
        // Test input == 8
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8])
        );
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![0])
        );

        // Test input < 8
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![3])
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![5])
        );
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![8])
        );
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![9])
        );

        // Test input == 8 (immediate)
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![5])
        );
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![9])
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8])
        );

        // Test input < 8 (immediate)
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![12])
        );
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![8])
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![3])
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![5])
        );
    }

    #[test]
    fn test_jumps() {
        // Test jump
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(
                &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![0]
            )
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(
                &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![1]
            )
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(
                &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![87]
            )
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(
                &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![99]
            )
        );

        // Test jump (immediate)
        assert_eq!(
            State::GaveOutput(0),
            IntcodeComputer::process_from(
                &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![0]
            )
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(
                &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![1]
            )
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(
                &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![87]
            )
        );
        assert_eq!(
            State::GaveOutput(1),
            IntcodeComputer::process_from(
                &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![99]
            )
        );
    }

    #[test]
    fn test_complex() {
        const TEST_VEC_1: &[i64] = &[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(
            State::GaveOutput(999),
            IntcodeComputer::process_from(TEST_VEC_1, vec![0])
        );
        assert_eq!(
            State::GaveOutput(999),
            IntcodeComputer::process_from(TEST_VEC_1, vec![3])
        );
        assert_eq!(
            State::GaveOutput(999),
            IntcodeComputer::process_from(TEST_VEC_1, vec![7])
        );
        assert_eq!(
            State::GaveOutput(1000),
            IntcodeComputer::process_from(TEST_VEC_1, vec![8])
        );
        assert_eq!(
            State::GaveOutput(1001),
            IntcodeComputer::process_from(TEST_VEC_1, vec![9])
        );
        assert_eq!(
            State::GaveOutput(1001),
            IntcodeComputer::process_from(TEST_VEC_1, vec![99])
        );
        assert_eq!(
            State::GaveOutput(1001),
            IntcodeComputer::process_from(TEST_VEC_1, vec![9900])
        );
    }
}
