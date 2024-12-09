use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn read_file(filename: &str) -> Vec<(String, String)> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            (a.to_string(), b.to_string())
        })
        .collect()
}

fn get_numeric_value(signal: &str, signal_values: &HashMap<&str, u16>) -> Option<u16> {
    signal
        .parse::<u16>()
        .ok()
        .or_else(|| signal_values.get(signal).copied())
}

fn find_signal(instructions: &Vec<(String, String)>, signal: &str) -> u16 {
    let mut signal_values: HashMap<&str, u16> = HashMap::new();
    let re =
        Regex::new(r"(?P<plain>^(\d+|[a-zA-Z]+)$)|(?P<unary>^(?P<op>NOT) (?P<v>\d+|[a-zA-Z]+)$)|(?P<binary>^(?P<a>\d+|[a-zA-Z]+) (?P<bin_op>(AND|OR|LSHIFT|RSHIFT)) (?P<b>\d+|[a-zA-Z]+)$)")
            .unwrap();
    let mut instructions_queue = VecDeque::from_iter(instructions.iter());
    while !instructions_queue.is_empty() {
        let instruction = instructions_queue.pop_front().unwrap();
        let (operation, output) = instruction;
        let captures = re.captures(operation).unwrap();
        captures
            .name("plain")
            .and_then(|v| get_numeric_value(v.as_str(), &signal_values))
            .or_else(|| {
                captures.name("unary").and_then(|_| {
                    let operation = captures.name("op").unwrap().as_str();
                    let string_value = captures.name("v").unwrap().as_str();
                    get_numeric_value(string_value, &signal_values).and_then(|v| match operation {
                        "NOT" => Some(!v),
                        _ => None,
                    })
                })
            })
            .or_else(|| {
                captures.name("binary").and_then(|_| {
                    let operation = captures.name("bin_op").unwrap().as_str();
                    let a = get_numeric_value(captures.name("a").unwrap().as_str(), &signal_values);
                    if a.is_none() {
                        return None;
                    }
                    let b = get_numeric_value(captures.name("b").unwrap().as_str(), &signal_values);
                    if b.is_none() {
                        return None;
                    }
                    match operation {
                        "AND" => Some(a.unwrap() & b.unwrap()),
                        "OR" => Some(a.unwrap() | b.unwrap()),
                        "LSHIFT" => Some(a.unwrap() << b.unwrap()),
                        "RSHIFT" => Some(a.unwrap() >> b.unwrap()),
                        _ => None,
                    }
                })
            })
            .map_or_else(
                || instructions_queue.push_back(instruction),
                |v| {
                    signal_values.insert(output.as_str(), v);
                },
            );
    }
    *signal_values.get(&signal).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let instructions = read_file(filename);
    // part one
    let signal_a_value = find_signal(&instructions, "a");
    println!("Signal a value: {}", signal_a_value);
    // part two
    let mut new_instructions = instructions.clone();
    let mut i = 0;
    while i < new_instructions.len() {
        let (_, output) = &new_instructions[i];
        if output.eq("b") {
            break;
        }
        i += 1;
    }
    new_instructions[i] = (signal_a_value.to_string(), "b".to_string());
    let signal_a_new_value = find_signal(&new_instructions, "a");
    println!("Signal a new value: {}", signal_a_new_value);
}
