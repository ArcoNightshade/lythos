Stuck on RFS reading superblock.

Issue:

- Polls status_pa (hdr_phys + 16) — the byte QEMU writes to directly
- Doesn't depend on used-ring address math
- If it still stalls, diagnostic shows status=0xff meaning QEMU never touched the DMA buffer at all, which would point to bad descriptor addresses or a PFN issue

If the diagnostic fires with status=0xff, the next thing to check would be the hdr_phys and dat_phys addresses — specifically whether they're in the range QEMU sees as guest RAM. Run and report what you get.

OUTPUT FROM run.sh
[run.sh] building OROS userspace...  
 [run.sh] building lythos kernel + disk image...  
 [run.sh] launching QEMU...  
 qemu-system-x86_64: -chardev socket,id=s0,path=/tmp/lythos-serial-91816.sock,server=on,wait=on: info: QEMU waiting for connection on:  
 disconnected:unix:/tmp/lythos-serial-91816.sock,server=on  
 [run.sh] connected — Ctrl+C to quit  
 lythos kernel initializing...  
 [gdt] loaded  
 [idt] loaded - exceptions active  
 [pmm] initialized — 32359 free frames (126 MiB)  
 [pmm] smoke-test passed  
 [vmm] paging active — identity 0–4MiB, higher-half kernel mapped  
 [vmm] smoke-test passed  
 [heap] initialized — 4096 KiB pre-mapped at 0xffffc00000000000  
 [heap] smoke-test passed  
 [sched] initialized  
 [task A] tick 0, yielding...  
 [task B] tick 0, yielding...  
 [task A] tick 1, yielding...  
 [task B] tick 1, yielding...  
 [task A] tick 2, yielding...  
 [task B] tick 2, yielding...  
 [sched] smoke-test passed  
 [apic] timer active — preemptive scheduling enabled  
 [ioapic] initialized — 24 GSIs, all masked  
 [task B] done, exiting  
 [virtio-blk] device ready — 131072 sectors (64 MiB)  
 [rfs] reading superblock...  
 [virtio-blk] poll stall: status=0xff used.idx=0x0 want=0x1 status_pa=0x588010 vq=0x586000 q=128  
 [rfs] no RFS_V1 image on disk (pass -drive file=disk.img,... to QEMU)  
 [apic] smoke-test passed — 50 ticks elapsed  
 [syscall] initialized — LSTAR/STAR/FMASK configured  
 [cap] smoke-test passed  
 [cap] cascade-revoke smoke-test passed  
 [ipc] sent message 1  
 [ipc] sent message 2  
 [ipc] sent message 3  
 [ipc] sender done  
 [ipc] receiver done  
 [ipc] smoke-test passed  
 [syscall] userspace entry smoke-test passed  
 [elf] smoke-test passed  
 [integration] ENOCAP check passed  
 [integration] IPC userspace send/recv passed  
 [integration] task_exit + scheduler reap verified  
 [integration] cap grant/revoke syscall passed  
 [integration] SYS_MMAP/SYS_MUNMAP lifecycle passed  
 [integration] IPC_SEND_CAP/IPC_RECV_CAP passed  
 [integration] triangular IPC passed  
 [integration] SYS_EXEC from userspace passed  
 [PANIC] panicked at src/main.rs:729:13:  
 assertion `left == right` failed: fuzz: nr=22 expected ENOSYS  
 left: 18446744073709551612  
 right: 18446744073709551615

Diagnostic:
⏺ Two separate issues in that output.

1. status=0xff — QEMU never touched the DMA buffer. Need to see the actual vmm identity map range.
2. Panic at main.rs:729 — fuzz test expects SYS_OPEN (22) to return ENOSYS but now it returns EINVAL (since we implemented it). Need to fix the fuzz test.
