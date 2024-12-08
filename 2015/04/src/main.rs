use md5::{Digest, Md5};
use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().trim().to_string()
}

fn find_number(input: &str) -> u32 {
    let mut i = 0;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input.as_bytes());
        hasher.update(i.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] < 0x10 {
            return i;
        }
        i += 1;
    }
}

fn find_number_2(input: &str) -> u32 {
    let mut i = 0;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input.as_bytes());
        hasher.update(i.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_input(filename);
    // part one
    let result = find_number(&input);
    println!("{}", result);
    // part two
    let result = find_number_2(&input);
    println!("{}", result);
}
