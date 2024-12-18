KDIR = /lib/modules/$(shell uname -r)/build
KMOD = \
	cdev \
	gpio \
	hello \
	sysfs

obj-m += $(foreach mod,$(KMOD),$(mod).o)

all:
	make -C $(KDIR) M=$(PWD) modules

clean:
	make -C $(KDIR) M=$(PWD) clean

load:
	for mod in $(KMOD); do sudo insmod $$mod.ko; done

unload:
	for mod in $(KMOD); do sudo rmmod $$mod; done
