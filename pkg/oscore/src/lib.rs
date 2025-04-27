// SPDX-License-Identifier: GPL-3.0

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

pub mod malloc {
    pub mod bump;
    pub mod fixed_sized_block;
    pub mod linked_list;
    pub mod malloc;
}

pub mod task {
    pub mod executor;
    pub mod keyboard;
    pub mod simple_executor;
    pub mod task;
}

pub use malloc::malloc::*;
pub use task::simple_executor::SimpleExecutor;
pub use task::task::*;

use core::any::type_name;
use core::panic::PanicInfo;
use x86_64::instructions;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...\t", type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn init() {
    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize() };
    interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use crate::{hlt_loop, test_panic_handler};
    use bootloader::{entry_point, BootInfo};
    use core::panic::PanicInfo;

    /// Entry pint for `cargo test`
    fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
        init();
        test_main();
        hlt_loop();
    }

    entry_point!(test_kernel_main);

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        test_panic_handler(info)
    }
}
