use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::i32;
use std::mem::swap;

fn read_input(filename: &str) -> Vec<(String, String, i32)> {
    let re = Regex::new(r"^(?P<a>\w+) to (?P<b>\w+) = (?P<d>\d+)$").unwrap();
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            (
                caps["a"].to_string(),
                caps["b"].to_string(),
                caps["d"].parse().unwrap(),
            )
        })
        .collect()
}

fn parse_matrix(input: &Vec<(String, String, i32)>) -> Vec<Vec<i32>> {
    let mut cities_distances: HashMap<(String, String), i32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();
    for (a, b, d) in input {
        cities.insert(a.to_string());
        cities.insert(b.to_string());
        cities_distances.insert((a.to_string(), b.to_string()), *d);
        cities_distances.insert((b.to_string(), a.to_string()), *d);
    }
    let cities_vec: Vec<String> = cities.into_iter().collect();
    let mut matrix = vec![vec![-1; cities_vec.len()]; cities_vec.len()];
    for i in 0..cities_vec.len() {
        for j in 0..cities_vec.len() {
            matrix[i][j] = if i == j {
                -1
            } else {
                *cities_distances
                    .get(&(cities_vec[i].to_string(), cities_vec[j].to_string()))
                    .unwrap()
            }
        }
    }
    matrix
}

fn visit(matrix: &Vec<Vec<i32>>, current_index: usize, indexes_to_visit: &Vec<usize>) -> i32 {
    if indexes_to_visit.len() == 1 {
        return matrix[current_index][indexes_to_visit[0]];
    }
    let mut min_acc_distance = i32::MAX;
    let mut next_index = indexes_to_visit[0];
    let mut next_indexes = indexes_to_visit[1..].to_vec();
    for i in 0..indexes_to_visit.len() {
        let distance_to_next = matrix[current_index][next_index];
        let partial_distance = visit(matrix, next_index, &next_indexes);
        let acc_distance = partial_distance + distance_to_next;
        if acc_distance < min_acc_distance {
            min_acc_distance = acc_distance;
        }
        if i < next_indexes.len() {
            swap(&mut next_index, &mut next_indexes[i]);
        }
    }
    min_acc_distance
}

fn find_shortest_path(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut min_total_distance = i32::MAX;
    let mut next_index = 0;
    let mut next_indexes = (1..matrix.len()).collect();
    for i in 0..matrix.len() {
        let total_distance = visit(matrix, next_index, &next_indexes);
        if total_distance < min_total_distance {
            min_total_distance = total_distance;
        }
        if i < next_indexes.len() {
            swap(&mut next_index, &mut next_indexes[i]);
        }
    }
    min_total_distance
}

fn find_longest_path(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut max_total_distance = i32::MIN;
    let mut next_index = 0;
    let mut next_indexes = (1..matrix.len()).collect();
    for i in 0..matrix.len() {
        let total_distance = visit_long(matrix, next_index, &next_indexes);
        if total_distance > max_total_distance {
            max_total_distance = total_distance;
        }
        if i < next_indexes.len() {
            swap(&mut next_index, &mut next_indexes[i]);
        }
    }
    max_total_distance
}

fn visit_long(matrix: &Vec<Vec<i32>>, current_index: usize, indexes_to_visit: &Vec<usize>) -> i32 {
    if indexes_to_visit.len() == 1 {
        return matrix[current_index][indexes_to_visit[0]];
    }
    let mut max_acc_distance = i32::MIN;
    let mut next_index = indexes_to_visit[0];
    let mut next_indexes = indexes_to_visit[1..].to_vec();
    for i in 0..indexes_to_visit.len() {
        let distance_to_next = matrix[current_index][next_index];
        let partial_distance = visit_long(matrix, next_index, &next_indexes);
        let acc_distance = partial_distance + distance_to_next;
        if acc_distance > max_acc_distance {
            max_acc_distance = acc_distance;
        }
        if i < next_indexes.len() {
            swap(&mut next_index, &mut next_indexes[i]);
        }
    }
    max_acc_distance
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_input(filename);
    let matrix = parse_matrix(&input);
    // part one
    let shortest_path = find_shortest_path(&matrix);
    println!("Shortest path: {}", shortest_path);
    // part two
    let longest_path = find_longest_path(&matrix);
    println!("Longest path: {}", longest_path);
}
