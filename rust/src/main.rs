mod day1;
mod day3;
mod day4;
mod intcode;

use day4::PasswordRange;
use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    let stdin = io::stdin();
    if stdin.read_line(&mut line).is_ok() {
        let passwords = line
            .trim_end()
            .parse::<PasswordRange>()
            .expect("Cannot parse password range")
            .valid_passwords();
        println!("Number of valid passwords: {}", passwords.len());
    }
    Ok(())
}
