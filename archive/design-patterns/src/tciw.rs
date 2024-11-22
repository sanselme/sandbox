// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/ffi/wrappers.html

use std::{collections::btree_map::Keys, ptr::NonNull};

struct MySet;
impl MySet {
    fn keys(&self) -> Keys<&Key, ()> {
        todo!()
    }
}
struct Key;

struct MySetWrapper {
    myset: MySet,
    iter_next: usize,
    // created from a transmuted Box<KeysIter + 'self>
    // iterator: Option<NonNull<KeysIter<'static>>>,
}

impl MySetWrapper {
    pub fn first_key(&mut self) -> Option<&Key> {
        self.iter_next = 0;
        self.next_key()
    }

    pub fn next_key(&mut self) -> Option<&Key> {
        if let Some(next) = self.myset.keys().nth(self.iter_next) {
            self.iter_next += 1;
            Some(next)
        } else {
            None
        }
    }
}

pub mod unsafe_module {
    // other module content

    use super::MySetWrapper;

    // pub fn myset_store(myset: *mut MySetWrapper, key: datum, value: datum) -> libc::c_int {
    //     // DO NOT USE THIS CODE. IT IS UNSAFE TO DEMONSTRATE A PROLBEM.

    //     let myset = unsafe {
    //         // safety: whoops, UB occurs in here!
    //         &mut (*myset).myset
    //     };

    //     // ...check and cast key and value data...

    //     match myset.store(casted_key, casted_key) {
    //         Ok(_) => 0,
    //         Err(e) => e.into(),
    //     }
    // }
}
