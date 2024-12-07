use std::collections::HashMap;
use std::env;
use std::fs;

fn read_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(filename).unwrap();

    let lines = contents.lines();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in lines {
        let mut numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        list1.push(numbers.next().unwrap());
        list2.push(numbers.next().unwrap());
    }
    (list1, list2)
}

fn find_total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut cloned_list1 = list1.clone();
    let mut cloned_list2 = list2.clone();
    cloned_list1.sort();
    cloned_list2.sort();
    let mut total_distance = 0;
    for i in 0..cloned_list1.len() {
        total_distance += (cloned_list1[i] - cloned_list2[i]).abs();
    }
    total_distance
}

fn find_similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut scores: HashMap<i32, i32> = HashMap::new();
    for n in list2 {
        let count = scores.entry(*n).or_insert(0);
        *count += n;
    }
    let mut similarity_score = 0;
    for n in list1 {
        similarity_score += scores.get(n).unwrap_or(&0);
    }
    similarity_score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (list1, list2) = read_input(filename);
    // part one
    let total_distance = find_total_distance(&list1, &list2);
    println!("Total distance: {}", total_distance);
    // part two
    let similarity_score = find_similarity_score(&list1, &list2);
    println!("Similarity score: {}", similarity_score);
}
