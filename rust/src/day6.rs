use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Object {
    name: String,
    orbiting: Option<String>,
}

impl Object {
    pub fn new_with_orbiting(name: &str, orbiting: &str) -> Object {
        Object {
            name: name.to_string(),
            orbiting: Some(orbiting.to_string()),
        }
    }

    pub fn set_orbiting(&mut self, orbiting: &str) {
        self.orbiting = Some(orbiting.to_string())
    }

    pub fn orbits(&self, map: &OrbitMap) -> u32 {
        let mut count = 0;
        let mut parent = self.orbiting.clone();
        while let Some(parent_name) = parent {
            count += 1;
            parent = match map.get(&parent_name) {
                Some(parent_object) => parent_object.borrow().orbiting.clone(),
                None => None,
            };
        }

        count
    }
}

type OrbitMap = HashMap<String, RefCell<Object>>;

pub fn orbit_map_from_string(mapping: &str) -> OrbitMap {
    let mut map: OrbitMap = HashMap::new();
    for definition_str in mapping.split("\n") {
        let definition: Vec<&str> = definition_str.split(")").collect();
        let default_orbiting = Object {
            name: definition[0].to_string(),
            orbiting: None,
        };
        let orbiting = map
            .entry(definition[0].to_string())
            .or_insert(RefCell::new(default_orbiting))
            .borrow()
            .name
            .clone();
        let default_orbiter = Object::new_with_orbiting(definition[1], &orbiting);
        let mut orbiter = map
            .entry(definition[1].to_string())
            .or_insert(RefCell::new(default_orbiter))
            .borrow_mut();
        if orbiter.orbiting.is_none() {
            orbiter.set_orbiting(&orbiting);
        }
    }

    map
}

pub fn orbit_count_checksum(mapping: &str) -> u32 {
    let map = orbit_map_from_string(mapping);
    map.values()
        .map(|object| object.borrow().orbits(&map))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{orbit_count_checksum, orbit_map_from_string};

    #[test]
    fn test_orbit_map_from_string() {
        let mapping = "A)B";
        let map = orbit_map_from_string(mapping);
        let mut names_vec: Vec<String> = map.keys().map(|k| k.to_string()).collect();
        let names: &mut [String] = names_vec.as_mut_slice();
        names.sort();
        let expected: Vec<&str> = vec!["A", "B"];
        assert_eq!(expected.as_slice(), names);
    }

    #[test]
    fn test_orbit_count_checksum() {
        println!("test orbit_count_checksum");
        let mapping = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        println!("Mapping: {}", mapping);
        assert_eq!(42, orbit_count_checksum(mapping));
    }
}
