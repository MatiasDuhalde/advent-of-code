use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn find_floor(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn find_basement_position(input: &str) -> usize {
    let mut floor = 0;
    for i in 0..input.len() {
        match input.chars().nth(i).unwrap() {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return i + 1;
        }
    }
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_input(&args[1]);
    // part one
    let floor = find_floor(&input);
    println!("Floor: {}", floor);
    // part two
    let basement_position = find_basement_position(&input);
    println!("Basement position: {}", basement_position);
}
