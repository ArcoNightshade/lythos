---
type: community
cohesion: 0.05
members: 61
---

# APIC & Interrupt Management

**Cohesion:** 0.05 - loosely connected
**Members:** 61 nodes

## Members
- [[BootInfo message]] - document - CLAUDE.md
- [[BootInfo message (lythd boot protocol)]] - document - docs/lythd.md
- [[CapKind (Memory, Ipc, Rollback)]] - document - CLAUDE.md
- [[CapRights bitflags]] - document - CLAUDE.md
- [[Capability System]] - document - CLAUDE.md
- [[ELF Loader]] - document - CLAUDE.md
- [[ELF Loader Limitations (no ASLR, no PIE, shared PML4)]] - document - docs/elf.md
- [[IPC Blocking Model (per-endpoint wait queue)]] - document - docs/ipc.md
- [[IPC BootInfo pre-queued message]] - document - docs/ipc.md
- [[IPC Endpoints]] - document - CLAUDE.md
- [[IPC Ring Buffer (4 KiB page)]] - document - docs/ipc.md
- [[IPC_RECEIVER_ELF]] - code - src/elf.rs
- [[IPC_SENDER_ELF]] - code - src/elf.rs
- [[Initial Boot Capabilities (mem, rollback, boot IPC)]] - document - docs/capabilities.md
- [[Kernel Heap]] - document - CLAUDE.md
- [[LYTHD_ELF embedded blob]] - document - docs/elf.md
- [[Lythos Microkernel]] - document - CLAUDE.md
- [[OROS userspace repository]] - document - docs/lythd.md
- [[PT_LOAD segment loading]] - document - docs/elf.md
- [[Physical Memory Manager (PMM)]] - document - CLAUDE.md
- [[SMOKE_ELF (hand-crafted test binary)]] - code - src/elf.rs
- [[SYS_EXEC (10)]] - document - docs/syscalls.md
- [[SYS_IPC_CREATE (8)]] - document - docs/syscalls.md
- [[SYS_IPC_RECV (7)]] - document - docs/syscalls.md
- [[SYS_IPC_SEND (6)]] - document - docs/syscalls.md
- [[SYS_TASK_EXIT (1)]] - document - docs/syscalls.md
- [[SYS_YIELD (0)]] - document - docs/syscalls.md
- [[Syscall Dispatch]] - document - CLAUDE.md
- [[Syscall Entry (LSTARFMASKsysretq)]] - document - docs/syscalls.md
- [[Task Scheduler]] - document - CLAUDE.md
- [[Virtual Memory Manager (VMM)]] - document - CLAUDE.md
- [[alloc_user_stack]] - document - docs/elf.md
- [[apiceoi]] - code - src/apic.rs
- [[block_and_yield]] - code - src/task.rs
- [[cap_inherit]] - code - src/cap.rs
- [[current_task_id]] - code - src/task.rs
- [[elfexec (ELF64 loader and spawner)]] - code - src/elf.rs
- [[exec_trampoline]] - code - src/elf.rs
- [[free_frame]] - code - src/pmm.rs
- [[ipcrecv]] - code - src/ipc.rs
- [[ipcrecv_cap]] - code - src/ipc.rs
- [[ipcsend]] - code - src/ipc.rs
- [[ipcsend_cap]] - code - src/ipc.rs
- [[kmain (boot entry point)]] - document - CLAUDE.md
- [[lysh interactive shell]] - document - docs/lythd.md
- [[lythd Boot Protocol]] - document - CLAUDE.md
- [[lythd Capability Delegation Policy]] - document - docs/lythd.md
- [[lythd init process]] - document - docs/lythd.md
- [[lythdist service manager]] - document - docs/lythd.md
- [[lythos-std syscall wrapper library]] - document - docs/lythd.md
- [[send_tlb_shootdown_ipi]] - code - src/apic.rs
- [[set_rsp0]] - code - src/tss.rs
- [[spawn_userspace_task]] - code - src/task.rs
- [[sweep_dead]] - code - src/task.rs
- [[switch_cr3 (CR3 + TSSRSP update)]] - code - src/task.rs
- [[task_exit]] - code - src/task.rs
- [[timer_interrupt_handler]] - code - src/apic.rs
- [[tlb_shootdown_handler]] - code - src/apic.rs
- [[wake_task]] - code - src/task.rs
- [[write_initial_stack_frame]] - code - src/elf.rs
- [[yield_task]] - code - src/task.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/APIC_&_Interrupt_Management
SORT file.name ASC
```

## Connections to other communities
- 4 edges to [[_COMMUNITY_Kernel Initialization]]
- 2 edges to [[_COMMUNITY_Capability Docs]]
- 1 edge to [[_COMMUNITY_Architecture Docs]]

## Top bridge nodes
- [[elfexec (ELF64 loader and spawner)]] - degree 18, connects to 1 community
- [[Lythos Microkernel]] - degree 9, connects to 1 community
- [[Capability System]] - degree 8, connects to 1 community
- [[ipcsend]] - degree 7, connects to 1 community
- [[Virtual Memory Manager (VMM)]] - degree 6, connects to 1 community