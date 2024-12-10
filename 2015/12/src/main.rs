use regex::Regex;
use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().trim().to_string()
}

fn add_numbers(json: &str) -> i64 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(json) {
        sum += cap[1].parse::<i64>().unwrap();
    }
    sum
}

fn find_opening_brace(chars: &Vec<char>, start_index: usize) -> usize {
    let mut i = start_index as i32;
    let mut braces_count = 0;
    while i >= 0 {
        if chars[i as usize] == '}' {
            braces_count -= 1;
        } else if chars[i as usize] == '{' {
            if braces_count == 0 {
                break;
            }
            braces_count += 1;
        }
        i -= 1;
    }
    i as usize
}

fn find_closing_brace(chars: &Vec<char>, start_index: usize) -> usize {
    let mut i = start_index;
    let mut braces_count = 0;
    while i < chars.len() {
        if chars[i] == '{' {
            braces_count += 1;
        } else if chars[i] == '}' {
            if braces_count == 0 {
                break;
            }
            braces_count -= 1;
        }
        i += 1;
    }
    i
}

fn add_non_red_numbers(json: &str) -> i64 {
    let chars: Vec<char> = json.chars().collect();
    let red_re = Regex::new(":\"red\"").unwrap();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for m in red_re.find_iter(json) {
        let start = find_opening_brace(&chars, m.start() - 1);
        let end = find_closing_brace(&chars, m.end());
        ranges.push((start, end));
    }
    ranges.sort_by_key(|x| x.0);
    if ranges.is_empty() {
        return add_numbers(json);
    }

    let mut curr_start = ranges[0].0;
    let mut curr_end = ranges[0].1;
    let mut non_overlapping_ranges: Vec<(usize, usize)> = Vec::new();
    for i in 1..ranges.len() {
        if ranges[i].0 > curr_end {
            non_overlapping_ranges.push((curr_start, curr_end));
            curr_start = ranges[i].0;
            curr_end = ranges[i].1;
        }
    }
    non_overlapping_ranges.push((curr_start, curr_end));

    let e = non_overlapping_ranges[0].0;
    let mut other_sum = add_numbers(&json[0..e]);
    for i in 1..non_overlapping_ranges.len() {
        let s = non_overlapping_ranges[i - 1].1;
        let e = non_overlapping_ranges[i].0;
        other_sum += add_numbers(&json[s..=e]);
    }
    let s = non_overlapping_ranges[non_overlapping_ranges.len() - 1].1;
    other_sum += add_numbers(&json[s..]);

    other_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let json = read_input(filename);
    // part one
    let sum = add_numbers(&json);
    println!("Sum of all numbers in JSON: {}", sum);
    // part two
    let non_red_sum = add_non_red_numbers(&json);
    println!("Sum of non-red numbers in JSON: {}", non_red_sum);
}
