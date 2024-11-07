// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/creational/fold.html

use crate::visitor::ast::{Expr, Name, Stmt};

pub trait Folder {
    // A leaf node just returns the node itself. In some cases, we can do this
    // to inner nodes too.
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
        n
    }

    // Create a new inner node by folder its children.
    // fn fold_stmt(&mut self, s: Box<Stmt>) -> Box<Stmt> {
    //     match *s {
    //         Stmt::Expr(e) => Box::new(Stmt::Expr(self.fold_expr(e))),
    //         Stmt::Let(n, e) => Box::new(Stmt::Let(self.fold_name(n), self.fold_expr(e))),
    //     }
    // }

    fn fold_expr(&mut self, e: Box<Expr>) -> Box<Expr> {
        e
    }
}

// An example concrete implementation - renames every name to 'foo'.
struct Renamer;
impl Folder for Renamer {
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
        Box::new(Name {
            value: "foo".to_owned(),
        })
    }
    // Use the default methods for the other nodes.
}
