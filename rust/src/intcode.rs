use std::{io, str};

type Memory = Vec<i32>;

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!(format!("Cannot parse mode {}", code)),
        }
    }
    pub fn modes_from_instruction(instruction: i32) -> Vec<Self> {
        let mut modes = vec![Mode::Position, Mode::Position, Mode::Position];
        let mut mode_codes = instruction / 100;
        let mut index = 0;
        while mode_codes > 0 {
            modes[index] = Self::from_code(mode_codes % 10);
            mode_codes /= 10;
            index += 1;
        }

        modes
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

pub struct Computer {
    pub memory: Memory,
    pointer: usize,
    input: Option<i32>,
    output: Option<i32>,
}

impl Computer {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            pointer: 0,
            input: None,
            output: None,
        }
    }

    pub fn execute(&mut self, input: Option<i32>) -> Option<i32> {
        self.input = input;
        self.pointer = 0;
        while self.pointer < self.memory.len() {
            let instruction = self.memory[self.pointer];
            let modes = Mode::modes_from_instruction(instruction);
            let opcode = instruction % 100;
            if self.execute_opcode(opcode, modes).is_err() {
                break;
            }
        }

        self.output
    }

    fn execute_opcode(&mut self, opcode: i32, modes: Vec<Mode>) -> Result<(), ()> {
        match opcode {
            1 => {
                self.execute_operation(Operation::Add, modes);
                self.pointer += 4;
            }
            2 => {
                self.execute_operation(Operation::Multiply, modes);
                self.pointer += 4;
            }
            3 => {
                if let Some(input_value) = self.input {
                    let input_address = self.memory[self.pointer + 1] as usize;
                    self.memory[input_address] = input_value;
                } else {
                    panic!("ERROR! Input opcode specified, but no input specified");
                }
                self.pointer += 2;
            }
            4 => {
                self.output = Some(self.memory[self.memory[self.pointer + 1] as usize]);
                self.pointer += 2;
            }
            99 => return Err(()),
            _ => panic!("Unknown opcode: {:?}", opcode),
        }

        Ok(())
    }

    fn execute_operation(&mut self, operation: Operation, modes: Vec<Mode>) {
        let output_address = self.memory[self.pointer + 3] as usize;

        let operand1 = self.get_value(1, &modes[0]);
        let operand2 = self.get_value(2, &modes[1]);

        let result = match operation {
            Operation::Add => operand1 + operand2,
            Operation::Multiply => operand1 * operand2,
        };

        self.memory[output_address] = result;
    }

    fn get_value(&self, offset: usize, mode: &Mode) -> i32 {
        match mode {
            Mode::Position => self.memory[self.memory[self.pointer + offset] as usize],
            Mode::Immediate => self.memory[self.pointer + offset],
        }
    }
}

pub fn memory_from_string<T: io::BufRead>(input: T) -> io::Result<Memory> {
    let mut memory = vec![];
    for item in input.split(b',') {
        let bytes = item?;
        let serialized = str::from_utf8(&bytes)
            .expect("Could not create string")
            .trim();
        memory.push(serialized.parse().expect("Could not parse number"));
    }

    Ok(memory)
}

#[cfg(test)]
mod tests {
    use super::{Computer, Memory};

    #[test]
    fn test_execute() {
        assert_intcode_executed(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            None,
        );

        assert_intcode_executed(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99], None);

        assert_intcode_executed(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99], None);

        assert_intcode_executed(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801], None);

        assert_intcode_executed(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            None,
        );
    }

    #[test]
    fn test_execute_input() {
        assert_intcode_executed(vec![3, 3, 99, 10], vec![3, 3, 99, 40], Some(40));
    }

    #[test]
    fn test_execute_output() {
        let mut computer = Computer::new(vec![4, 0, 4, 7, 4, 3, 99, 12]);
        assert_eq!(Some(7), computer.execute(None));
    }

    #[test]
    fn test_execute_with_immediate_mode() {
        assert_intcode_executed(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99], None);
    }

    fn assert_intcode_executed(memory: Memory, expected: Memory, input: Option<i32>) {
        let mut computer = Computer::new(memory);
        computer.execute(input);
        assert_eq!(expected.as_slice(), computer.memory.as_slice());
    }
}
