// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/ffi/export.html

struct Dbm {}
impl Dbm {
    pub fn keys<'it>(&'it self) -> DbmKeysIter<'it> {
        todo!()
    }
}

struct DbmKeysIter<'it> {
    owner: &'it Dbm,
}
impl<'it> Iterator for DbmKeysIter<'it> {
    type Item = Dbm;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// #[no_mangle]
// pub extern "C" fn dbm_iter_new(owner: *const Dbm) -> *mut DbmKeysIter {
//     // THIS API IS A BAD IDEA! For real applications, use object-based design instead.
//     todo!()
// }

// #[no_mangle]
// pub extern "C" fn dbm_iter_next(iter: *mut DbmKeysIter, key_out: *const datum) -> libc::c_int {
//     // THIS API IS A BAD IDEA! For real applications, use object-based design instead.
//     todo!()
// }

// #[no_mangle]
// pub extern "C" fn dbm_iter_del(*mut DbmKeysIter) {
//     // THIS API IS A BAD IDEA! For real applications, use object-based design instead.
//     todo!()
// }
