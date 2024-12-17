obj=gpio
obj-m += gpio.o

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

load:
	sudo insmod gpio.ko

unload:
	sudo rmmod gpio

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
