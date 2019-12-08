mod day1;
mod day3;
mod day4;
mod intcode;

use intcode::{memory_from_io, Computer};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("USAGE: {} [INPUT]", args[0]);
    }
    let input = args[1].parse().ok();
    let mut computer = Computer::new(memory_from_io(io::stdin().lock())?);
    println!(
        "Diagnostic Code: {:?}",
        computer.execute(input).expect("Expected output")
    );
    Ok(())
}
