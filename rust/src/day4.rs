use std::num::ParseIntError;
use std::str::FromStr;

fn valid_password(password: &str) -> bool {
    password.len() == 6 && has_exactly_double_digits(password) && digits_never_decrease(password)
}

fn has_exactly_double_digits(password: &str) -> bool {
    vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        .iter()
        .any(|digit| password.contains(&digit.repeat(2)) && !password.contains(&digit.repeat(3)))
}

fn digits_never_decrease(password: &str) -> bool {
    password.chars().enumerate().all(|(i, chr)| {
        if i == password.len() - 1 {
            true
        } else {
            let current = chr.to_digit(10).expect("Is not a digit");
            let adjacent = password
                .get(i + 1..i + 2)
                .expect("Out of bounds")
                .parse::<u32>()
                .expect("Could not parse character into number");

            current <= adjacent
        }
    })
}

/// Inclusive range
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordRange {
    begin: u32,
    end: u32,
    current: Option<u32>,
}

impl FromStr for PasswordRange {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        let begin = parts[0].parse::<u32>()?;
        let end = parts[1].parse::<u32>()?;

        Ok(PasswordRange {
            begin,
            end,
            current: None,
        })
    }
}

impl Iterator for PasswordRange {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => self.current = Some(self.begin),
            Some(value) => {
                if value == self.end {
                    self.current = None
                } else {
                    self.current = Some(value + 1)
                }
            }
        }

        self.current
    }
}

impl ExactSizeIterator for PasswordRange {
    fn len(&self) -> usize {
        (self.end - self.begin + 1) as usize
    }
}

impl PasswordRange {
    pub fn valid_passwords(&self) -> Vec<String> {
        self.map(|password_num| password_num.to_string())
            .filter(|password| valid_password(&password))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{valid_password, PasswordRange};

    #[test]
    fn test_password_range_parse() {
        assert_eq!(
            PasswordRange {
                begin: 123456,
                end: 567890,
                current: None,
            },
            "123456-567890".parse().expect("Cannot parse range")
        );
    }

    #[test]
    fn test_valid_password() {
        assert!(!valid_password("111111"));
        assert!(!valid_password("223450"));
        assert!(!valid_password("123789"));
        assert!(valid_password("122345"));
        assert!(!valid_password("111123"));
        assert!(valid_password("112233"));
        assert!(!valid_password("123444"));
        assert!(valid_password("111122"));
    }

    #[test]
    fn test_password_range_valid_password() {
        let valid = "123450-123499"
            .parse::<PasswordRange>()
            .expect("Cannot parse range")
            .valid_passwords();
        assert_eq!(
            vec![
                "123455".to_string(),
                "123466".to_string(),
                "123477".to_string(),
                "123488".to_string(),
                "123499".to_string()
            ]
            .as_slice(),
            valid.as_slice()
        )
    }
}
