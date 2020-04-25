#![allow(dead_code)]
use std::fs;

/// Calulates fuel cost by mass.
fn fuel(mass: i32) -> i32 {
    (mass as f32 / 3.0).floor() as i32 - 2
}

/// Recursively calulates fuel cost by mass, i.e.
/// mass of accreting fuel is also taken into account.
fn fuel_rec(mass: i32) -> i32 {
    let mut result = fuel(mass);
    let mut next = Some(result);
    while let Some(n) = next {
        let cost = fuel(n);
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
        .unwrap()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .map(fuel)
        .sum()
}

fn y19_d01_p2() -> i32 {
    fs::read_to_string("./static/2019_1_input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .map(fuel_rec)
        .sum()
}

fn y19_d02_p1() -> usize {
    let mut memory = fs::read_to_string("./static/2019_2_input.txt")
        .expect("Could not read file")
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().expect("Could not parse to usize"))
        .collect::<Vec<usize>>();

    println!("{:?}", memory);

    memory[1] = 12;
    memory[2] = 2;

    for x in 0..memory.len() {
        let position = x * 4;
        let operation = memory[position];
        if operation == 99 {
            break;
        }
        let a = memory[memory[position + 1]];
        let b = memory[memory[position + 2]];
        let position_final = memory[position + 3];
        println!(
            "before {}, {}, {}, {}",
            operation, a, b, memory[position_final]
        );
        match operation {
            1 => memory[position_final] = a + b,
            2 => memory[position_final] = a * b,
            _ => panic!("cenas"),
        }
        println!("after {}", memory[position_final]);
    }

    memory[0]
}

fn main() {
    println!("{}", y19_d02_p1());
}
