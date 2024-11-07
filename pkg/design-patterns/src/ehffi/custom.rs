// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/ffi/errors.html

struct ParseError {
    expected: char,
    line: u32,
    ch: u16,
}

impl ParseError {}

// Create a second version which is exposed as a C structure
#[repr(C)]
pub struct parse_error {
    pub expected: libc::c_char,
    pub line: u32,
    pub ch: u16,
}

impl From<ParseError> for parse_error {
    fn from(e: ParseError) -> parse_error {
        let ParseError { expected, line, ch } = e;
        parse_error {
            expected: expected as u8,
            line,
            ch,
        }
    }
}
