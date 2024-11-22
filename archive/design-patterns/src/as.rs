// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/ffi/accepting-strings.html

use std::ffi::{CStr, CString};

pub enum LogLevel {
    Info,
    Debug,
    Alert,
}

/// Log a message at the specified level.
///
/// # Safety
///
/// It is the caller's guarantee to ensure `msg`:
///
/// - is not a null pointer
/// - points to valid, inintialized data
/// - points to memory ending in a null byte
/// - won't be mutated for the duration of this function call
#[no_mangle]
pub unsafe extern "C" fn unsafe_mylib_log(msg: *const libc::c_char, level: libc::c_int) {
    let level: LogLevel = LogLevel::Info;

    // safety: The caller has already guaranteed this is OK (see the
    // `# Safety` section of the doc-comment).
    let msg_str: &str = match CStr::from_ptr(msg).to_str() {
        Ok(s) => s,
        Err(_e) => {
            log_error("ffi string conversion failed");
            return;
        }
    };

    log(msg_str, level);
}

pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
    // DO NOT USE THIS CODE.
    // IT IS UGLY, VERBOSE AND CONTAINS A SUBTLE BUG.

    let level: LogLevel = LogLevel::Info;

    let msg_len = unsafe { libc::strlen(msg) }; // safety: strlen is what it is, I guess?
    let mut msg_data = Vec::with_capacity(msg_len + 1);

    let msg_cstr: CString = unsafe {
        // safety: copying from a forein pointer expected to live
        // for the entire stack frame into owned memory
        // ptr::copy_nonoverlapping(msg, msg_data.as_mut(), msg_len); fixme: the trait `AsMut<u8>` is not implemented for `std::vec::Vec<_>`

        msg_data.set_len(msg_len + 1);
        CString::from_vec_with_nul(msg_data).unwrap()
    };

    let msg_str: String = unsafe {
        match msg_cstr.into_string() {
            Ok(s) => s,
            Err(_e) => {
                log_error("ffi string conversion failed");
                return;
            }
        }
    };

    log(&msg_str, level);
}

fn log_error(_msg: &str) {}
fn log(_msg: &str, _level: LogLevel) {}
