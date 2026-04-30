# Graph Report - .  (2026-04-30)

## Corpus Check
- 36 files · ~53,180 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 553 nodes · 1002 edges · 58 communities detected
- Extraction: 80% EXTRACTED · 20% INFERRED · 0% AMBIGUOUS · INFERRED: 203 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Kernel Core Subsystems|Kernel Core Subsystems]]
- [[_COMMUNITY_APIC & Interrupt Management|APIC & Interrupt Management]]
- [[_COMMUNITY_ELF Loader & Process Launch|ELF Loader & Process Launch]]
- [[_COMMUNITY_Capability System (Code)|Capability System (Code)]]
- [[_COMMUNITY_Kernel Initialization|Kernel Initialization]]
- [[_COMMUNITY_Documentation Index|Documentation Index]]
- [[_COMMUNITY_Physical Memory Manager|Physical Memory Manager]]
- [[_COMMUNITY_RFS Allocator & Main|RFS Allocator & Main]]
- [[_COMMUNITY_Build Plan & Exceptions|Build Plan & Exceptions]]
- [[_COMMUNITY_Architecture Docs|Architecture Docs]]
- [[_COMMUNITY_IDT & Interrupt Descriptors|IDT & Interrupt Descriptors]]
- [[_COMMUNITY_Capability Docs|Capability Docs]]
- [[_COMMUNITY_APIC Hardware Interface|APIC Hardware Interface]]
- [[_COMMUNITY_GDT & Segment Setup|GDT & Segment Setup]]
- [[_COMMUNITY_IOAPIC Driver|IOAPIC Driver]]
- [[_COMMUNITY_Kernel Heap Allocator|Kernel Heap Allocator]]
- [[_COMMUNITY_PCI Config Scanner|PCI Config Scanner]]
- [[_COMMUNITY_Syscall Entry & Dispatch|Syscall Entry & Dispatch]]
- [[_COMMUNITY_SpinLock Guard|SpinLock Guard]]
- [[_COMMUNITY_VirtIO Block IO|VirtIO Block I/O]]
- [[_COMMUNITY_RFS Host Tool|RFS Host Tool]]
- [[_COMMUNITY_TSS|TSS]]
- [[_COMMUNITY_IPC Ring Buffer|IPC Ring Buffer]]
- [[_COMMUNITY_Cascade Revoke|Cascade Revoke]]
- [[_COMMUNITY_VirtIO Block Device|VirtIO Block Device]]
- [[_COMMUNITY_Heap Allocator|Heap Allocator]]
- [[_COMMUNITY_PCI Device|PCI Device]]
- [[_COMMUNITY_IDT Entry|IDT Entry]]
- [[_COMMUNITY_IOAPIC IRQ Mask|IOAPIC IRQ Mask]]
- [[_COMMUNITY_APIC Tick Counter|APIC Tick Counter]]
- [[_COMMUNITY_Task State|Task State]]
- [[_COMMUNITY_Task Context|Task Context]]
- [[_COMMUNITY_Task Scheduler|Task Scheduler]]
- [[_COMMUNITY_Task Block|Task Block]]
- [[_COMMUNITY_Task Cap Table|Task Cap Table]]
- [[_COMMUNITY_Task Kill|Task Kill]]
- [[_COMMUNITY_VMA Insert|VMA Insert]]
- [[_COMMUNITY_VMA Remove|VMA Remove]]
- [[_COMMUNITY_Task Exists|Task Exists]]
- [[_COMMUNITY_Physical Address|Physical Address]]
- [[_COMMUNITY_PMM Contiguous Free|PMM Contiguous Free]]
- [[_COMMUNITY_ELF MMAP Test|ELF MMAP Test]]
- [[_COMMUNITY_Exec From Userspace|Exec From Userspace]]
- [[_COMMUNITY_Syscall User RSP|Syscall User RSP]]
- [[_COMMUNITY_VMM Kernel PML4|VMM Kernel PML4]]
- [[_COMMUNITY_SMP TODO|SMP TODO]]
- [[_COMMUNITY_Display TODO|Display TODO]]
- [[_COMMUNITY_GDT Doc|GDT Doc]]
- [[_COMMUNITY_TSS Doc|TSS Doc]]
- [[_COMMUNITY_IDT Doc|IDT Doc]]
- [[_COMMUNITY_Serial Doc|Serial Doc]]
- [[_COMMUNITY_Memory Layout Doc|Memory Layout Doc]]
- [[_COMMUNITY_Syscall ABI Doc|Syscall ABI Doc]]
- [[_COMMUNITY_Capability Security Model|Capability Security Model]]
- [[_COMMUNITY_SYS_MUNMAP|SYS_MUNMAP]]
- [[_COMMUNITY_SYS_ROLLBACK|SYS_ROLLBACK]]
- [[_COMMUNITY_mkrfs Tool|mkrfs Tool]]
- [[_COMMUNITY_IPC Limitations|IPC Limitations]]

## God Nodes (most connected - your core abstractions)
1. `syscall_dispatch()` - 44 edges
2. `kmain()` - 27 edges
3. `VirtAddr` - 21 edges
4. `get_sched()` - 20 edges
5. `PhysAddr` - 19 edges
6. `elf::exec (ELF64 loader and spawner)` - 18 edges
7. `kmain (kernel entry point)` - 17 edges
8. `core_smoke()` - 14 edges
9. `map_page()` - 13 edges
10. `main()` - 12 edges

## Surprising Connections (you probably didn't know these)
- `ipc::send` --semantically_similar_to--> `SYS_IPC_SEND (6)`  [INFERRED] [semantically similar]
  src/ipc.rs → docs/syscalls.md
- `ipc::recv` --semantically_similar_to--> `SYS_IPC_RECV (7)`  [INFERRED] [semantically similar]
  src/ipc.rs → docs/syscalls.md
- `Capability System` --semantically_similar_to--> `Capability (kernel capability struct)`  [INFERRED] [semantically similar]
  CLAUDE.md → src/cap.rs
- `cap_grant` --semantically_similar_to--> `SYS_CAP_GRANT (4)`  [INFERRED] [semantically similar]
  src/cap.rs → docs/syscalls.md
- `cap_revoke` --semantically_similar_to--> `SYS_CAP_REVOKE (5)`  [INFERRED] [semantically similar]
  src/cap.rs → docs/syscalls.md

## Hyperedges (group relationships)
- **IPC Blocking/Waking Protocol** — ipc_send, ipc_recv, task_block_and_yield, task_wake_task, ipc_ipcendpoint [EXTRACTED 0.95]
- **Kernel Boot Initialization Sequence** — main_kmain, gdt_init, idt_init, pmm_init, heap_init, task_init, apic_init, ioapic_init [EXTRACTED 0.97]
- **ELF Exec and Ring-3 Entry Pipeline** — elf_exec, elf_load_segment_into, elf_alloc_user_stack_into, task_spawn_userspace_task, elf_exec_trampoline [EXTRACTED 0.95]
- **syscall_dispatch integrates task, cap, ipc, vmm, pmm subsystems** — syscall_syscall_dispatch, vmm_map_page, vmm_unmap_page, syscall_valid_user_range, syscall_with_user_access [EXTRACTED 0.95]
- **4-level page walk chain: walk_or_create/walk_existing operate on PML4/PDPT/PD/PT** — vmm_walk_or_create, vmm_walk_existing, vmm_pagetableentry, vmm_pagetable [EXTRACTED 0.95]
- **OROS Critical Daemon Triad (lythd + lythdist + lythmsg) triggers rollback** — oros_doc_lythd, oros_doc_lythdist, oros_doc_lythmsg, oros_doc_rollback_timer [EXTRACTED 1.00]
- **Boot Capability Delegation Flow (kmain → exec → lythd with mem/rollback/boot caps)** — claude_kmain, cap_initial_caps, elf_exec, lythd_init, ipc_bootinfo_msg [EXTRACTED 0.95]
- **IPC Blocking/Waking Task Flow (ipc send/recv ↔ task block_and_yield/wake_task)** — ipc_send, ipc_recv, ipc_blocking_model, claude_task [EXTRACTED 0.92]
- **ELF Load and Execute Flow (exec parses ELF, maps pages via VMM/PMM, allocates stack, spawns trampoline)** — elf_exec, claude_vmm, claude_pmm, elf_alloc_user_stack, elf_exec_trampoline [EXTRACTED 0.93]

## Communities

### Community 0 - "Kernel Core Subsystems"
Cohesion: 0.07
Nodes (66): ticks(), alloc_cap_id(), cap_grant(), cap_inherit(), create_object(), create_root_cap(), exec_trampoline(), exception_handler() (+58 more)

### Community 1 - "APIC & Interrupt Management"
Cohesion: 0.05
Nodes (61): apic::eoi, send_tlb_shootdown_ipi, timer_interrupt_handler, tlb_shootdown_handler, cap_inherit, Initial Boot Capabilities (mem, rollback, boot IPC), BootInfo message, Capability System (+53 more)

### Community 2 - "ELF Loader & Process Launch"
Cohesion: 0.12
Nodes (26): alloc_user_stack_into(), Elf64Ehdr, Elf64Phdr, ElfError, exec(), load_segment_into(), read_ehdr(), read_phdr() (+18 more)

### Community 3 - "Capability System (Code)"
Cohesion: 0.1
Nodes (18): cap_cascade_revoke(), cap_revoke(), Capability, CapabilityTable, CapError, CapHandle, CapKind, CapRights (+10 more)

### Community 4 - "Kernel Initialization"
Cohesion: 0.06
Nodes (36): apic::init, create_object, create_root_cap, alloc_user_stack_into, load_segment_into, gdt::init, heap::init, idt::init (+28 more)

### Community 5 - "Documentation Index"
Cohesion: 0.06
Nodes (36): Kernel Boot Sequence (docs/boot.md), Heap Documentation, PMM Documentation, VMM Documentation, Lythos Internal Docs Index, Context Switch (docs), Round-Robin Scheduler (docs), Task Model (docs) (+28 more)

### Community 6 - "Physical Memory Manager"
Cohesion: 0.12
Nodes (31): alloc_frames_contiguous(), free_frame(), free_frames_contiguous(), init(), is_used(), mark_range_free(), mark_range_used(), parse_mb1() (+23 more)

### Community 7 - "RFS Allocator & Main"
Cohesion: 0.17
Nodes (21): Allocator, attach_extent(), count_free(), create_file(), create_symlink(), Disk, find_last_entry_off(), get_u32() (+13 more)

### Community 8 - "Build Plan & Exceptions"
Cohesion: 0.08
Nodes (33): 14-Step Build Plan, exception_handler, ExceptionFrame, page_fault_handler, OROS Boot Sequence, Btrfs Subvolume Layout (@core, @store, @cfg, @home), lythos-linux-compat (Linux compat server), lysh (system shell) (+25 more)

### Community 9 - "Architecture Docs"
Cohesion: 0.09
Nodes (26): Critical Architecture Invariants, IOAPIC driver, Lythos Architecture, PCI config space scanner, Userspace (ring 3) — lythd, lythdist, lysh, VirtIO Block Driver, Local APIC, SpinLock (interrupt-safe) (+18 more)

### Community 10 - "IDT & Interrupt Descriptors"
Cohesion: 0.13
Nodes (12): IdtEntry, IdtPtr, init(), register_irq(), remap_and_mask_pic(), inb(), init(), outb() (+4 more)

### Community 11 - "Capability Docs"
Cohesion: 0.14
Nodes (15): cap_grant, cap_revoke, Capability (kernel capability struct), CapabilityTable (per-task cap table), CapHandle (opaque per-task handle), CapKind (enum: Memory/Ipc/Device/Rollback), CapRights (bitfield), cap_cascade_revoke operation (+7 more)

### Community 12 - "APIC Hardware Interface"
Cohesion: 0.32
Nodes (12): apic_read(), apic_write(), disable_pic(), eoi(), inb(), init(), outb(), pit_wait_ms() (+4 more)

### Community 13 - "GDT & Segment Setup"
Cohesion: 0.22
Nodes (8): encode_tss_descriptor(), GdtPtr, GlobalGdt, init(), GlobalTss, load(), Tss, tss_addr()

### Community 14 - "IOAPIC Driver"
Cohesion: 0.44
Nodes (7): init(), ioapic_read(), ioapic_write(), map_irq(), mask_irq(), redir_lo_reg(), unmask_irq()

### Community 15 - "Kernel Heap Allocator"
Cohesion: 0.32
Nodes (3): align_up(), FreeBlock, KernelAllocator

### Community 16 - "PCI Config Scanner"
Cohesion: 0.5
Nodes (7): cfg_read32(), cfg_write32(), config_addr(), find_device(), inl(), outl(), PciDevice

### Community 17 - "Syscall Entry & Dispatch"
Cohesion: 0.38
Nodes (6): init(), rdmsr(), SyscallFrame, valid_user_range(), with_user_access(), wrmsr()

### Community 18 - "SpinLock Guard"
Cohesion: 0.5
Nodes (1): SpinLockGuard<'_, T>

### Community 19 - "VirtIO Block I/O"
Cohesion: 0.67
Nodes (3): virtio_blk::read_sector, VirtioBlkDev::submit, virtio_blk::write_sector

### Community 20 - "RFS Host Tool"
Cohesion: 1.0
Nodes (2): Inode (RFS inode struct), RFS_V1 filesystem format

### Community 21 - "TSS"
Cohesion: 1.0
Nodes (1): Tss (task state segment struct)

### Community 22 - "IPC Ring Buffer"
Cohesion: 1.0
Nodes (1): RingBuffer (4 KiB shared page layout)

### Community 23 - "Cascade Revoke"
Cohesion: 1.0
Nodes (1): cap_cascade_revoke

### Community 24 - "VirtIO Block Device"
Cohesion: 1.0
Nodes (1): VirtioBlkDev (driver state)

### Community 25 - "Heap Allocator"
Cohesion: 1.0
Nodes (1): KernelAllocator (global heap allocator)

### Community 26 - "PCI Device"
Cohesion: 1.0
Nodes (1): PciDevice (PCI device descriptor)

### Community 27 - "IDT Entry"
Cohesion: 1.0
Nodes (1): IdtEntry (IDT gate descriptor)

### Community 28 - "IOAPIC IRQ Mask"
Cohesion: 1.0
Nodes (1): ioapic::mask_irq

### Community 29 - "APIC Tick Counter"
Cohesion: 1.0
Nodes (1): apic::ticks

### Community 30 - "Task State"
Cohesion: 1.0
Nodes (1): TaskState (enum)

### Community 31 - "Task Context"
Cohesion: 1.0
Nodes (1): TaskContext (saved rsp)

### Community 32 - "Task Scheduler"
Cohesion: 1.0
Nodes (1): Scheduler (round-robin scheduler)

### Community 33 - "Task Block"
Cohesion: 1.0
Nodes (1): block_task

### Community 34 - "Task Cap Table"
Cohesion: 1.0
Nodes (1): cap_table_ptr

### Community 35 - "Task Kill"
Cohesion: 1.0
Nodes (1): kill_task

### Community 36 - "VMA Insert"
Cohesion: 1.0
Nodes (1): vma_insert

### Community 37 - "VMA Remove"
Cohesion: 1.0
Nodes (1): vma_remove

### Community 38 - "Task Exists"
Cohesion: 1.0
Nodes (1): task_exists

### Community 39 - "Physical Address"
Cohesion: 1.0
Nodes (1): PhysAddr (physical address newtype)

### Community 40 - "PMM Contiguous Free"
Cohesion: 1.0
Nodes (1): free_frames_contiguous

### Community 41 - "ELF MMAP Test"
Cohesion: 1.0
Nodes (1): MMAP_TEST_ELF

### Community 42 - "Exec From Userspace"
Cohesion: 1.0
Nodes (1): EXEC_FROM_USER_ELF

### Community 43 - "Syscall User RSP"
Cohesion: 1.0
Nodes (1): SYSCALL_USER_RSP

### Community 44 - "VMM Kernel PML4"
Cohesion: 1.0
Nodes (1): vmm::kernel_pml4

### Community 45 - "SMP TODO"
Cohesion: 1.0
Nodes (1): Multi-processor SMP TODO

### Community 46 - "Display TODO"
Cohesion: 1.0
Nodes (1): Display/GUI TODO (framebuffer/webwm)

### Community 47 - "GDT Doc"
Cohesion: 1.0
Nodes (1): GDT

### Community 48 - "TSS Doc"
Cohesion: 1.0
Nodes (1): TSS

### Community 49 - "IDT Doc"
Cohesion: 1.0
Nodes (1): IDT

### Community 50 - "Serial Doc"
Cohesion: 1.0
Nodes (1): Serial / SpinLock

### Community 51 - "Memory Layout Doc"
Cohesion: 1.0
Nodes (1): Kernel Memory Layout

### Community 52 - "Syscall ABI Doc"
Cohesion: 1.0
Nodes (1): Syscall ABI (register conventions)

### Community 53 - "Capability Security Model"
Cohesion: 1.0
Nodes (1): Capability Security Model (no ambient authority)

### Community 54 - "SYS_MUNMAP"
Cohesion: 1.0
Nodes (1): SYS_MUNMAP (3)

### Community 55 - "SYS_ROLLBACK"
Cohesion: 1.0
Nodes (1): SYS_ROLLBACK (9)

### Community 56 - "mkrfs Tool"
Cohesion: 1.0
Nodes (1): mkrfs host formatting tool

### Community 57 - "IPC Limitations"
Cohesion: 1.0
Nodes (1): IPC Limitations (64-byte fixed, no multicast, no timeout)

## Knowledge Gaps
- **156 isolated node(s):** `GlobalTss`, `RingBuffer`, `EpTable`, `CapKind`, `KernelObject` (+151 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `SpinLock Guard`** (4 nodes): `SpinLockGuard<'_, T>`, `.deref()`, `.deref_mut()`, `.drop()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `RFS Host Tool`** (2 nodes): `Inode (RFS inode struct)`, `RFS_V1 filesystem format`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `TSS`** (1 nodes): `Tss (task state segment struct)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `IPC Ring Buffer`** (1 nodes): `RingBuffer (4 KiB shared page layout)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Cascade Revoke`** (1 nodes): `cap_cascade_revoke`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `VirtIO Block Device`** (1 nodes): `VirtioBlkDev (driver state)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Heap Allocator`** (1 nodes): `KernelAllocator (global heap allocator)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `PCI Device`** (1 nodes): `PciDevice (PCI device descriptor)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `IDT Entry`** (1 nodes): `IdtEntry (IDT gate descriptor)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `IOAPIC IRQ Mask`** (1 nodes): `ioapic::mask_irq`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `APIC Tick Counter`** (1 nodes): `apic::ticks`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task State`** (1 nodes): `TaskState (enum)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task Context`** (1 nodes): `TaskContext (saved rsp)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task Scheduler`** (1 nodes): `Scheduler (round-robin scheduler)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task Block`** (1 nodes): `block_task`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task Cap Table`** (1 nodes): `cap_table_ptr`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task Kill`** (1 nodes): `kill_task`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `VMA Insert`** (1 nodes): `vma_insert`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `VMA Remove`** (1 nodes): `vma_remove`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Task Exists`** (1 nodes): `task_exists`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Physical Address`** (1 nodes): `PhysAddr (physical address newtype)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `PMM Contiguous Free`** (1 nodes): `free_frames_contiguous`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `ELF MMAP Test`** (1 nodes): `MMAP_TEST_ELF`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Exec From Userspace`** (1 nodes): `EXEC_FROM_USER_ELF`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Syscall User RSP`** (1 nodes): `SYSCALL_USER_RSP`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `VMM Kernel PML4`** (1 nodes): `vmm::kernel_pml4`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `SMP TODO`** (1 nodes): `Multi-processor SMP TODO`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Display TODO`** (1 nodes): `Display/GUI TODO (framebuffer/webwm)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `GDT Doc`** (1 nodes): `GDT`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `TSS Doc`** (1 nodes): `TSS`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `IDT Doc`** (1 nodes): `IDT`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Serial Doc`** (1 nodes): `Serial / SpinLock`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Memory Layout Doc`** (1 nodes): `Kernel Memory Layout`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Syscall ABI Doc`** (1 nodes): `Syscall ABI (register conventions)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Capability Security Model`** (1 nodes): `Capability Security Model (no ambient authority)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `SYS_MUNMAP`** (1 nodes): `SYS_MUNMAP (3)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `SYS_ROLLBACK`** (1 nodes): `SYS_ROLLBACK (9)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `mkrfs Tool`** (1 nodes): `mkrfs host formatting tool`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `IPC Limitations`** (1 nodes): `IPC Limitations (64-byte fixed, no multicast, no timeout)`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `syscall_dispatch()` connect `Kernel Core Subsystems` to `ELF Loader & Process Launch`, `Capability System (Code)`, `Physical Memory Manager`, `APIC Hardware Interface`, `Syscall Entry & Dispatch`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Why does `Capability System` connect `APIC & Interrupt Management` to `Capability Docs`?**
  _High betweenness centrality (0.073) - this node is a cross-community bridge._
- **Why does `Capability (kernel capability struct)` connect `Capability Docs` to `APIC & Interrupt Management`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **Are the 41 inferred relationships involving `syscall_dispatch()` (e.g. with `core_smoke()` and `yield_task()`) actually correct?**
  _`syscall_dispatch()` has 41 INFERRED edges - model-reasoned connections that need verification._
- **Are the 24 inferred relationships involving `kmain()` (e.g. with `init()` and `PhysAddr`) actually correct?**
  _`kmain()` has 24 INFERRED edges - model-reasoned connections that need verification._
- **Are the 14 inferred relationships involving `VirtAddr` (e.g. with `create_endpoint()` and `init()`) actually correct?**
  _`VirtAddr` has 14 INFERRED edges - model-reasoned connections that need verification._
- **Are the 14 inferred relationships involving `PhysAddr` (e.g. with `init()` and `init()`) actually correct?**
  _`PhysAddr` has 14 INFERRED edges - model-reasoned connections that need verification._