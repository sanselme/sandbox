// SPDX-License-Identifier: GPL-2.0

#include <linux/module.h>
#include <linux/init.h>
#include <linux/fs.h>
#include <linux/device.h>

MODULE_AUTHOR("Schubert Anselme");
MODULE_DESCRIPTION("SYSFS Class Linux Kernel Module");
MODULE_LICENSE("GPL v2");

static dev_t dev_num;
static struct class *dev_class;
static struct device *dev;

static char text[64] = "Hello world!";
static int answer = 42;

ssize_t store(struct device *d, struct device_attribute *a, const char *buf, size_t len)
{
  int size = len < sizeof(text) ? len : sizeof(text);
  strncpy(text, buf, size);
  return size;
}

ssize_t show(struct device *d, struct device_attribute *a, char *buf)
{
  strcpy(buf, text);
  return strlen(text);
}

DEVICE_ATTR(text, 0660, show, store);
DEVICE_INT_ATTR(answer, 0660, answer);

static int __init load(void)
{
  int status;

  status = alloc_chrdev_region(&dev_num, 0, MINORMASK + 1, "cdev");
  if (status != 0)
  {
    pr_err("sysfs - Error allocating the device number\n");
  }

  dev_class = class_create("lkm");
  if (IS_ERR(dev_class))
  {
    pr_err("sysfs - Error creating class\n");
    status = PTR_ERR(dev_class);
    goto free_dev_num;
  }

  dev = device_create(dev_class, NULL, dev_num, NULL, "cdev%d", MINOR(dev_num));
  if (IS_ERR(dev))
  {
    pr_err("sysfs - Error creating device\n");
    status = PTR_ERR(dev);
    goto free_class;
  }

  status = device_create_file(dev, &dev_attr_text);
  if (status != 0)
  {
    pr_err("sysfs - Error creating text attribute\n");
    goto free_dev;
  }

  status = device_create_file(dev, &dev_attr_answer.attr);
  if (status != 0)
  {
    pr_err("sysfs - Error creating answer attribute\n");
    goto free_text;
  }

  return 0;

free_text:
  device_remove_file(dev, &dev_attr_text);
free_dev:
  device_destroy(dev_class, dev_num);
free_class:
  class_destroy(dev_class);
free_dev_num:
  unregister_chrdev_region(dev_num, MINORMASK + 1);
  return status;
}

static void __exit unload(void)
{
  device_remove_file(dev, &dev_attr_answer.attr);
  device_remove_file(dev, &dev_attr_text);
  device_destroy(dev_class, dev_num);
  class_destroy(dev_class);
  unregister_chrdev_region(dev_num, MINORMASK + 1);
}

module_init(load);
module_exit(unload);
