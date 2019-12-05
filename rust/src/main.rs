mod day1;
mod day2;
mod day3;

use day3::Wire;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line1 = String::new();
    let mut line2 = String::new();
    if stdin.read_line(&mut line1).is_ok() && stdin.read_line(&mut line2).is_ok() {
        let wire1 = Wire::from_directions_string(&line1);
        let wire2 = Wire::from_directions_string(&line2);
        let steps = wire1.total_steps_for_nearest_intersection(&wire2);
        println!("Steps: {}", steps);
    }
    Ok(())
}
