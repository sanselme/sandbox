// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/anti_patterns/borrow_clone.html

fn main() {
    // define any variable
    let mut x = 5;

    // borrow `x` -- but clone it first
    let y = &mut x.clone();

    // without the x.clone() two lines prior, this line would fail on compile as
    // x has been borrowed
    // thanks to the x.clone(), x was never borrowed, and this line will run.
    println!("{x}");

    // perform some action on the borrow to prevent rust from optimizing this
    // out of existence
    *y += 1;
}
