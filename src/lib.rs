// SPDX-License-Identifier: GPL-3.0

use std::collections::HashMap;

pub enum Command {
    Quit,
    All,
    List(String),
    Add { dept: String, name: String },
}

impl Command {
    pub fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        match words.as_slice() {
            ["Quit"] => Some(Command::Quit),
            ["All"] => Some(Command::All),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            &_ => None,
        }
    }
}

pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

pub fn mode(numbers: &[i32]) -> i32 {
    let mut occurences = HashMap::new();

    for &value in numbers {
        *occurences.entry(value).or_insert(0) += 1;
    }

    occurences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

pub fn pigify(text: &str) -> String {
    text.split_whitespace()
        .map(pigify_one)
        .fold(String::new(), folder)
}

fn pigify_one(word: &str) -> String {
    let mut chars = word.chars();

    let first_char = match chars.next() {
        None => return String::new(),
        Some(c) => c,
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

fn folder(mut current: String, next: String) -> String {
    if !current.is_empty() {
        current.push(' ');
    }
    current.push_str(&next);
    current
}
