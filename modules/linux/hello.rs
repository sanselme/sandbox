// SPDX-License-Identifier: GPL-2.0

/// Linux lkm module
use kernel::prelude::*;

module! {
    type: Hello,
    name: "hello",
    author: "Schubert Anselme <schubert@anselm.es>",
    description: "Linux lkm module",
    license: "GPL",
}

struct Hello;

impl kernel::Module for Hello {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("lkm - hello world\n");
        Ok(Self)
    }
}

impl Drop for Hello {
    fn drop(&mut self) {
        pr_info!("lkm - goodbye world\n");
    }
}
