OROS_DIR   := ../RaptorOS
ROOTFS_BIN := rootfs/bin
TARGET     := x86_64-oros
OROS_OUT   := $(OROS_DIR)/target/$(TARGET)/release

KERNEL_DEBUG   := target/x86_64-lythos/debug/lythos
KERNEL_RELEASE := target/x86_64-lythos/release/lythos

QEMU := qemu-system-x86_64
QEMU_FLAGS := -serial stdio -display none \
              -drive file=disk.img,format=raw,if=none,id=hd0 \
              -device virtio-blk-pci,drive=hd0

.PHONY: all oros kernel run run-release debug clean

all: oros kernel

## Build OROS userspace binaries and copy to rootfs/bin/
oros:
	cd $(OROS_DIR) && cargo build --release
	mkdir -p $(ROOTFS_BIN)
	cp $(OROS_OUT)/lythd     $(ROOTFS_BIN)/lythd
	cp $(OROS_OUT)/lythdist  $(ROOTFS_BIN)/lythdist
	cp $(OROS_OUT)/lysh      $(ROOTFS_BIN)/lysh
	cp $(OROS_OUT)/rutils    $(ROOTFS_BIN)/rutils

## Build the kernel (also runs build.rs → mkrfs → disk.img)
kernel:
	cargo build

kernel-release:
	cargo build --release

## Run debug kernel under QEMU
run: kernel
	$(QEMU) -kernel $(KERNEL_DEBUG) $(QEMU_FLAGS)

## Run release kernel under QEMU
run-release: kernel-release
	$(QEMU) -kernel $(KERNEL_RELEASE) $(QEMU_FLAGS)

## Run with interrupt/triple-fault tracing
debug: kernel
	$(QEMU) -kernel $(KERNEL_DEBUG) $(QEMU_FLAGS) -d int,cpu_reset

clean:
	cargo clean
	cd $(OROS_DIR) && cargo clean
	rm -f disk.img
