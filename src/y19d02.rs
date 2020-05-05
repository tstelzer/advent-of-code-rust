use std::fs;

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

pub fn part1() {
    let mut memory = create_intcode_memory();
    println!("{}", run_intcode(&mut memory, 12, 2));
}

pub fn part2() {
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

