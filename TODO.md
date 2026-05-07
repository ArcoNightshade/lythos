# Lythos / OROS — next steps

## Shell (lysh)

- [x] Command history (up-arrow scrolls previous commands)
- [x] Tab completion (builtins only for now)
- [x] `uptime` — print milliseconds since boot via SYS_TIME
- [x] `free` — print free physical frames via a new SYS_MEM_STAT syscall
- [x] `kill <tid>` — terminate a task by ID (new SYS_TASK_KILL syscall)
- [ ] Pipe support (`cmd1 | cmd2`) — needs SYS_IPC_CREATE plumbing in lysh
- [ ] I/O redirection (`>`, `<`)

## Filesystem

- [x] VirtIO block device driver (virtio-blk, MMIO or PCI)
- [x] Raw block read/write syscalls (SYS_BLK_READ / SYS_BLK_WRITE)
- [x] RFS kernel driver (`src/rfs.rs`) — read/write/lookup/stat; extent allocator, dir entry management
- [x] mkrfs integration — `build.rs` runs `mkrfs` to produce `disk.img` automatically
- [x] VFS layer: SYS_OPEN, SYS_READ, SYS_WRITE, SYS_CLOSE, SYS_STAT, SYS_READDIR, SYS_CREATE, SYS_UNLINK (SYS 22–29)
- [x] lysh `exec <path>` — load and run an ELF off the filesystem
- [x] lysh `ls`, `cat`, `cp`, `rm`

## Networking

- [ ] VirtIO network device driver (virtio-net)
- [ ] Ethernet + ARP
- [ ] IP + ICMP (ping)
- [ ] TCP stack
- [ ] SYS_SOCKET / SYS_BIND / SYS_CONNECT / SYS_SEND / SYS_RECV

## Kernel reliability

- [x] IOAPIC driver (replace 8259 PIC — needed for VirtIO PCI interrupts)
- [ ] Multi-processor support (AP startup, per-CPU scheduler)
- [ ] Larger default kernel stack (current 16 KiB can be tight)
- [ ] Kernel ASLR
- [ ] SYS_MMAP range enforcement — currently no bounds check on virt arg; `syscalls.md` flags this as a planned improvement
- [ ] Per-process PML4 — all tasks share the kernel PML4; needed for real isolation (ELF loader limitation, `docs/elf.md`)
- [ ] ELF ASLR — randomise PT_LOAD base address per exec; depends on per-process PML4
- [ ] Reclaim lythd module frames — PMM reserves 512 KiB at `0x400000` forever; free after the ELF is copied to heap
- [ ] VirtIO interrupt-driven completion — replace polled spin on `used_ring.idx`; IRQ line already read at init, just unused
- [ ] IPC timeout / cancellation — tasks that block on empty/full ring wait forever; add a deadline or cancel token (`docs/ipc.md`)
- [x] ELF user-facing error reporting — `exec()` panics on malformed ELF; surface a proper error code instead

## lythd / userspace

- [ ] lythdist service manifest format — line-based text, stored as `/etc/svc/<name>.svc` on RFS
  - Fields: `name=`, `path=`, `restart=` (never|on-failure[:N]|always), `cap=` (memory|rollback|ipc:<rights>), `dep=`
  - lythd reads `/etc/svc/` at boot, parses manifests, toposorts by deps, spawns in order
  - `cap=ipc` → lythd creates fresh endpoint and passes handle; `cap=memory:<rights>` → sys_cap_grant; `cap=rollback` → grant rollback cap
  - Replaces hardcoded `managed` array in lythd with manifest-driven `Vec<ManagedSvc>`
- [ ] lythd: spawn lythdist and lysh automatically after BootInfo recv (currently manual in test ELFs)

## Text editor (rkilo)

- [ ] `rkilo [path]` — kilo-style screen editor ported to OROS (no termios, no POSIX)
  - Input via `SYS_SERIAL_READ` (14) — already raw, no `tcsetattr` needed
  - Output via `SYS_LOG` / `SYS_WRITE`; ANSI VT100 escapes work through QEMU `-serial stdio`
  - Terminal size: ANSI CPR trick (`\x1b[999C\x1b[999B\x1b[6n`) or fallback hardcode 80×24
  - File I/O via VFS: `SYS_OPEN`/`SYS_READ`/`SYS_WRITE`/`SYS_CREATE`/`SYS_CLOSE`
  - Key bindings: `Ctrl-S` save, `Ctrl-Q` quit, `Ctrl-F` find, arrow keys / PgUp / PgDn
  - No syntax highlighting required for v1
  - Primary use case: editing `/etc/svc/*.svc` manifests on a live system

## Display / GUI

- [ ] VGA text-mode fallback (80×25)
- [ ] Framebuffer driver (VESA / Multiboot framebuffer tag)
- [ ] Basic window manager (webwm is already in OROS, needs a framebuffer)
