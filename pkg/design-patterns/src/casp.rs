// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/deref.html

use std::ops::Deref;

struct RawVec<T> {
    _finzlizer: T,
}

struct Vec<T> {
    data: RawVec<T>,
    // todo: other fields
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        todo!()
    }
}
