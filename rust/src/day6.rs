use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

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

    pub fn orbit_list(&self, map: &OrbitMap) -> Vec<String> {
        let mut orbits = vec![];
        let mut parent = self.orbiting.clone();
        while let Some(parent_name) = parent {
            orbits.push(parent_name.clone());
            parent = match map.get(&parent_name) {
                Some(parent_object) => parent_object.borrow().orbiting.clone(),
                None => None,
            };
        }

        orbits
    }

    fn orbit_distance(&self, other: &Object, map: &OrbitMap) -> usize {
        let intersection = self.orbit_intersection(other, map);
        let orbit_list = self.orbit_list(map);
        let other_orbit_list = other.orbit_list(map);

        2 + orbit_list
            .iter()
            .position(|item| *item == intersection)
            .unwrap()
            + other_orbit_list
                .iter()
                .position(|item| *item == intersection)
                .unwrap()
    }

    pub fn orbit_intersection(&self, other: &Object, map: &OrbitMap) -> String {
        let orbit_list = self.orbit_list(map);
        let orbit_set: HashSet<&str> = orbit_list.iter().map(|item| item.as_str()).collect();
        let other_orbit_list = other.orbit_list(map);
        let other_orbit_set: HashSet<&str> =
            other_orbit_list.iter().map(|item| item.as_str()).collect();
        let intersection = orbit_set.intersection(&other_orbit_set);
        intersection
            .min_by(|x, y| {
                let x_position = orbit_list
                    .iter()
                    .position(|item| item.as_str() == **x)
                    .expect(format!("Cannot find {}", x).as_str())
                    + other_orbit_list
                        .iter()
                        .position(|item| item.as_str() == **x)
                        .expect(format!("Cannot find {}", x).as_str());
                let y_position = orbit_list
                    .iter()
                    .position(|item| item.as_str() == **y)
                    .expect(format!("Cannot find {}", y).as_str())
                    + other_orbit_list
                        .iter()
                        .position(|item| item.as_str() == **y)
                        .expect(format!("Cannot find {}", y).as_str());

                x_position.cmp(&y_position)
            })
            .expect("List empty")
            .to_string()
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

pub fn orbital_transfers(mapping: &str, object_name_1: &str, object_name_2: &str) -> usize {
    let map = orbit_map_from_string(mapping);
    let object_1 = map
        .get(object_name_1)
        .expect(format!("Cannot find {}", object_name_1).as_str())
        .borrow();
    let object_2 = map
        .get(object_name_2)
        .expect(format!("Cannot find {}", object_name_2).as_str())
        .borrow();

    object_1.orbit_distance(&object_2, &map) - 2
}

#[cfg(test)]
mod tests {
    use super::{orbit_count_checksum, orbit_map_from_string, orbital_transfers};

    const MAPPING: &'static str = "COM)B
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
    const SANTA_MAPPING: &'static str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

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
        assert_eq!(42, orbit_count_checksum(MAPPING));
    }

    #[test]
    fn test_object_orbit_list() {
        let map = orbit_map_from_string(MAPPING);
        let object = map.get("H").expect("Cannot find H").borrow();
        assert_eq!(
            vec!["G", "B", "COM"].as_slice(),
            object.orbit_list(&map).as_slice()
        );
    }

    #[test]
    fn test_object_orbit_intersection() {
        let map = orbit_map_from_string(SANTA_MAPPING);
        let santa = map.get("SAN").expect("Cannot find SAN").borrow();
        let you = map.get("YOU").expect("Cannot find YOU").borrow();
        assert_eq!("D".to_string(), you.orbit_intersection(&santa, &map));
    }

    #[test]
    fn test_object_orbit_distance() {
        let map = orbit_map_from_string(SANTA_MAPPING);
        let santa = map.get("SAN").expect("Cannot find SAN").borrow();
        let you = map.get("YOU").expect("Cannot find YOU").borrow();
        assert_eq!(6, you.orbit_distance(&santa, &map));
    }

    #[test]
    fn test_orbital_transfers() {
        assert_eq!(4, orbital_transfers(SANTA_MAPPING, "SAN", "YOU"));
    }
}
