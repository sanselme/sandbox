// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

use super::{
    ast::{Expr, Name, Stmt},
    visit::Visitor,
};

// An example concrete implementation - walks the AST interpreting it as a code.
pub struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    }

    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntList(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

pub fn walk_expr(visitor: &mut dyn Visitor<Interpreter>, e: &Expr) {
    match *e {
        Expr::IntList(_) => {}
        Expr::Add(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        Expr::Sub(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
    }
}
