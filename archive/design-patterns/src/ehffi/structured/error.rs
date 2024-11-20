// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/ffi/errors.html

use std::io::Error;

pub enum DatabaseError {
    IsReadOnly,
    IoError(Error),
    FileCorrupted(String), // message describing the issue
}

impl From<DatabaseError> for libc::c_int {
    fn from(e: DatabaseError) -> libc::c_int {
        match e {
            DatabaseError::IsReadOnly => 1,
            DatabaseError::IoError(_) => 2,
            DatabaseError::FileCorrupted(_) => 3,
        }
    }
}
