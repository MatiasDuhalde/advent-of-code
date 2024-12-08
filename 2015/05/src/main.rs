use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn has_at_least_tree_vowels(chars: &Vec<char>) -> bool {
    let mut count = 0;
    for c in chars {
        if *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u' {
            count += 1;
        }
        if count > 2 {
            return true;
        }
    }
    false
}

fn has_repeated_letter(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

// static bad strings
static BAD_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn has_bad_strings(word: &String) -> bool {
    for bad in BAD_STRINGS {
        if word.contains(bad) {
            return true;
        }
    }
    false
}

fn is_nice_word(word: &String) -> bool {
    let chars = word.chars().collect();
    has_at_least_tree_vowels(&chars) && has_repeated_letter(&chars) && !has_bad_strings(word)
}

fn count_nice_words(words: &Vec<String>) -> i32 {
    let mut count = 0;
    for word in words {
        if is_nice_word(word) {
            count += 1;
        }
    }
    count
}

fn has_repeated_letter_with_gap(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn has_repeated_pair(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() - 3 {
        for j in i + 2..chars.len() - 1 {
            if chars[i] == chars[j] && chars[i + 1] == chars[j + 1] {
                return true;
            }
        }
    }
    false
}

fn is_nice_word_2(word: &String) -> bool {
    let chars = word.chars().collect();
    has_repeated_letter_with_gap(&chars) && has_repeated_pair(&chars)
}

fn count_nice_words_2(words: &Vec<String>) -> i32 {
    let mut count = 0;
    for word in words {
        if is_nice_word_2(word) {
            count += 1;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let words = read_input(filename);
    // part one
    let count = count_nice_words(&words);
    println!("Nice words: {}", count);
    // part two
    let count = count_nice_words_2(&words);
    println!("Nice words: {}", count);
}
