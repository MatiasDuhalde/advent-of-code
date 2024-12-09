use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle,
}

type Instruction = (Operation, (usize, usize), (usize, usize));

fn read_file(filename: &str) -> Vec<Instruction> {
    let re =
        Regex::new(r"(?P<op>turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let operation = match &cap["op"] {
                "turn on" => Operation::On,
                "turn off" => Operation::Off,
                "toggle" => Operation::Toggle,
                _ => panic!("Invalid operation"),
            };
            (
                operation,
                (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
            )
        })
        .collect()
}

fn count_lights_on(instructions: &Vec<Instruction>) -> usize {
    let mut lights_on: HashSet<(usize, usize)> = HashSet::new();
    for instruction in instructions {
        let operation = &instruction.0;
        let start = &instruction.1;
        let end = &instruction.2;
        match operation {
            Operation::On => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights_on.insert((i, j));
                    }
                }
            }
            Operation::Off => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights_on.remove(&(i, j));
                    }
                }
            }
            Operation::Toggle => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        if lights_on.contains(&(i, j)) {
                            lights_on.remove(&(i, j));
                        } else {
                            lights_on.insert((i, j));
                        }
                    }
                }
            }
        }
    }
    lights_on.len()
}

fn get_total_brightness(instructions: &Vec<Instruction>) -> usize {
    let mut brightness: HashMap<(usize, usize), usize> = HashMap::new();
    for instruction in instructions {
        let operation = &instruction.0;
        let start = &instruction.1;
        let end = &instruction.2;
        match operation {
            Operation::On => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        brightness
                            .entry((i, j))
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }
            }
            Operation::Off => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        brightness
                            .entry((i, j))
                            .and_modify(|e| {
                                if *e > 0 {
                                    *e -= 1
                                }
                            })
                            .or_insert(0);
                    }
                }
            }
            Operation::Toggle => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        brightness
                            .entry((i, j))
                            .and_modify(|e| *e += 2)
                            .or_insert(2);
                    }
                }
            }
        }
    }
    brightness.values().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let instructions = read_file(filename);
    // part one
    let lights_on = count_lights_on(&instructions);
    println!("Lights on: {}", lights_on);
    // part two
    let brightness = get_total_brightness(&instructions);
    println!("Brightness: {}", brightness);
}
