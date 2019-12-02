pub fn fuel_for_modules(module_masses: Vec<u32>) -> u32 {
    module_masses
        .into_iter()
        .map(|mass| required_fuel(mass))
        .sum()
}

fn required_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::{fuel_for_modules, required_fuel};

    #[test]
    fn test_fuel_for_masses() {
        assert_eq!(4, fuel_for_modules(vec![12, 14]));
    }

    #[test]
    fn test_required_fuel() {
        assert_eq!(2, required_fuel(12));
        assert_eq!(2, required_fuel(14));
        assert_eq!(654, required_fuel(1969));
        assert_eq!(33583, required_fuel(100756));
    }
}
