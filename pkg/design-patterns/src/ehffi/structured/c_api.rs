// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/ffi/errors.html

use super::error::DatabaseError;

use core::ptr::NonNull;

#[no_mangle]
pub extern "C" fn db_error_description(
    e: Option<NonNull<DatabaseError>>,
) -> Option<NonNull<libc::c_char>> {
    // safety: we assume that the lifetime of `e` is greater than
    // the current stack frame.
    let error = unsafe { e?.as_ref() };

    let error_str: String = match error {
        DatabaseError::IsReadOnly => "cannot write to read-only database".to_string(),
        DatabaseError::IoError(e) => format!("I/O Error: {e}"),
        DatabaseError::FileCorrupted(s) => format!("File corrupted, run repair: {}", &s),
    };
    let error_bytes = error_str.as_bytes();
    let c_error = unsafe {
        // safety: copying error_bytes to an allocated buffer with a '\0'
        // byte at the end.
        let buffer = NonNull::<u8>::new(libc::malloc(error_bytes.len() + 1).cast())?;
        buffer
            .as_ptr()
            .copy_from_nonoverlapping(error_bytes.as_ptr(), error_bytes.len());
        buffer.as_ptr().add(error_bytes.len()).write(0_u8);
        buffer
    };

    Some(c_error.cast())
}
