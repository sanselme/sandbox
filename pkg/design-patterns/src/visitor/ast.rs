// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

pub enum Stmt {
    Expr(Expr),
    Let(Name, Expr),
}

pub enum Expr {
    IntList(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

pub struct Name {
    pub(crate) value: String,
}
