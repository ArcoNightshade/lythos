# Graph Report - .  (2026-04-30)

## Corpus Check
- Corpus is ~46,630 words - fits in a single context window. You may not need a graph.

## Summary
- 478 nodes · 903 edges · 49 communities detected
- Extraction: 79% EXTRACTED · 21% INFERRED · 0% AMBIGUOUS · INFERRED: 191 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Kernel Core Infrastructure|Kernel Core Infrastructure]]
- [[_COMMUNITY_Interrupt & TLB Management|Interrupt & TLB Management]]
- [[_COMMUNITY_ELF Loader & Userspace|ELF Loader & Userspace]]
- [[_COMMUNITY_Capability Operations|Capability Operations]]
- [[_COMMUNITY_Architecture Documentation|Architecture Documentation]]
- [[_COMMUNITY_Physical Memory Manager|Physical Memory Manager]]
- [[_COMMUNITY_RFS Filesystem Allocator|RFS Filesystem Allocator]]
- [[_COMMUNITY_Build Plan & Design Docs|Build Plan & Design Docs]]
- [[_COMMUNITY_Interrupt Descriptor Table|Interrupt Descriptor Table]]
- [[_COMMUNITY_GDT & TSS Setup|GDT & TSS Setup]]
- [[_COMMUNITY_Capability Type Definitions|Capability Type Definitions]]
- [[_COMMUNITY_APIC MMIO Interface|APIC MMIO Interface]]
- [[_COMMUNITY_IO APIC|I/O APIC]]
- [[_COMMUNITY_Kernel Heap Allocator|Kernel Heap Allocator]]
- [[_COMMUNITY_PCI Bus Access|PCI Bus Access]]
- [[_COMMUNITY_SpinLock Primitives|SpinLock Primitives]]
- [[_COMMUNITY_VirtIO Block Driver|VirtIO Block Driver]]
- [[_COMMUNITY_RFS Filesystem Format|RFS Filesystem Format]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 45|Community 45]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]
- [[_COMMUNITY_Community 48|Community 48]]

## God Nodes (most connected - your core abstractions)
1. `syscall_dispatch()` - 44 edges
2. `kmain()` - 27 edges
3. `VirtAddr` - 21 edges
4. `get_sched()` - 20 edges
5. `PhysAddr` - 19 edges
6. `kmain (kernel entry point)` - 17 edges
7. `core_smoke()` - 14 edges
8. `map_page()` - 13 edges
9. `main()` - 12 edges
10. `exec()` - 12 edges

## Surprising Connections (you probably didn't know these)
- `Allocator (block bitmap allocator)` --semantically_similar_to--> `alloc_frame`  [INFERRED] [semantically similar]
  tools/mkrfs/src/main.rs → src/pmm.rs
- `InodeTable (RFS inode table)` --semantically_similar_to--> `CapabilityTable (per-task cap table)`  [INFERRED] [semantically similar]
  tools/mkrfs/src/main.rs → src/cap.rs
- `RingBuffer (4 KiB shared page layout)` --semantically_similar_to--> `Allocator (block bitmap allocator)`  [INFERRED] [semantically similar]
  src/ipc.rs → tools/mkrfs/src/main.rs
- `SpinLock<T>` --references--> `Critical Invariants (BSS zero, U/S bit, huge page guard)`  [INFERRED]
  src/serial.rs → docs/architecture.md
- `Kernel Boot Sequence (docs/boot.md)` --references--> `serial::init`  [EXTRACTED]
  docs/boot.md → src/serial.rs

## Hyperedges (group relationships)
- **Kernel Boot Initialization Sequence** — main_kmain, gdt_init, idt_init, pmm_init, heap_init, task_init, apic_init, ioapic_init [EXTRACTED 0.97]
- **IPC Blocking/Waking Protocol** — ipc_send, ipc_recv, task_block_and_yield, task_wake_task, ipc_ipcendpoint [EXTRACTED 0.95]
- **ELF Exec and Ring-3 Entry Pipeline** — elf_exec, elf_load_segment_into, elf_alloc_user_stack_into, task_spawn_userspace_task, elf_exec_trampoline [EXTRACTED 0.95]
- **syscall_dispatch integrates task, cap, ipc, vmm, pmm subsystems** — syscall_syscall_dispatch, vmm_map_page, vmm_unmap_page, syscall_valid_user_range, syscall_with_user_access [EXTRACTED 0.95]
- **OROS Critical Daemon Triad (lythd + lythdist + lythmsg) triggers rollback** — oros_doc_lythd, oros_doc_lythdist, oros_doc_lythmsg, oros_doc_rollback_timer [EXTRACTED 1.00]
- **4-level page walk chain: walk_or_create/walk_existing operate on PML4/PDPT/PD/PT** — vmm_walk_or_create, vmm_walk_existing, vmm_pagetableentry, vmm_pagetable [EXTRACTED 0.95]

## Communities

### Community 0 - "Kernel Core Infrastructure"
Cohesion: 0.09
Nodes (52): ticks(), create_object(), exception_handler(), ExceptionFrame, page_fault_handler(), create_endpoint(), endpoint_phys(), ep_table() (+44 more)

### Community 1 - "Interrupt & TLB Management"
Cohesion: 0.05
Nodes (50): apic::eoi, apic::init, send_tlb_shootdown_ipi, timer_interrupt_handler, tlb_shootdown_handler, cap_inherit, create_object, create_root_cap (+42 more)

### Community 2 - "ELF Loader & Userspace"
Cohesion: 0.1
Nodes (33): alloc_user_stack_into(), Elf64Ehdr, Elf64Phdr, ElfError, exec(), load_segment_into(), read_ehdr(), read_phdr() (+25 more)

### Community 3 - "Capability Operations"
Cohesion: 0.1
Nodes (22): alloc_cap_id(), cap_cascade_revoke(), cap_grant(), cap_inherit(), cap_revoke(), Capability, CapabilityTable, CapError (+14 more)

### Community 4 - "Architecture Documentation"
Cohesion: 0.06
Nodes (41): Architecture Component Map, Critical Invariants (BSS zero, U/S bit, huge page guard), Kernel Boot Sequence (docs/boot.md), Heap Documentation, PMM Documentation, VMM Documentation, Lythos Internal Docs Index, Context Switch (docs) (+33 more)

### Community 5 - "Physical Memory Manager"
Cohesion: 0.12
Nodes (31): alloc_frames_contiguous(), free_frame(), free_frames_contiguous(), init(), is_used(), mark_range_free(), mark_range_used(), parse_mb1() (+23 more)

### Community 6 - "RFS Filesystem Allocator"
Cohesion: 0.17
Nodes (21): Allocator, attach_extent(), count_free(), create_file(), create_symlink(), Disk, find_last_entry_off(), get_u32() (+13 more)

### Community 7 - "Build Plan & Design Docs"
Cohesion: 0.09
Nodes (29): 14-Step Build Plan, exception_handler, ExceptionFrame, page_fault_handler, OROS Boot Sequence, Btrfs Subvolume Layout (@core, @store, @cfg, @home), lythos-linux-compat (Linux compat server), lysh (system shell) (+21 more)

### Community 8 - "Interrupt Descriptor Table"
Cohesion: 0.13
Nodes (12): IdtEntry, IdtPtr, init(), register_irq(), remap_and_mask_pic(), inb(), init(), outb() (+4 more)

### Community 9 - "GDT & TSS Setup"
Cohesion: 0.11
Nodes (17): exec_trampoline(), encode_tss_descriptor(), GdtPtr, GlobalGdt, init(), enter_userspace(), init(), rdmsr() (+9 more)

### Community 10 - "Capability Type Definitions"
Cohesion: 0.11
Nodes (18): Capability (kernel capability struct), CapabilityTable (per-task cap table), CapKind (enum: Memory/Ipc/Device/Rollback), CapRights (bitfield), KernelObject (enum: Memory/Ipc/Device/Rollback), KernelObjectRef (generation-tagged index), IpcEndpoint (kernel IPC state), RingBuffer (4 KiB shared page layout) (+10 more)

### Community 11 - "APIC MMIO Interface"
Cohesion: 0.32
Nodes (12): apic_read(), apic_write(), disable_pic(), eoi(), inb(), init(), outb(), pit_wait_ms() (+4 more)

### Community 12 - "I/O APIC"
Cohesion: 0.44
Nodes (7): init(), ioapic_read(), ioapic_write(), map_irq(), mask_irq(), redir_lo_reg(), unmask_irq()

### Community 13 - "Kernel Heap Allocator"
Cohesion: 0.32
Nodes (3): align_up(), FreeBlock, KernelAllocator

### Community 14 - "PCI Bus Access"
Cohesion: 0.5
Nodes (7): cfg_read32(), cfg_write32(), config_addr(), find_device(), inl(), outl(), PciDevice

### Community 15 - "SpinLock Primitives"
Cohesion: 0.5
Nodes (1): SpinLockGuard<'_, T>

### Community 16 - "VirtIO Block Driver"
Cohesion: 0.67
Nodes (3): virtio_blk::read_sector, VirtioBlkDev::submit, virtio_blk::write_sector

### Community 17 - "RFS Filesystem Format"
Cohesion: 1.0
Nodes (2): Inode (RFS inode struct), RFS_V1 filesystem format

### Community 18 - "Community 18"
Cohesion: 1.0
Nodes (1): PhysAddr (physical address newtype)

### Community 19 - "Community 19"
Cohesion: 1.0
Nodes (1): free_frames_contiguous

### Community 20 - "Community 20"
Cohesion: 1.0
Nodes (1): KernelAllocator (global heap allocator)

### Community 21 - "Community 21"
Cohesion: 1.0
Nodes (1): Tss (task state segment struct)

### Community 22 - "Community 22"
Cohesion: 1.0
Nodes (1): IdtEntry (IDT gate descriptor)

### Community 23 - "Community 23"
Cohesion: 1.0
Nodes (1): apic::ticks

### Community 24 - "Community 24"
Cohesion: 1.0
Nodes (1): ioapic::mask_irq

### Community 25 - "Community 25"
Cohesion: 1.0
Nodes (1): TaskState (enum)

### Community 26 - "Community 26"
Cohesion: 1.0
Nodes (1): TaskContext (saved rsp)

### Community 27 - "Community 27"
Cohesion: 1.0
Nodes (1): Scheduler (round-robin scheduler)

### Community 28 - "Community 28"
Cohesion: 1.0
Nodes (1): block_task

### Community 29 - "Community 29"
Cohesion: 1.0
Nodes (1): cap_table_ptr

### Community 30 - "Community 30"
Cohesion: 1.0
Nodes (1): kill_task

### Community 31 - "Community 31"
Cohesion: 1.0
Nodes (1): vma_insert

### Community 32 - "Community 32"
Cohesion: 1.0
Nodes (1): vma_remove

### Community 33 - "Community 33"
Cohesion: 1.0
Nodes (1): task_exists

### Community 34 - "Community 34"
Cohesion: 1.0
Nodes (1): CapHandle (opaque per-task handle)

### Community 35 - "Community 35"
Cohesion: 1.0
Nodes (1): cap_grant

### Community 36 - "Community 36"
Cohesion: 1.0
Nodes (1): cap_revoke

### Community 37 - "Community 37"
Cohesion: 1.0
Nodes (1): cap_cascade_revoke

### Community 38 - "Community 38"
Cohesion: 1.0
Nodes (1): SMOKE_ELF (hand-crafted test binary)

### Community 39 - "Community 39"
Cohesion: 1.0
Nodes (1): IPC_SENDER_ELF

### Community 40 - "Community 40"
Cohesion: 1.0
Nodes (1): IPC_RECEIVER_ELF

### Community 41 - "Community 41"
Cohesion: 1.0
Nodes (1): MMAP_TEST_ELF

### Community 42 - "Community 42"
Cohesion: 1.0
Nodes (1): EXEC_FROM_USER_ELF

### Community 43 - "Community 43"
Cohesion: 1.0
Nodes (1): PciDevice (PCI device descriptor)

### Community 44 - "Community 44"
Cohesion: 1.0
Nodes (1): VirtioBlkDev (driver state)

### Community 45 - "Community 45"
Cohesion: 1.0
Nodes (1): SYSCALL_USER_RSP

### Community 46 - "Community 46"
Cohesion: 1.0
Nodes (1): vmm::kernel_pml4

### Community 47 - "Community 47"
Cohesion: 1.0
Nodes (1): Multi-processor SMP TODO

### Community 48 - "Community 48"
Cohesion: 1.0
Nodes (1): Display/GUI TODO (framebuffer/webwm)

## Knowledge Gaps
- **119 isolated node(s):** `GlobalTss`, `RingBuffer`, `EpTable`, `CapKind`, `KernelObject` (+114 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `SpinLock Primitives`** (4 nodes): `SpinLockGuard<'_, T>`, `.deref()`, `.deref_mut()`, `.drop()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `RFS Filesystem Format`** (2 nodes): `Inode (RFS inode struct)`, `RFS_V1 filesystem format`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 18`** (1 nodes): `PhysAddr (physical address newtype)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 19`** (1 nodes): `free_frames_contiguous`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 20`** (1 nodes): `KernelAllocator (global heap allocator)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 21`** (1 nodes): `Tss (task state segment struct)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 22`** (1 nodes): `IdtEntry (IDT gate descriptor)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 23`** (1 nodes): `apic::ticks`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 24`** (1 nodes): `ioapic::mask_irq`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 25`** (1 nodes): `TaskState (enum)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 26`** (1 nodes): `TaskContext (saved rsp)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 27`** (1 nodes): `Scheduler (round-robin scheduler)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 28`** (1 nodes): `block_task`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 29`** (1 nodes): `cap_table_ptr`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 30`** (1 nodes): `kill_task`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 31`** (1 nodes): `vma_insert`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 32`** (1 nodes): `vma_remove`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 33`** (1 nodes): `task_exists`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 34`** (1 nodes): `CapHandle (opaque per-task handle)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 35`** (1 nodes): `cap_grant`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 36`** (1 nodes): `cap_revoke`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 37`** (1 nodes): `cap_cascade_revoke`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 38`** (1 nodes): `SMOKE_ELF (hand-crafted test binary)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 39`** (1 nodes): `IPC_SENDER_ELF`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 40`** (1 nodes): `IPC_RECEIVER_ELF`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 41`** (1 nodes): `MMAP_TEST_ELF`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 42`** (1 nodes): `EXEC_FROM_USER_ELF`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 43`** (1 nodes): `PciDevice (PCI device descriptor)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 44`** (1 nodes): `VirtioBlkDev (driver state)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 45`** (1 nodes): `SYSCALL_USER_RSP`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 46`** (1 nodes): `vmm::kernel_pml4`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 47`** (1 nodes): `Multi-processor SMP TODO`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 48`** (1 nodes): `Display/GUI TODO (framebuffer/webwm)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `syscall_dispatch()` connect `Kernel Core Infrastructure` to `ELF Loader & Userspace`, `Capability Operations`, `Physical Memory Manager`, `GDT & TSS Setup`, `APIC MMIO Interface`?**
  _High betweenness centrality (0.121) - this node is a cross-community bridge._
- **Why does `kmain()` connect `Kernel Core Infrastructure` to `ELF Loader & Userspace`, `Capability Operations`, `Physical Memory Manager`, `SpinLock Primitives`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `PhysAddr` connect `ELF Loader & Userspace` to `Kernel Core Infrastructure`, `APIC MMIO Interface`, `I/O APIC`, `Physical Memory Manager`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Are the 41 inferred relationships involving `syscall_dispatch()` (e.g. with `core_smoke()` and `yield_task()`) actually correct?**
  _`syscall_dispatch()` has 41 INFERRED edges - model-reasoned connections that need verification._
- **Are the 24 inferred relationships involving `kmain()` (e.g. with `init()` and `PhysAddr`) actually correct?**
  _`kmain()` has 24 INFERRED edges - model-reasoned connections that need verification._
- **Are the 14 inferred relationships involving `VirtAddr` (e.g. with `create_endpoint()` and `init()`) actually correct?**
  _`VirtAddr` has 14 INFERRED edges - model-reasoned connections that need verification._
- **Are the 14 inferred relationships involving `PhysAddr` (e.g. with `init()` and `init()`) actually correct?**
  _`PhysAddr` has 14 INFERRED edges - model-reasoned connections that need verification._