// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/behavioural/interpreter.html

fn main() {
    let x = -3_f64;
    let y = 4_f64;

    assert_eq!(3_f64, norm!(x));
    assert_eq!(5_f64, norm!(x, y));
    assert_eq!(0_f64, norm!(0, 0, 0));
    assert_eq!(1_f64, norm!(0.5, -0.5, 0.5, -0.5));
}

#[macro_export]
macro_rules! norm {
    ($($element:expr),*) => {
        {
            let mut n = 0.0;
            $(
                n += ($element as f64)*($element as f64);
            )*
            n.sqrt()
        }
    };
}
