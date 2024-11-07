// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/anti_patterns/deref.html

use std::ops::Deref;

struct Foo {}
impl Foo {
    fn m(&self) {}
}

struct Bar {
    f: Foo,
}

impl Deref for Bar {
    type Target = Foo;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

fn main() {
    let b = Bar { f: Foo {} };
    b.m();
}
