// SPDX-License-Identifier: GPL-2.0

#include <linux/module.h>
#include <linux/init.h>
#include <linux/fs.h>

MODULE_AUTHOR("Schubert Anselme");
MODULE_DESCRIPTION("Character Device Linux Kernel Module");
MODULE_LICENSE("GPL v2");

static int major;

static ssize_t read(struct file *f, char __user *u, size_t l, loff_t *o)
{
  pr_info("cdev - Read is called\n");
  return 0;
}

static struct file_operations fops = {
    .read = read};

static int __init load(void)
{
  major = register_chrdev(0, "cdev", &fops);
  if (major < 0)
  {
    pr_err("cdev - Error registering chrdev\n");
    return major;
  }

  printk("cdev - Major Device Number: %d\n", major);
  return 0;
}

static void __exit unload(void)
{
  unregister_chrdev(major, "cdev");
}

module_init(load);
module_exit(unload);
