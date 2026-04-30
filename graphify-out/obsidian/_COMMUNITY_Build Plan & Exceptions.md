---
type: community
cohesion: 0.08
members: 33
---

# Build Plan & Exceptions

**Cohesion:** 0.08 - loosely connected
**Members:** 33 nodes

## Members
- [[14-Step Build Plan]] - document - lythos_kernel_build_plan.md
- [[30-second StabilityRollback Timer]] - document - OROS_Design_Document.md
- [[Btrfs Subvolume Layout (@core, @store, @cfg, @home)]] - document - OROS_Design_Document.md
- [[Error Codes (ENOSYS, ENOCAP, ENOPERM, EINVAL)]] - code - src/syscall.rs
- [[ExceptionFrame_1]] - code - src/exceptions.rs
- [[Filesystem TODO (FAT32VFSlysh exec)]] - document - TODO.md
- [[Lythos Project (README)]] - document - README.md
- [[Networking TODO (virtio-netTCP)]] - document - TODO.md
- [[OROS (Open Runtime Operating System)]] - document - OROS_Design_Document.md
- [[OROS Boot Sequence]] - document - OROS_Design_Document.md
- [[SERIAL (global SpinLockSerialPort)]] - code - src/serial.rs
- [[SMAP_ENABLED (AtomicBool)]] - code - src/syscall.rs
- [[SYS_CAP_GRANT (4)]] - document - docs/syscalls.md
- [[Syscall Numbers (SYS_YIELD..SYS_BLK_WRITE)]] - code - src/syscall.rs
- [[SyscallFrame_1]] - code - src/syscall.rs
- [[exception_handler]] - code - src/exceptions.rs
- [[kprint! macro]] - code - src/serial.rs
- [[kprintln! macro]] - code - src/serial.rs
- [[lysh (system shell)]] - document - OROS_Design_Document.md
- [[lythd (PID 1 supervisor)]] - document - OROS_Design_Document.md
- [[lythdist (capability distributor)]] - document - OROS_Design_Document.md
- [[lythmsg (IPC bus)]] - document - OROS_Design_Document.md
- [[lythos-linux-compat (Linux compat server)]] - document - OROS_Design_Document.md
- [[lythos-std (native runtime)]] - document - OROS_Design_Document.md
- [[page_fault_handler]] - code - src/exceptions.rs
- [[rpbreak (chaos engineering tool)]] - document - OROS_Design_Document.md
- [[rpkg (package manager)]] - document - OROS_Design_Document.md
- [[rpview (service TUI)]] - document - OROS_Design_Document.md
- [[syscallinit]] - code - src/syscall.rs
- [[syscall_dispatch]] - code - src/syscall.rs
- [[syscall_entry (asm stub)]] - code - src/syscall.rs
- [[valid_user_range]] - code - src/syscall.rs
- [[with_user_access (SMAP window)]] - code - src/syscall.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Build_Plan_&_Exceptions
SORT file.name ASC
```

## Connections to other communities
- 2 edges to [[_COMMUNITY_Documentation Index]]
- 1 edge to [[_COMMUNITY_Capability Docs]]

## Top bridge nodes
- [[syscall_dispatch]] - degree 7, connects to 1 community
- [[syscallinit]] - degree 3, connects to 1 community
- [[SYS_CAP_GRANT (4)]] - degree 2, connects to 1 community