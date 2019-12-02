enum Operation {
    Add,
    Multiply,
}

pub fn execute_intcode(memory: &mut Vec<u32>) {
    let mut pointer = 0;
    while pointer < memory.len() {
        let opcode = memory[pointer];
        match opcode {
            1 => execute_instruction(memory, pointer, Operation::Add),
            2 => execute_instruction(memory, pointer, Operation::Multiply),
            99 => break,
            _ => panic!("Unknown opcode: {:?}", opcode),
        }

        pointer += 4;
    }
}

fn execute_instruction(memory: &mut Vec<u32>, pointer: usize, operation: Operation) {
    let first_input_address = memory[pointer + 1] as usize;
    let second_input_address = memory[pointer + 2] as usize;
    let output_address = memory[pointer + 3] as usize;

    let operand1 = memory[first_input_address];
    let operand2 = memory[second_input_address];

    let result = match operation {
        Operation::Add => operand1 + operand2,
        Operation::Multiply => operand1 * operand2,
    };

    memory[output_address] = result;
}

#[cfg(test)]
mod tests {
    use super::execute_intcode;

    #[test]
    fn test_execute_intcode() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_intcode_executed(
            &mut memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );

        memory = vec![1, 0, 0, 0, 99];
        assert_intcode_executed(&mut memory, vec![2, 0, 0, 0, 99]);

        memory = vec![2, 3, 0, 3, 99];
        assert_intcode_executed(&mut memory, vec![2, 3, 0, 6, 99]);

        memory = vec![2, 4, 4, 5, 99, 0];
        assert_intcode_executed(&mut memory, vec![2, 4, 4, 5, 99, 9801]);

        memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_intcode_executed(&mut memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    fn assert_intcode_executed(memory: &mut Vec<u32>, expected: Vec<u32>) {
        execute_intcode(memory);
        assert_eq!(expected.as_slice(), memory.as_slice());
    }
}
