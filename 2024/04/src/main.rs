use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn rotate_matrix_clockwise(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = vec![vec![' '; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            new_matrix[j][matrix.len() - 1 - i] = matrix[i][j];
        }
    }
    new_matrix
}

fn count_xmas(matrix: &Vec<Vec<char>>) -> i32 {
    let mut rot_matrix = matrix.clone();
    let mut count = find_xmas_horizontally(&rot_matrix);
    count += find_xmas_diagonally(&rot_matrix);
    for _ in 0..3 {
        rot_matrix = rotate_matrix_clockwise(&rot_matrix);
        count += find_xmas_horizontally(&rot_matrix);
        count += find_xmas_diagonally(&rot_matrix);
    }
    count
}

fn find_xmas_horizontally(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() - 3 {
            if matrix[i][j] == 'X'
                && matrix[i][j + 1] == 'M'
                && matrix[i][j + 2] == 'A'
                && matrix[i][j + 3] == 'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn find_xmas_diagonally(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..matrix.len() - 3 {
        for j in 0..matrix[i].len() - 3 {
            if matrix[i][j] == 'X'
                && matrix[i + 1][j + 1] == 'M'
                && matrix[i + 2][j + 2] == 'A'
                && matrix[i + 3][j + 3] == 'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn find_cross(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..matrix.len() - 2 {
        for j in 0..matrix[i].len() - 2 {
            if matrix[i][j] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j + 2] == 'S'
                && matrix[i + 2][j] == 'M'
                && matrix[i][j + 2] == 'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_cross(matrix: &Vec<Vec<char>>) -> i32 {
    let mut rot_matrix = matrix.clone();
    let mut count = find_cross(&rot_matrix);
    rot_matrix = rotate_matrix_clockwise(&rot_matrix);
    count += find_cross(&rot_matrix);
    rot_matrix = rotate_matrix_clockwise(&rot_matrix);
    count += find_cross(&rot_matrix);
    rot_matrix = rotate_matrix_clockwise(&rot_matrix);
    count += find_cross(&rot_matrix);
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let matrix = read_input(filename);
    // part one
    let xmas_count = count_xmas(&matrix);
    println!("Count: {}", xmas_count);
    // part two
    let cross_count = count_cross(&matrix);
    println!("Cross count: {}", cross_count);
}
