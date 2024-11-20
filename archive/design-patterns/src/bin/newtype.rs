// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html

use std::fmt::Display;

// Create NewType Password to override the Display trait for String
struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

fn main() {
    let unsecured_password = "ThisIsMyPassword".to_string();
    let secured_password = Password(unsecured_password.clone());
    println!("unsecure_password: {unsecured_password}");
    println!("secured_password: {secured_password}");
}
