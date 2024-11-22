// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/behavioural/interpreter.html

use core::str;

pub struct Interpreter<'a> {
    it: str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("unexpected symbol '{op}'");
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("unexpected symbol '{ch}'"),
            None => panic!("unexpected end of string"),
        }
    }
}

fn main() {
    let mut interp = Interpreter::new("2+3");
    let mut postfix = String::new();
    interp.interpret(&mut postfix);
    assert_eq!(postfix, "23+");

    interp = Interpreter::new("1-2+3-4");
    postfix.clear();
    interp.interpret(&mut postfix);
    assert_eq!(postfix, "12-3+4-");
}
