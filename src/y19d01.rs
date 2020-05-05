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

pub fn part1() -> i32 {
    fs::read_to_string("./static/2019_1_input.txt")
        .expect("Could not read file.")
        .lines()
        .map(|s| s.parse::<i32>().expect("Could not parse to i32."))
        .map(calculate_fuel)
        .sum()
}

pub fn part2() -> i32 {
    fs::read_to_string("./static/2019_1_input.txt")
        .expect("Could not read file.")
        .lines()
        .map(|s| s.parse::<i32>().expect("Could not parse to i32."))
        .map(calculate_fuel_rec)
        .sum()
}
