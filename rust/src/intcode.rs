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
    LessThan,
    Equals,
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
            1 => self.execute_operation(Operation::Add, modes),
            2 => self.execute_operation(Operation::Multiply, modes),
            3 => self.set_input(),
            4 => self.set_output(modes),
            5 => self.set_pointer(true, modes),
            6 => self.set_pointer(false, modes),
            7 => self.execute_operation(Operation::LessThan, modes),
            8 => self.execute_operation(Operation::Equals, modes),
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
            Operation::LessThan => {
                if operand1 < operand2 {
                    1
                } else {
                    0
                }
            }
            Operation::Equals => {
                if operand1 == operand2 {
                    1
                } else {
                    0
                }
            }
        };

        self.memory[output_address] = result;
        self.pointer += 4;
    }

    fn set_input(&mut self) {
        if let Some(input_value) = self.input {
            let input_address = self.memory[self.pointer + 1] as usize;
            self.memory[input_address] = input_value;
        } else {
            panic!("ERROR! Input opcode specified, but no input specified");
        }
        self.pointer += 2;
    }

    fn set_output(&mut self, modes: Vec<Mode>) {
        self.output = Some(self.get_value(1, &modes[0]));
        self.pointer += 2;
    }

    fn set_pointer(&mut self, test: bool, modes: Vec<Mode>) {
        let value = self.get_value(1, &modes[0]);
        if (value != 0) == test {
            self.pointer = self.get_value(2, &modes[1]) as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn get_value(&self, offset: usize, mode: &Mode) -> i32 {
        match mode {
            Mode::Position => self.memory[self.memory[self.pointer + offset] as usize],
            Mode::Immediate => self.memory[self.pointer + offset],
        }
    }
}

pub fn memory_from_io<T: io::BufRead>(input: T) -> io::Result<Memory> {
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
        assert_intcode_output(vec![4, 0, 4, 7, 4, 3, 99, 12], None, 7);

        let memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_intcode_output(memory.clone(), Some(4), 999);
        assert_intcode_output(memory.clone(), Some(8), 1000);
        assert_intcode_output(memory.clone(), Some(12), 1001);
    }

    #[test]
    fn test_execute_with_immediate_mode() {
        assert_intcode_executed(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99], None);
    }

    #[test]
    fn test_less_than() {
        assert_intcode_output(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], Some(1), 1);
        assert_intcode_output(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], Some(8), 0);
        assert_intcode_output(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], Some(9), 0);

        assert_intcode_output(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], Some(1), 1);
        assert_intcode_output(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], Some(8), 0);
        assert_intcode_output(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], Some(9), 0);
    }

    #[test]
    fn test_equals() {
        assert_intcode_output(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], Some(1), 0);
        assert_intcode_output(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], Some(8), 1);
        assert_intcode_output(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], Some(9), 0);

        assert_intcode_output(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], Some(1), 0);
        assert_intcode_output(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], Some(8), 1);
        assert_intcode_output(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], Some(9), 0);
    }

    #[test]
    fn test_jump() {
        assert_intcode_output(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            Some(0),
            0,
        );
        assert_intcode_output(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            Some(1),
            1,
        );

        assert_intcode_output(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            Some(0),
            0,
        );
        assert_intcode_output(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            Some(1),
            1,
        );
    }

    fn assert_intcode_executed(memory: Memory, expected: Memory, input: Option<i32>) {
        let mut computer = Computer::new(memory);
        computer.execute(input);
        assert_eq!(expected.as_slice(), computer.memory.as_slice());
    }

    fn assert_intcode_output(memory: Memory, input: Option<i32>, expected: i32) {
        let mut computer = Computer::new(memory);
        assert_eq!(Some(expected), computer.execute(input));
    }
}
