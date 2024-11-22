// SPDX-License-Identifier: GPL-3.0

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oscore::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use oscore::memory::{self, BootInfoFrameAllocator};
use oscore::task::executor::Executor;
use oscore::task::keyboard;
use oscore::{init, init_heap, println, Task};
use x86_64::VirtAddr;

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    init();

    let phy_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phy_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypress()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    oscore::hlt_loop();
}

entry_point!(kernel_main);

#[cfg(test)]
mod tests {
    use core::panic::PanicInfo;
    use oscore::test_panic_handler;

    /// This function is called on panic.
    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        test_panic_handler(info)
    }

    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
