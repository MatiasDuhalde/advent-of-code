use std::collections::HashSet;
use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn count_houses(directions: &str) -> i32 {
    let mut visited_houses = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited_houses.insert((x, y));
    for c in directions.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Invalid direction"),
        }
        visited_houses.insert((x, y));
    }
    visited_houses.len() as i32
}

fn count_robot_houses(directions: &str) -> i32 {
    let mut visited_houses = HashSet::new();
    let mut x_santa = 0;
    let mut y_santa = 0;
    let mut x_robot = 0;
    let mut y_robot = 0;
    visited_houses.insert((0, 0));
    let mut santa = true;
    for c in directions.chars() {
        let x = if santa { &mut x_santa } else { &mut x_robot };
        let y = if santa { &mut y_santa } else { &mut y_robot };
        match c {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => panic!("Invalid direction"),
        };
        visited_houses.insert((*x, *y));
        santa = !santa;
    }
    visited_houses.len() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let directions = read_input(filename);
    // part one
    let houses = count_houses(&directions);
    println!("Houses: {}", houses);
    // part two
    let houses = count_robot_houses(&directions);
    println!("Houses: {}", houses);
}
