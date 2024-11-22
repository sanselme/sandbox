// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html

pub fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The dtor of _exit will run however the function `bar` is exited.
    let _exit = Foo;

    // Implicit return with `?` operator.
    baz()?;

    // Normal return.
    Ok(())
}

fn baz() -> Result<(), ()> {
    Ok(())
}
