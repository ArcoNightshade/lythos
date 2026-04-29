# Lythos / OROS — next steps

## Shell (lysh)

- [x] Command history (up-arrow scrolls previous commands)
- [ ] Tab completion (builtins only for now)
- [x] `uptime` — print milliseconds since boot via SYS_TIME
- [x] `free`   — print free physical frames via a new SYS_MEM_STAT syscall
- [x] `kill <tid>` — terminate a task by ID (new SYS_TASK_KILL syscall)

## Filesystem

- [ ] VirtIO block device driver (virtio-blk, MMIO or PCI)
- [ ] Raw block read/write syscalls (SYS_BLK_READ / SYS_BLK_WRITE)
- [ ] Simple flat filesystem (custom or FAT32)
- [ ] VFS layer: SYS_OPEN, SYS_READ, SYS_WRITE, SYS_CLOSE, SYS_STAT
- [ ] lysh `exec <path>` — load and run an ELF off the filesystem
- [ ] lysh `ls`, `cat`, `cp`, `rm`

## Networking

- [ ] VirtIO network device driver (virtio-net)
- [ ] Ethernet + ARP
- [ ] IP + ICMP (ping)
- [ ] TCP stack
- [ ] SYS_SOCKET / SYS_BIND / SYS_CONNECT / SYS_SEND / SYS_RECV

## Kernel reliability

- [ ] IOAPIC driver (replace 8259 PIC — needed for VirtIO PCI interrupts)
- [ ] Multi-processor support (AP startup, per-CPU scheduler)
- [ ] Larger default kernel stack (current 16 KiB can be tight)
- [ ] Kernel ASLR

## Display / GUI

- [ ] VGA text-mode fallback (80×25)
- [ ] Framebuffer driver (VESA / Multiboot framebuffer tag)
- [ ] Basic window manager (webwm is already in OROS, needs a framebuffer)
