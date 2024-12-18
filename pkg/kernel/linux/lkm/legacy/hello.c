// SPDX-License-Identifier: GPL-2.0

#include <linux/module.h>
#include <linux/init.h>

MODULE_AUTHOR("Schubert Anselme");
MODULE_DESCRIPTION("Linux Kernel Module");
MODULE_LICENSE("GPL v2");

static int __init load(void)
{
  printk("hello - Hello, Kernel!\n");
  return 0;
}

static void __exit unload(void)
{
  printk("hello - Goodby, Kernel\n");
}

module_init(load);
module_exit(unload);
