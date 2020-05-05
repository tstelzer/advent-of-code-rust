use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

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
    use super::Wire;

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

pub fn part1() {
    let contents = fs::read_to_string("./static/2019_3_input.txt").expect("Could not read file");

    let mut wires = contents.trim().split('\n').map(Wire::from_input);

    let wire_1 = wires.next().expect("No wire left");
    let wire_2 = wires.next().expect("No wire left");

    let result = wire_1.closest_intersection(&wire_2);

    println!("{:?}", result);
}

pub fn part2() {
    let contents = fs::read_to_string("./static/2019_3_input.txt").expect("Could not read file");

    let mut wires = contents.trim().split('\n').map(Wire::from_input);

    let wire_1 = wires.next().expect("No wire left");
    let wire_2 = wires.next().expect("No wire left");

    let result = wire_1.shortest_intersection(&wire_2);

    println!("{:?}", result);
}
