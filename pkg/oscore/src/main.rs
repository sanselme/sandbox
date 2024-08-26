// SPDX-License-Identifier: GPL-3.0

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oscore::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use oscore::{hlt_loop, init, println};
use x86_64::registers::control::Cr3;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    init();

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oscore::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
