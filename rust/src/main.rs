mod day1;
mod day3;
mod day4;
mod intcode;

use intcode::{memory_from_string, Computer};
use std::io;

fn main() -> io::Result<()> {
    let mut computer = Computer::new(memory_from_string(io::stdin().lock())?);
    println!(
        "Diagnostic Code: {:?}",
        computer.execute(Some(1)).expect("Expected output")
    );
    Ok(())
}
