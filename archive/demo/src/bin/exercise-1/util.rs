// SPDX-License-Identifier: GPL-3.0

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
