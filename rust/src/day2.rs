enum Operation {
    Add,
    Multiply,
}

pub fn execute_intcode(instructions: &mut Vec<u32>) {
    let mut position = 0;
    while position < instructions.len() {
        let opcode = instructions[position];
        match opcode {
            1 => execute_operation(instructions, position, Operation::Add),
            2 => execute_operation(instructions, position, Operation::Multiply),
            99 => break,
            _ => panic!("Unknown opcode: {:?}", opcode),
        }

        position += 4;
    }
}

fn execute_operation(instructions: &mut Vec<u32>, position: usize, operation: Operation) {
    let first_input_position = instructions[position + 1] as usize;
    let second_input_position = instructions[position + 2] as usize;
    let output_position = instructions[position + 3] as usize;

    let operand1 = instructions[first_input_position];
    let operand2 = instructions[second_input_position];

    let result = match operation {
        Operation::Add => operand1 + operand2,
        Operation::Multiply => operand1 * operand2,
    };

    instructions[output_position] = result;
}

#[cfg(test)]
mod tests {
    use super::execute_intcode;

    #[test]
    fn test_execute_intcode() {
        let mut instructions = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_intcode_executed(
            &mut instructions,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );

        instructions = vec![1, 0, 0, 0, 99];
        assert_intcode_executed(&mut instructions, vec![2, 0, 0, 0, 99]);

        instructions = vec![2, 3, 0, 3, 99];
        assert_intcode_executed(&mut instructions, vec![2, 3, 0, 6, 99]);

        instructions = vec![2, 4, 4, 5, 99, 0];
        assert_intcode_executed(&mut instructions, vec![2, 4, 4, 5, 99, 9801]);

        instructions = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_intcode_executed(&mut instructions, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    fn assert_intcode_executed(instructions: &mut Vec<u32>, expected: Vec<u32>) {
        execute_intcode(instructions);
        assert_eq!(expected.as_slice(), instructions.as_slice());
    }
}
