// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/mem-replace.html

use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // This takes out our `name` and puts in an empty String instrad
        // note: empty strings don't allocate.
        // Then, construct the new enum variant (which will
        // be assigned to `*e`).
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}
