mod day1;
mod day2;

use day2::execute_intcode;
use std::io;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let mut original_memory: Vec<u32> = vec![];
    let stdin = io::stdin();
    for item in stdin.lock().split(b',') {
        let bytes = item?;
        let serialized = str::from_utf8(&bytes)
            .expect("Could not create string")
            .replace("\n", "");
        original_memory.push(serialized.parse().expect("Could not parse number"));
    }
    'outer: for i in 0..99 {
        for j in 0..99 {
            let mut instructions = original_memory.clone();
            instructions[1] = i;
            instructions[2] = j;
            execute_intcode(&mut instructions);
            if instructions[0] == 19690720 {
                println!("noun: {:?}, verb: {:?}, answer: {:?}", i, j, 100 * i + j);
                break 'outer;
            }
        }
    }
    Ok(())
}
