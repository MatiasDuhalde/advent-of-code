use std::env;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().trim().to_string()
}

fn increment_password(password: &str) -> String {
    let mut password_chars: Vec<char> = password.chars().collect();
    let mut current_index = password_chars.len();
    let mut carry = true;
    let mut new_char;
    while carry {
        current_index -= 1;
        (new_char, carry) = increment_char_with_restriction(password_chars[current_index]);
        password_chars[current_index] = new_char;
    }
    password_chars.iter().collect()
}

fn is_valid_password(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let mut i = 0;
    let mut pairs = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            if pairs == 1 {
                pairs = 2;
                break;
            }
            pairs = 1;
            i += 1
        }
        i += 1;
    }
    if pairs < 2 {
        return false;
    }
    let mut i = 0;
    while i < chars.len() - 2 {
        let c = chars[i] as u8;
        if c + 2 == chars[i + 2] as u8 && chars[i + 1] as u8 == c + 1 {
            return true;
        }
        i += 1;
    }
    false
}

fn increment_char_with_restriction(c: char) -> (char, bool) {
    match c {
        'h' => ('j', false),
        'k' => ('m', false),
        'n' => ('p', false),
        _ => increment_char(c),
    }
}

fn increment_char(c: char) -> (char, bool) {
    if c == 'z' {
        ('a', true)
    } else {
        (((c as u8 - 96) % 26 + 97) as char, false)
    }
}

fn find_new_password(password: &str) -> String {
    let mut new_password = increment_password(password);
    while !is_valid_password(&new_password) {
        new_password = increment_password(&new_password);
    }
    new_password
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let original_password = read_input(filename);
    // part one
    let new_password = find_new_password(&original_password);
    println!("New password: {}", new_password);
    // part two
    let another_password = find_new_password(&new_password);
    println!("Another password: {}", another_password);
}
