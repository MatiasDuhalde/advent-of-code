use std::cmp;
use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<(u32, u32, u32)> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut presents = Vec::new();
    for line in contents.lines() {
        let mut sides = line.split('x');
        let a: u32 = sides.next().unwrap().parse().unwrap();
        let b: u32 = sides.next().unwrap().parse().unwrap();
        let c: u32 = sides.next().unwrap().parse().unwrap();
        presents.push((a, b, c));
    }
    return presents;
}

fn calculate_wrapping_area(dimensions: &(u32, u32, u32)) -> u32 {
    let (x, y, z) = dimensions;
    let area1 = x * y;
    let area2 = y * z;
    let area3 = x * z;
    let slack = cmp::min(cmp::min(area1, area2), area3);
    2 * area1 + 2 * area2 + 2 * area3 + slack
}

fn calculate_ribbon_length(dimensions: &(u32, u32, u32)) -> u32 {
    let (x, y, z) = dimensions;
    let mut array: [u32; 3] = (*dimensions).into();
    array.sort();
    let a = array[0];
    let b = array[1];
    2 * a + 2 * b + x * y * z
}

fn calculate_total_wrapping_area(presents: &Vec<(u32, u32, u32)>) -> u32 {
    presents
        .iter()
        .fold(0, |acc, present| acc + calculate_wrapping_area(present))
}

fn calculate_total_ribbon_length(presents: &Vec<(u32, u32, u32)>) -> u32 {
    presents
        .iter()
        .fold(0, |acc, present| acc + calculate_ribbon_length(present))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let presents = read_input(filename);
    // part one
    let total_wrapping_area = calculate_total_wrapping_area(&presents);
    println!("Total wrapping area: {}", total_wrapping_area);
    // part two
    let total_ribbon_length = calculate_total_ribbon_length(&presents);
    println!("Total ribbon length: {}", total_ribbon_length);
}
