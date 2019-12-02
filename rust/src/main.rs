mod day1;

use day1::fuel_for_modules;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut masses: Vec<u32> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        masses.push(line?.parse().expect("Could not parse number"));
    }
    println!("Fuel requirements: {:?}", fuel_for_modules(masses));
    Ok(())
}
