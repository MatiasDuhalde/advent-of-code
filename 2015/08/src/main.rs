use regex::{Captures, Regex};
use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn find_unescape_difference(s: &str) -> i32 {
    let total_characters = s.len() as i32;
    let unescaped = unescape_string(s);
    let unescaped_characters = unescaped.chars().count() as i32;
    total_characters - unescaped_characters
}

fn find_total_unescape_difference(strings: &Vec<String>) -> i32 {
    strings.iter().map(|s| find_unescape_difference(s)).sum()
}

fn unescape_string(s: &str) -> String {
    let mut result = s.to_string();
    result = result[1..result.len() - 1].to_string();
    let re = Regex::new(r"\\x(?P<v>[0-9a-f]{2})|\\(?P<c>.)").unwrap();
    result = re
        .replace_all(&result, |caps: &Captures| {
            if caps.name("v").is_some() {
                let v = u8::from_str_radix(&caps["v"], 16).unwrap();
                return format!("{}", v as char);
            }
            if caps.name("c").is_some() {
                return caps["c"].to_string();
            }
            "".to_string()
        })
        .to_string();

    result
}

fn find_escape_difference(s: &str) -> i32 {
    let total_characters = s.len() as i32;
    let escaped = escape_string(s);
    let escaped_characters = escaped.chars().count() as i32;
    escaped_characters - total_characters
}

fn find_total_escape_difference(strings: &Vec<String>) -> i32 {
    strings.iter().map(|s| find_escape_difference(s)).sum()
}

fn escape_string(s: &str) -> String {
    let mut result = s.to_string();
    result = result.replace("\\", "\\\\");
    result = result.replace("\"", "\\\"");
    result = format!("\"{}\"", result);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let strings = read_input(filename);
    // part one
    let unescape_difference = find_total_unescape_difference(&strings);
    println!("Unescape difference: {}", unescape_difference);
    // part two
    let escape_difference = find_total_escape_difference(&strings);
    println!("Escape difference: {}", escape_difference);
}
