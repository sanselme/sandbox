KDIR := ""
KMOD := $(shell pwd)

obj-m := hello.o

all:
	make -C $(KDIR) M=$(KMOD) modules

clean:
	make -C $(KDIR) M=$(KMOD) clean
