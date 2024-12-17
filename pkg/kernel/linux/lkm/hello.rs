// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

module! {
  type: Hello,
  name: "hello",
  author: "Schubert Anselme <schubert@anselm.es",
  description: "Linux Kernel Module",
  license: "GPL v2",
  params: {},
}

struct Hello;

impl KernelModule for HelloWorld {
  fn init() -> KernelResult<Self> {
    pr_info("Hello world, from the kernel!\n");
    Ok(Hello {})
  }
}

impl Drop for Hello {
  fn drop(&mut self) {
    pr_info!("Goodby!\n");
  }
}
