#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn calculate_fuel(mass: i32) -> i32 {
    (mass as f32 / 3.0).floor() as i32 - 2
}

/// Recursively calulates fuel cost by mass, i.e.
/// mass of accreting fuel is also taken into account.
fn calculate_fuel_rec(mass: i32) -> i32 {
    let mut result = calculate_fuel(mass);
    let mut next = Some(result);
    while let Some(n) = next {
        let cost = calculate_fuel(n);
        if cost > 0 {
            result += cost;
            next = Some(cost);
        } else {
            next = None;
        }
    }
    result
}

fn y19_d01_p1() -> i32 {
    fs::read_to_string("./static/2019_1_input.txt")
        .expect("Could not read file.")
        .lines()
        .map(|s| s.parse::<i32>().expect("Could not parse to i32."))
        .map(calculate_fuel)
        .sum()
}

fn y19_d01_p2() -> i32 {
    fs::read_to_string("./static/2019_1_input.txt")
        .expect("Could not read file.")
        .lines()
        .map(|s| s.parse::<i32>().expect("Could not parse to i32."))
        .map(calculate_fuel_rec)
        .sum()
}

fn create_intcode_memory() -> Vec<usize> {
    fs::read_to_string("./static/2019_2_input.txt")
        .expect("Could not read file.")
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().expect("Could not parse to usize."))
        .collect::<Vec<usize>>()
}

fn run_intcode(memory: &mut Vec<usize>, noun: usize, verb: usize) -> usize {
    memory[1] = noun;
    memory[2] = verb;

    for x in 0..memory.len() {
        let position = x * 4;
        let operation = memory[position];
        if operation == 99 {
            break;
        }
        let a = memory[memory[position + 1]];
        let b = memory[memory[position + 2]];
        let position_final = memory[position + 3];
        match operation {
            1 => memory[position_final] = a + b,
            2 => memory[position_final] = a * b,
            _ => panic!("Unknown operation."),
        }
    }

    memory[0]
}

#[derive(Debug)]
pub struct Wire {
    points: HashMap<(i32, i32), u32>,
}

impl Wire {
    pub fn shortest_intersection(&self, wire: &Wire) -> u32 {
        self.points
            .keys()
            .collect::<HashSet<_>>()
            .intersection(&wire.points.keys().collect::<HashSet<_>>())
            .fold(0, |acc, key| {
                let x = self.points.get(key).unwrap() + wire.points.get(key).unwrap();
                if acc == 0 || x < acc {
                    x
                } else {
                    acc
                }
            })
    }

    pub fn closest_intersection(&self, wire: &Wire) -> u32 {
        self.points
            .keys()
            .collect::<HashSet<_>>()
            .intersection(&wire.points.keys().collect::<HashSet<_>>())
            .map(|(x, y)| (x.abs() + y.abs()) as u32)
            .min()
            .expect("Couldnt find min intersection point")
    }

    pub fn from_input(s: &str) -> Self {
        #[derive(Debug, Eq, PartialEq, Hash, Clone)]
        struct WirePart {
            vector: (i32, i32),
            length: u32,
        }

        let wireparts = s.trim().split(',').map(|s| {
            let (head, tail) = s.split_at(1);

            let vector = match head {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => panic!("Couldnt parse vector."),
            };

            let length = tail
                .parse::<u32>()
                .expect(&format!("Couldn't parse length, {}", tail));

            WirePart { vector, length }
        });

        let mut points: HashMap<(i32, i32), u32> = HashMap::new();

        let (mut x, mut y, mut step) = (0, 0, 0);
        for wirepart in wireparts {
            let (dx, dy) = wirepart.vector;
            for _ in 0..wirepart.length {
                x += dx;
                y += dy;
                step += 1;

                let point = (x, y);
                points.entry(point).or_insert(step);
            }
        }

        Wire { points }
    }
}

#[cfg(test)]
mod tests {
    use crate::Wire;

    const WIRE_1A: &str = "R8,U5,L5,D3";
    const WIRE_1B: &str = "U7,R6,D4,L4";

    const WIRE_2A: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    const WIRE_2B: &str = "U62,R66,U55,R34,D71,R55,D58,R83";

    const WIRE_3A: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    const WIRE_3B: &str = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_shortest_intersection() {
        assert_eq!(
            Wire::from_input(WIRE_1A).shortest_intersection(&Wire::from_input(WIRE_1B)),
            30
        );

        assert_eq!(
            Wire::from_input(WIRE_2A).shortest_intersection(&Wire::from_input(WIRE_2B)),
            610
        );
        assert_eq!(
            Wire::from_input(WIRE_3A).shortest_intersection(&Wire::from_input(WIRE_3B)),
            410
        );
    }

    #[test]
    fn test_closest_intersection() {
        assert_eq!(
            Wire::from_input(WIRE_1A).closest_intersection(&Wire::from_input(WIRE_1B)),
            6
        );

        assert_eq!(
            Wire::from_input(WIRE_2A).closest_intersection(&Wire::from_input(WIRE_2B)),
            159
        );

        assert_eq!(
            Wire::from_input(WIRE_3A).closest_intersection(&Wire::from_input(WIRE_3B)),
            135
        );
    }
}

fn y19_d03_p1() {
    let contents = fs::read_to_string("./static/2019_3_input.txt").expect("Could not read file");

    let mut wires = contents.trim().split('\n').map(Wire::from_input);

    let wire_1 = wires.next().expect("No wire left");
    let wire_2 = wires.next().expect("No wire left");

    let result = wire_1.closest_intersection(&wire_2);

    println!("{:?}", result);
}

fn y19_d03_p2() {
    let contents = fs::read_to_string("./static/2019_3_input.txt").expect("Could not read file");

    let mut wires = contents.trim().split('\n').map(Wire::from_input);

    let wire_1 = wires.next().expect("No wire left");
    let wire_2 = wires.next().expect("No wire left");

    let result = wire_1.shortest_intersection(&wire_2);

    println!("{:?}", result);
}

fn main() {
    y19_d03_p2();
}
