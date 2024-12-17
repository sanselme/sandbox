KDIR = /lib/modules/$(shell uname -r)/build
KMOD = hello

obj-m += $(KMOD).o

all:
	make -C $(KDIR) M=$(PWD) modules

load:
	sudo insmod $(KMOD).ko

unload:
	sudo rmmod $(KMOD)

clean:
	make -C $(KDIR) M=$(PWD) clean
