// SPDX-License-Identifier: GPL-2.0

#include <linux/module.h>
#include <linux/init.h>
#include <linux/gpio/consumer.h>

MODULE_AUTHOR("Schubert Anselme");
MODULE_DESCRIPTION("GPIO Linux Kernel Module");
MODULE_LICENSE("GPL v2");

#define IO_BUTTON 20
#define IO_LED 21
#define IO_OFFSET 0

static struct gpio_desc *led, *button;

static int load(void)
{
  int status;

  button = gpio_to_desc(IO_BUTTON + IO_OFFSET);
  if (!button)
  {
    pr_err("gpio - Error getting pin 20\n");
    return -ENODEV;
  }

  led = gpio_to_desc(IO_LED + IO_OFFSET);
  if (!led)
  {
    pr_err("gpio - Error getting pin 21\n");
    return -ENODEV;
  }

  status = gpiod_direction_output(led, 0);
  if (status)
  {
    pr_err("gpio - Error setting pin 20 to output\n");
    return status;
  }

  status = gpiod_direction_input(button);
  if (status)
  {
    pr_err("gpio - Error setting pin 21 to input\n");
    return status;
  }

  gpiod_set_value(led, 1);
  pr_info("gpio - Button is %spressed\n", gpiod_get_value(button) ? "" : "not ");

  return 0;
}

static void unload(void)
{
  gpiod_set_value(led, 0);
}

module_init(load);
module_exit(unload);
