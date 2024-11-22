KMOD	:= $(shell pwd)
KDIR	:= /usr/local/src/linux

ARCH	:= arm64
TARGET	:= hello

obj-m += $(TARGET).o

default:
	LLVM=1 CC=clang ARCH=$(ARCH) make -C $(KDIR) M=$(KMOD) modules

load:
	sudo insmod $(TARGET).ko
	sudo lsmod | grep $(TARGET)
	sudo dmesg | tail -n 5

unload:
	sudo rmmod $(TARGET)
	sudo lsmod | grep $(TARGET) || true
	sudo dmesg | tail -n 5

install:
	LLVM=1 CC=clang ARCH=$(ARCH) make -C $(KDIR) M=$(KMOD) modules_install

clean:
	LLVM=1 CC=clang ARCH=$(ARCH) make -C $(KDIR) M=$(KMOD) clean
