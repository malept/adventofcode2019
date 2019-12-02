mod day1;
mod day2;

use day2::execute_intcode;
use std::io;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let mut instructions: Vec<u32> = vec![];
    let stdin = io::stdin();
    for item in stdin.lock().split(b',') {
        let bytes = item?;
        let serialized = str::from_utf8(&bytes)
            .expect("Could not create string")
            .replace("\n", "");
        instructions.push(serialized.parse().expect("Could not parse number"));
    }
    execute_intcode(&mut instructions);
    let stringified: Vec<String> = instructions
        .into_iter()
        .map(|item| item.to_string())
        .collect();
    println!("Results: {}", stringified.join(","));
    Ok(())
}
