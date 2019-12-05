mod day1;
mod day2;
mod day3;

use day3::{calculate_manhattan_distance_for_directions, Direction};
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line1 = String::new();
    let mut line2 = String::new();
    if stdin.read_line(&mut line1).is_ok() && stdin.read_line(&mut line2).is_ok() {
        let distance = calculate_manhattan_distance_for_directions(
            Direction::from_line(line1),
            Direction::from_line(line2),
        );
        println!("Distance: {}", distance);
    }
    Ok(())
}
