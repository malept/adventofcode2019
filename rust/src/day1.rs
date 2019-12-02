pub fn fuel_for_modules(module_masses: Vec<u32>) -> u32 {
    module_masses
        .into_iter()
        .map(|mass| {
            let mut total = 0;
            let mut fuel = required_fuel(mass);
            while fuel > 0 {
                total += fuel;
                fuel = required_fuel(fuel);
            }

            total
        })
        .sum()
}

fn required_fuel(mass: u32) -> u32 {
    let quotient = mass / 3;
    if quotient > 1 {
        quotient - 2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{fuel_for_modules, required_fuel};

    #[test]
    fn test_fuel_for_modules() {
        assert_eq!(2, fuel_for_modules(vec![14]));
        assert_eq!(966, fuel_for_modules(vec![1969]));
        assert_eq!(50346, fuel_for_modules(vec![100756]));
    }

    #[test]
    fn test_required_fuel() {
        assert_eq!(2, required_fuel(12));
        assert_eq!(2, required_fuel(14));
        assert_eq!(654, required_fuel(1969));
        assert_eq!(33583, required_fuel(100756));
    }
}
