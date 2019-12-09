mod day1;
mod day3;
mod day4;
mod day6;
mod intcode;

use day6::orbital_transfers;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    println!(
        "Transfers: {}",
        orbital_transfers(buffer.as_str().trim(), "SAN", "YOU")
    );
    Ok(())
}
