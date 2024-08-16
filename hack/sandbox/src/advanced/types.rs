// SPDX-License-Identifier: GPL-3.0

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

// fn bar() -> ! {}

fn takes_long_type(f: Thunk) {}
fn returns_long_type() -> Thunk {
    Box::new(|| {})
}

fn generic<T>(t: T) {}
fn generic_sized<T: ?Sized>(t: &T) {}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    generic(5);
    generic_sized(&String::from("hello"));
}
