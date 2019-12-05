use std::cmp::{Ord, Ordering};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distance = s
            .trim_end()
            .get(1..)
            .expect("Missing a distance from the direction")
            .parse::<i32>()
            .expect("Could not parse distance");
        match &s[0..1] {
            "U" => Ok(Direction::Up(distance)),
            "D" => Ok(Direction::Down(distance)),
            "L" => Ok(Direction::Left(distance)),
            "R" => Ok(Direction::Right(distance)),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn from_line(line: &str) -> Vec<Direction> {
        line.split(",")
            .map(|serialized| serialized.parse().expect("Cannot parse direction"))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }
}

impl Point {
    pub fn manhattan_distance(&self) -> i32 {
        self.x + self.y
    }
}

#[derive(Debug)]
pub struct Wire(Vec<Point>);

impl Wire {
    pub fn from_directions(directions: Vec<Direction>) -> Wire {
        let mut points = vec![];
        let mut position = Point::default();

        for direction in directions {
            let mut line = match direction {
                Direction::Up(distance) => {
                    points.reserve(distance as usize);
                    (0..distance)
                        .map(|_i| {
                            position.x += 1;
                            position.clone()
                        })
                        .collect()
                }
                Direction::Down(distance) => {
                    points.reserve(distance as usize);
                    (0..distance)
                        .map(|_i| {
                            position.x -= 1;
                            position.clone()
                        })
                        .collect()
                }
                Direction::Left(distance) => {
                    points.reserve(distance as usize);
                    (0..distance)
                        .map(|_i| {
                            position.y -= 1;
                            position.clone()
                        })
                        .collect()
                }
                Direction::Right(distance) => {
                    points.reserve(distance as usize);
                    (0..distance)
                        .map(|_i| {
                            position.y += 1;
                            position.clone()
                        })
                        .collect()
                }
            };

            points.append(&mut line);
        }

        Wire(points)
    }

    pub fn from_directions_string(serialized: &str) -> Wire {
        Wire::from_directions(Direction::from_line(serialized))
    }

    fn intersections(&self, other: &Wire) -> HashSet<Point> {
        let self_set: HashSet<Point> = self.0.clone().into_iter().collect();
        let other_set: HashSet<Point> = other.0.clone().into_iter().collect();

        self_set
            .intersection(&other_set)
            .map(|point| *point)
            .collect()
    }

    fn nearest_intersection(&self, other: &Wire) -> Point {
        *self
            .intersections(other)
            .iter()
            .filter(|point| point.x >= 0 && point.y >= 0)
            .min()
            .expect("Should be at least one intersection")
    }

    fn steps(&self, selected: &Point) -> usize {
        1 + self
            .0
            .iter()
            .position(|&point| point == *selected)
            .expect(&format!(
                "Point {:?} should exist on Wire {:?}",
                selected, self
            ))
    }

    pub fn total_steps_for_nearest_intersection(&self, other: &Wire) -> usize {
        let intersection = self
            .intersections(other)
            .into_iter()
            .min_by(|x, y| (self.steps(x) + other.steps(x)).cmp(&(self.steps(y) + other.steps(y))))
            .expect("There should be at least one intersection");

        self.steps(&intersection) + other.steps(&intersection)
    }
}

pub fn manhattan_distance_for_directions_string(
    wire1_directions: &str,
    wire2_directions: &str,
) -> i32 {
    let wire1 = Wire::from_directions_string(wire1_directions);
    let wire2 = Wire::from_directions_string(wire2_directions);

    wire1.nearest_intersection(&wire2).manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::{manhattan_distance_for_directions_string, Direction, Point, Wire};

    #[test]
    fn test_direction_from_line() {
        assert_eq!(
            vec![Right(8), Up(5), Left(5), Down(3)].as_slice(),
            Direction::from_line("R8,U5,L5,D3\n").as_slice(),
        );
        assert_eq!(
            vec![Up(7), Right(6), Down(4), Left(4)].as_slice(),
            Direction::from_line("U7,R6,D4,L4\n").as_slice(),
        );
    }

    #[test]
    fn test_manhattan_distance() {
        let wire1 = Wire::from_directions(vec![Right(8), Up(5), Left(5), Down(3)]);
        let wire2 = Wire::from_directions(vec![Up(7), Right(6), Down(4), Left(4)]);
        let intersections = wire1.intersections(&wire2);
        assert_eq!(2, intersections.len());
        assert_eq!(6, Point { x: 3, y: 3 }.manhattan_distance());
    }

    #[test]
    fn test_manhattan_distance_for_directions() {
        assert_eq!(
            6,
            manhattan_distance_for_directions_string("R8,U5,L5,D3\n", "U7,R6,D4,L4\n")
        );
        assert_eq!(
            159,
            manhattan_distance_for_directions_string(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            135,
            manhattan_distance_for_directions_string(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn test_total_steps_for_nearest_intersection() {
        let wire1 = Wire::from_directions(vec![Right(8), Up(5), Left(5), Down(3)]);
        let wire2 = Wire::from_directions(vec![Up(7), Right(6), Down(4), Left(4)]);

        assert_eq!(30, wire1.total_steps_for_nearest_intersection(&wire2));

        assert_eq!(
            610,
            Wire::from_directions_string("R75,D30,R83,U83,L12,D49,R71,U7,L72")
                .total_steps_for_nearest_intersection(&Wire::from_directions_string(
                    "U62,R66,U55,R34,D71,R55,D58,R83"
                ))
        );

        assert_eq!(
            410,
            Wire::from_directions_string("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
                .total_steps_for_nearest_intersection(&Wire::from_directions_string(
                    "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                ))
        );
    }
}
