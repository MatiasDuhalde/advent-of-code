use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn is_in_map(map: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    position.0 >= 0
        && position.0 < map.len() as i32
        && position.1 >= 0
        && position.1 < map[0].len() as i32
}

fn build_antenna_map(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let character = map[i][j];
            if character != '.' {
                antennas.entry(character).or_insert(Vec::new()).push((i, j));
            }
        }
    }
    antennas
}

fn get_vector(position1: (usize, usize), position2: (usize, usize)) -> (i32, i32) {
    (
        position2.0 as i32 - position1.0 as i32,
        position2.1 as i32 - position1.1 as i32,
    )
}

fn count_antinodes(map: &Vec<Vec<char>>) -> usize {
    let antennas = build_antenna_map(map);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (_, v) in &antennas {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let antenna1 = v[i];
                let antenna2 = v[j];
                let vector = get_vector(antenna1, antenna2);
                let antinode1 = add_vector(antenna2, vector);
                if is_in_map(map, antinode1) {
                    antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                }
                let antinode2 = subtract_vector(antenna1, vector);
                if is_in_map(map, antinode2) {
                    antinodes.insert((antinode2.0 as usize, antinode2.1 as usize));
                }
            }
        }
    }
    antinodes.len()
}

fn add_vector(position: (usize, usize), vector: (i32, i32)) -> (i32, i32) {
    (position.0 as i32 + vector.0, position.1 as i32 + vector.1)
}

fn subtract_vector(position: (usize, usize), vector: (i32, i32)) -> (i32, i32) {
    (position.0 as i32 - vector.0, position.1 as i32 - vector.1)
}

fn count_updated_antinodes(map: &Vec<Vec<char>>) -> usize {
    let antennas = build_antenna_map(map);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (_, v) in &antennas {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let antenna1 = v[i];
                let antenna2 = v[j];
                let vector = get_vector(antenna1, antenna2);
                let mut antinode = add_vector(antenna1, vector);
                while is_in_map(map, antinode) {
                    let antinode_usize = (antinode.0 as usize, antinode.1 as usize);
                    antinodes.insert(antinode_usize);
                    antinode = add_vector(antinode_usize, vector);
                }
                let mut antinode = subtract_vector(antenna2, vector);
                while is_in_map(map, antinode) {
                    let antinode_usize = (antinode.0 as usize, antinode.1 as usize);
                    antinodes.insert(antinode_usize);
                    antinode = subtract_vector(antinode_usize, vector);
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let map = read_input(filename);
    // part one
    let antinodes = count_antinodes(&map);
    println!("Antinodes: {}", antinodes);
    // part two
    let updated_antinodes = count_updated_antinodes(&map);
    println!("Updated antinodes: {}", updated_antinodes);
}
