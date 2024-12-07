use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(rule: &Vec<i32>) -> bool {
    let mut asc = 0;
    for i in 1..rule.len() {
        let prev = rule[i - 1];
        let curr = rule[i];
        if (prev - curr).abs() > 3 || prev == curr {
            return false;
        }
        if asc == 0 {
            asc = if prev < curr { 1 } else { -1 };
        } else if (asc == 1 && prev > curr) || (asc == -1 && prev < curr) {
            return false;
        }
    }
    true
}

fn count_safe_rules(rules: &Vec<Vec<i32>>) -> i32 {
    let mut safe_rules = 0;
    for rule in rules {
        if is_safe(rule) {
            safe_rules += 1;
        }
    }
    safe_rules
}

fn count_dampener_safe_rules(rules: &Vec<Vec<i32>>) -> i32 {
    let mut safe_rules = 0;
    for rule in rules {
        if is_safe(rule) {
            safe_rules += 1;
        } else {
            for i in 0..rule.len() {
                let mut rule_copy = rule.clone();
                rule_copy.remove(i);
                if is_safe(&rule_copy) {
                    safe_rules += 1;
                    break;
                }
            }
        }
    }
    safe_rules
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let rules = read_input(filename);
    // part one
    let safe_rules = count_safe_rules(&rules);
    println!("Safe rules: {}", safe_rules);
    // part two
    let dampener_safe_rules = count_dampener_safe_rules(&rules);
    println!("Dampener safe rules: {}", dampener_safe_rules);
}
