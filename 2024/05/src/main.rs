use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn read_input(filename: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(filename).unwrap();
    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    let mut reading_rules = true;
    for line in contents.lines() {
        if line == "" {
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let values = line
                .split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let key = values[0];
            let value = values[1];
            let entry = rules.entry(key).or_insert(HashSet::new());
            entry.insert(value);
        } else {
            let values = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            updates.push(values);
        }
    }
    (rules, updates)
}

fn is_correct(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    for i in 1..update.len() {
        if (rules.get(&update[i]).and_then(|set| {
            for value_before in update[..i].iter() {
                if set.contains(value_before) {
                    return Some(false);
                }
            }
            None
        }))
        .is_some()
        {
            return false;
        }
    }
    true
}

fn add_middle_pages_of_correct(rules: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for update in updates.iter() {
        if is_correct(rules, update) {
            sum += update[update.len() / 2]
        }
    }
    sum
}

fn order_if_incorrect(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> Option<Vec<i32>> {
    let mut ordered = false;
    let mut ordered_update = update.clone();
    for i in 1..ordered_update.len() {
        let rule = rules.get(&ordered_update[i]);
        if rule.is_some() {
            let set = rule.unwrap();
            for j in 0..i {
                if set.contains(&ordered_update[j]) {
                    ordered_update = ordered_update.clone();
                    ordered_update.swap(i, j);
                    ordered = true;
                }
            }
        }
    }
    if ordered {
        Some(ordered_update)
    } else {
        None
    }
}

fn sum_ordered_middle_pages(rules: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for update in updates.iter() {
        sum += order_if_incorrect(rules, update)
            .and_then(|ordered_update| Some(ordered_update[ordered_update.len() / 2]))
            .unwrap_or(0);
    }
    sum
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = &args[1];
    let (rules, updates) = read_input(filename);
    // part one
    let middle_pages_sum = add_middle_pages_of_correct(&rules, &updates);
    println!("Sum of middle pages: {}", middle_pages_sum);
    //part two
    let ordered_middle_pages_sum = sum_ordered_middle_pages(&rules, &updates);
    println!("Sum of ordered middle pages: {}", ordered_middle_pages_sum);
}
