---
type: community
cohesion: 0.06
members: 36
---

# Documentation Index

**Cohesion:** 0.06 - loosely connected
**Members:** 36 nodes

## Members
- [[Context Switch (docs)]] - document - docs/tasks.md
- [[Heap Documentation]] - document - docs/memory.md
- [[Kernel Boot Sequence (docsboot.md)]] - document - docs/boot.md
- [[Lythos Internal Docs Index]] - document - docs/README.md
- [[PMM Documentation]] - document - docs/memory.md
- [[PageFlags_1]] - code - src/vmm.rs
- [[PageTable (512 entries)]] - code - src/vmm.rs
- [[PageTableEntry_1]] - code - src/vmm.rs
- [[Round-Robin Scheduler (docs)]] - document - docs/tasks.md
- [[SYSCALL_KERN_RSP]] - code - src/syscall.rs
- [[SerialPort_1]] - code - src/serial.rs
- [[SerialPorttry_read_byte]] - code - src/serial.rs
- [[SerialPortwrite_byte]] - code - src/serial.rs
- [[SpinLockT_1]] - code - src/serial.rs
- [[SpinLockGuardT]] - code - src/serial.rs
- [[Task Model (docs)]] - document - docs/tasks.md
- [[User Stack Layout (docs)]] - document - docs/tasks.md
- [[VMM Documentation]] - document - docs/memory.md
- [[VirtAddr_1]] - code - src/vmm.rs
- [[enter_userspace]] - code - src/syscall.rs
- [[enter_userspace_asm (asm)]] - code - src/syscall.rs
- [[inb (IO port read)]] - code - src/serial.rs
- [[outb (IO port write)]] - code - src/serial.rs
- [[serialinit]] - code - src/serial.rs
- [[vmmcreate_user_page_table]] - code - src/vmm.rs
- [[vmmfree_user_page_table]] - code - src/vmm.rs
- [[vmminit]] - code - src/vmm.rs
- [[vmmmap_page]] - code - src/vmm.rs
- [[vmmmap_page_in]] - code - src/vmm.rs
- [[vmmquery_page]] - code - src/vmm.rs
- [[vmmquery_page_in]] - code - src/vmm.rs
- [[vmmunmap_page]] - code - src/vmm.rs
- [[vmmunmap_page_in]] - code - src/vmm.rs
- [[vmmupdate_page_flags]] - code - src/vmm.rs
- [[walk_existing]] - code - src/vmm.rs
- [[walk_or_create]] - code - src/vmm.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Documentation_Index
SORT file.name ASC
```

## Connections to other communities
- 2 edges to [[_COMMUNITY_Build Plan & Exceptions]]

## Top bridge nodes
- [[Kernel Boot Sequence (docsboot.md)]] - degree 4, connects to 1 community
- [[SerialPorttry_read_byte]] - degree 2, connects to 1 community