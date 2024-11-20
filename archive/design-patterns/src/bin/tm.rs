// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/temporary-mutability.html

fn main() {
    let data = {
        let mut data = get_vec();
        data.sort();
        data
    };
    // Here `data` is immutable.

    let mut data = get_vec();
    data.sort();
    let data = data;
    // Here `data` is immutable.
}

fn get_vec() -> Vec<()> {
    Vec::new()
}
