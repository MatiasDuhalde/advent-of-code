use regex::Regex;
use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn add_multiplications(input: &str) -> i32 {
    // match strings like "mul(a,b)" and parse a and b
    let re = Regex::new(r"mul\((?P<a>\d+),(?P<b>\d+)\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let a = cap["a"].parse::<i32>().unwrap();
        let b = cap["b"].parse::<i32>().unwrap();
        sum += a * b;
    }
    sum
}

fn add_multiplications_with_flags(input: &str) -> i32 {
    // match strings like "mul(a,b)" and parse a and b
    let re =
        Regex::new(r"(?P<mul>mul\((?P<a>\d+),(?P<b>\d+)\))|(?P<do>do\(\))|(?P<dont>don\'t\(\))")
            .unwrap();
    let mut sum = 0;
    let mut enable = true;
    for cap in re.captures_iter(input) {
        if cap.name("do").is_some() {
            enable = true;
        } else if cap.name("dont").is_some() {
            enable = false;
        } else if cap.name("mul").is_some() && enable {
            let a = cap["a"].parse::<i32>().unwrap();
            let b = cap["b"].parse::<i32>().unwrap();
            sum += a * b;
        }
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_input(filename);
    // part one
    let sum = add_multiplications(&input);
    println!("Sum: {}", sum);
    // part two
    let sum_with_flags = add_multiplications_with_flags(&input);
    println!("Sum with flags: {}", sum_with_flags);
}
