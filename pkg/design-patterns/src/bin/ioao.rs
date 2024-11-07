// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/option-iter.html

fn main() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);
    println!("{logicians:?}");

    // equivalent to
    // if let Some(turing_inner) = turing {
    //     logicians.push(turing_inner);
    // }

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{logician} is a logician");
    }
}
