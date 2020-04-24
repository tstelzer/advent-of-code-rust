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

fn main() {
    println!("{:?}", y19_d01_p2());
}
