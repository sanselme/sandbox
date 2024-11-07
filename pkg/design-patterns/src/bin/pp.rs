// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/functional/paradigms.html

fn main() {
    // imperative
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("{sum}");

    // declarative
    println!("{}", (1..11).fold(0, |a, b| a + b));
}
