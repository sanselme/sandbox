// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

use super::ast::{Expr, Name, Stmt};

pub trait Visitor<T> {
    fn visit_name(&mut self, n: &Name) -> T;
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
}
