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
    pub fn from_line(line: String) -> Vec<Direction> {
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
        calculate_manhattan_distance(self).cmp(&calculate_manhattan_distance(other))
    }
}

struct Wire(Vec<Point>);

fn calculate_wire(directions: Vec<Direction>) -> Wire {
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

fn find_wire_intersections(wire1: Wire, wire2: Wire) -> HashSet<Point> {
    let points1: HashSet<Point> = wire1.0.into_iter().collect();
    let points2: HashSet<Point> = wire2.0.into_iter().collect();

    points1
        .intersection(&points2)
        .map(|point| point.clone())
        .collect()
}

fn calculate_manhattan_distance(point: &Point) -> i32 {
    point.x + point.y
}

pub fn calculate_manhattan_distance_for_directions(
    wire1_directions: Vec<Direction>,
    wire2_directions: Vec<Direction>,
) -> i32 {
    let wire1 = calculate_wire(wire1_directions);
    let wire2 = calculate_wire(wire2_directions);
    let intersections = find_wire_intersections(wire1, wire2);

    calculate_manhattan_distance(
        intersections
            .iter()
            .filter(|point| point.x >= 0 && point.y >= 0)
            .min()
            .expect("Should be at least one intersection"),
    )
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::{
        calculate_manhattan_distance, calculate_manhattan_distance_for_directions, calculate_wire,
        find_wire_intersections, Direction,
    };

    #[test]
    fn test_direction_from_line() {
        assert_eq!(
            vec![Right(8), Up(5), Left(5), Down(3)].as_slice(),
            Direction::from_line(String::from("R8,U5,L5,D3\n")).as_slice(),
        );
        assert_eq!(
            vec![Up(7), Right(6), Down(4), Left(4)].as_slice(),
            Direction::from_line(String::from("U7,R6,D4,L4\n")).as_slice(),
        );
    }

    #[test]
    fn test_calculate_manhattan_distance() {
        let wire1 = calculate_wire(vec![Right(8), Up(5), Left(5), Down(3)]);
        let wire2 = calculate_wire(vec![Up(7), Right(6), Down(4), Left(4)]);
        let intersections = find_wire_intersections(wire1, wire2);
        assert_eq!(2, intersections.len());
        assert_eq!(
            6,
            calculate_manhattan_distance(
                intersections
                    .iter()
                    .min()
                    .expect("Should be a minimum intersection")
            )
        );
    }

    #[test]
    fn test_calculate_manhattan_distance_for_directions() {
        assert_eq!(
            6,
            calculate_manhattan_distance_for_directions(
                vec![Right(8), Up(5), Left(5), Down(3)],
                vec![Up(7), Right(6), Down(4), Left(4)]
            )
        );
        assert_eq!(
            159,
            calculate_manhattan_distance_for_directions(
                vec![
                    Right(75),
                    Down(30),
                    Right(83),
                    Up(83),
                    Left(12),
                    Down(49),
                    Right(71),
                    Up(7),
                    Left(72)
                ],
                vec![
                    Up(62),
                    Right(66),
                    Up(55),
                    Right(34),
                    Down(71),
                    Right(55),
                    Down(58),
                    Right(83)
                ]
            )
        );
        assert_eq!(
            135,
            calculate_manhattan_distance_for_directions(
                vec![
                    Right(98),
                    Up(47),
                    Right(26),
                    Down(63),
                    Right(33),
                    Up(87),
                    Left(62),
                    Down(20),
                    Right(33),
                    Up(53),
                    Right(51)
                ],
                vec![
                    Up(98),
                    Right(91),
                    Down(20),
                    Right(16),
                    Down(67),
                    Right(40),
                    Up(7),
                    Right(15),
                    Up(6),
                    Right(7)
                ]
            )
        );
    }
}
