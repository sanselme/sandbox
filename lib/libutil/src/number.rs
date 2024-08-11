// SPDX-License-Identifier: GPL-3.0

use std::collections::HashMap;

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
