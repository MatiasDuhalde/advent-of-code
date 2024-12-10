use regex::Regex;
use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn look_and_say(input: &str) -> String {
    let re = Regex::new(r"0+|1+|2+|3+|4+|5+|6+|7+|8+|9+").unwrap();
    re.find_iter(input)
        .map(|m| m.len().to_string() + &m.as_str().chars().next().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn iterative_look_and_say(input: &str, times: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..times {
        result = look_and_say(&result);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_input(filename);
    // part one
    let result = iterative_look_and_say(&input, 40);
    println!("Part One: {}", result.len());
    // part two
    let result = iterative_look_and_say(&input, 50);
    println!("Part Two: {}", result.len());
}
