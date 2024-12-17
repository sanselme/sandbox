KDIR = /lib/modules/$(shell uname -r)/build
KMOD = cdev

MAJOR = 236
MINOR = 0

obj-m += $(KMOD).o

all:
	make -C $(KDIR) M=$(PWD) modules

load:
	sudo insmod $(KMOD).ko
	sudo mknod /dev/$(KMOD)$(MINOR) c $(MAJOR) $(MINOR)

unload:
	sudo rm -f /dev/$(KMOD)$(MINOR) c $(MAJOR) $(MINOR)
	sudo rmmod $(KMOD)

clean:
	make -C $(KDIR) M=$(PWD) clean
