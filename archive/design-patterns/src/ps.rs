// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/ffi/passing-strings.html

use std::ffi::{self, CString, NulError};

extern "C" {
    fn seterr(message: *const libc::c_char);
    fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
}

fn report_error_to_ffi<S>(err: S) -> Result<(), NulError>
where
    S: Into<String>,
{
    let c_err = CString::new(err.into())?;

    unsafe {
        // safety: calling an FFI whose documentation says the pointer is
        // const, so no modification should occur
        seterr(c_err.as_ptr());
    }

    Ok(())
    // the lifetime of c_err continues until here
}

fn get_error_from_ffi() -> Result<String, ffi::IntoStringError> {
    let mut buffer = vec![0_u8; 1024];
    unsafe {
        // safety: calling an FFI whose documentation implies
        // that the input need only live as long as the call
        let written: usize = geterr(buffer.as_mut_ptr(), 1023).try_into().unwrap();
        buffer.truncate(written + 1);
    }

    CString::new(buffer).unwrap().into_string()
}

fn report_error<S>(err: S) -> Result<(), NulError>
where
    S: Into<String>,
{
    unsafe {
        // safety: whoops, this contains a dangling pointer!
        seterr(CString::new(err.into())?.as_ptr());
    }
    Ok(())
}
