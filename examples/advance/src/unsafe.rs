// SPDX-License-Identifier: GPL-3.0

use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

// note: external api
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// note: private api
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])

    // unsafe: block
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe: trait
unsafe trait Foo {}
unsafe impl Foo for i32 {}

// unsafe: functions
unsafe fn dangerous() {}

// unsafe: external
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // note: undefined behavior
    // let address = 0x012345usize;
    // let r = address as *mut i32;
    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // note: nothing printed
    // call_from_c();

    add_to_count(3);

    // unsafe: block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();

        println!("Absolute value of -3 according to C: {}", abs(-3));

        // call_from_c();

        println!("COUNTER: {COUNTER}");
    }

    println!("name is: {HELLO_WORLD}");
}
