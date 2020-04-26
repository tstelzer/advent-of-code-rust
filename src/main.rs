#![allow(dead_code)]
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

fn y19_d02_p1() {
    let mut memory = create_intcode_memory();
    println!("{}", run_intcode(&mut memory, 12, 2));
}

fn y19_d02_p2() {
    let memory = create_intcode_memory();
    let expected = 19_690_720;
    for noun in 1..100 {
        for verb in 1..100 {
            if run_intcode(&mut memory.clone(), noun, verb) == expected {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}

fn main() {
    y19_d02_p2();
}
