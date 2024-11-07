// SPDX-License-Identifier: GPL-3.0

#![feature(error_generic_member_access)]

pub mod r#as;
pub mod builder;
pub mod casp;
pub mod constructors;
pub mod cswf;
pub mod edi;
pub mod fid;
pub mod fold;
pub mod kovice;
pub mod obapi;
pub mod pffe;
pub mod ps;
pub mod raii;
pub mod tciw;

mod ehffi {
    pub mod custom;
    pub mod flat;
    mod structured {
        pub mod c_api;
        pub mod error;
    }
}

mod visitor {
    pub mod ast; // The data we will visit
    mod interp;
    pub mod visit; // The abstact visitor

    pub use interp::Interpreter;
}
