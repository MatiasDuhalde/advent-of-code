use std::collections::HashSet;
use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_guard_tile(tile: char) -> bool {
    tile == 'v' || tile == '>' || tile == '<' || tile == '^'
}

fn find_guard_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if is_guard_tile(map[i][j]) {
                return (i, j);
            }
        }
    }
    panic!("Guard not found");
}

fn get_next_tile(
    map: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: char,
) -> Option<(usize, usize)> {
    let i = position.0 as i32;
    let j = position.1 as i32;
    let next_tile = match direction {
        'v' => (i + 1, j),
        '>' => (i, j + 1),
        '<' => (i, j - 1),
        '^' => (i - 1, j),
        _ => panic!("Invalid guard tile"),
    };
    let max_i = map.len() as i32;
    let max_j = map[0].len() as i32;
    if next_tile.0 < 0 || next_tile.0 >= max_i || next_tile.1 < 0 || next_tile.1 >= max_j {
        None
    } else {
        Some((next_tile.0 as usize, next_tile.1 as usize))
    }
}

fn is_obstacle_tile(tile: char) -> bool {
    tile == '#' || tile == 'O'
}

fn get_tile(map: &Vec<Vec<char>>, position: (usize, usize)) -> char {
    let (i, j) = position;
    map[i][j]
}

fn rotate_direction(direction: char) -> char {
    match direction {
        'v' => '<',
        '>' => 'v',
        '<' => '^',
        '^' => '>',
        _ => panic!("Invalid guard tile"),
    }
}

fn count_visited_tiles(map: &Vec<Vec<char>>) -> usize {
    let mut visited_tiles = HashSet::new();
    let mut guard_position = find_guard_position(map);
    let mut guard_direction = get_tile(map, guard_position);
    visited_tiles.insert(guard_position);
    loop {
        let next_position = get_next_tile(map, guard_position, guard_direction);
        if next_position.is_none() {
            break;
        }
        let position = next_position.unwrap();
        if is_obstacle_tile(get_tile(map, position)) {
            guard_direction = rotate_direction(guard_direction);
        } else {
            guard_position = position;
            visited_tiles.insert(guard_position);
        }
    }
    visited_tiles.len()
}

fn count_possible_obstacles(map: &Vec<Vec<char>>) -> usize {
    let mut possible_obstacles = 0;
    let starting_guard_position = find_guard_position(map);
    let starting_guard_direction = get_tile(map, starting_guard_position);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let tile = get_tile(map, (i, j));
            if is_obstacle_tile(tile) || is_guard_tile(tile) {
                continue;
            }

            let mut visited_tiles = HashSet::new();
            let mut guard_position = starting_guard_position;
            let mut guard_direction = starting_guard_direction;
            visited_tiles.insert((guard_position, guard_direction));
            loop {
                let next_position = get_next_tile(map, guard_position, guard_direction);
                if next_position.is_none() {
                    break;
                }
                let position = next_position.unwrap();
                if (i, j) == position || is_obstacle_tile(get_tile(map, position)) {
                    guard_direction = rotate_direction(guard_direction);
                } else {
                    guard_position = position;
                }
                if visited_tiles.contains(&(guard_position, guard_direction)) {
                    possible_obstacles += 1;
                    break;
                }
                visited_tiles.insert((guard_position, guard_direction));
            }
        }
    }
    possible_obstacles
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let map = read_input(filename);
    // part one
    let visited_tiles = count_visited_tiles(&map);
    println!("Visited tiles: {}", visited_tiles);
    // part two
    let possible_obstacles = count_possible_obstacles(&map);
    println!("Possible obstacles: {}", possible_obstacles);
}
