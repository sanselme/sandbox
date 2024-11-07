// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html

// fn three_vowels(word: &String) -> bool {
//     let mut vovel_count = 0;
//     for c in word.chars() {
//         match c {
//             'a' | 'e' | 'i' | 'o' | 'u' => {
//                 vovel_count += 1;
//                 if vovel_count >= 3 {
//                     return true;
//                 }
//             }
//             _ => vovel_count = 0,
//         }
//     }
//     false
// }

// fn main() {
//     let ferris = "Ferris".to_string();
//     let curious = "Curious".to_string();
//     println!("{}: {}", ferris, three_vowels(&ferris));
//     println!("{}: {}", curious, three_vowels(&curious));

//     // This works fine, but the following two lines would fail:
//     // println!("Ferris: {}", three_vowels("Ferris"));
//     // println!("Curiuos: {}", three_vowels("Curious"));
// }

fn three_vowels(word: &str) -> bool {
    let mut vovel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vovel_count += 1;
                if vovel_count >= 3 {
                    return true;
                }
            }
            _ => vovel_count = 0,
        }
    }
    false
}

fn main() {
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{word} has three consecutive vowels!");
        }
    }
}
