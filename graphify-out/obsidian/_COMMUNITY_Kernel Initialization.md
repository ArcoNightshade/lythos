---
type: community
cohesion: 0.06
members: 36
---

# Kernel Initialization

**Cohesion:** 0.06 - loosely connected
**Members:** 36 nodes

## Members
- [[Allocator (block bitmap allocator)]] - code - tools/mkrfs/src/main.rs
- [[BootInfo (boot message struct)]] - code - src/main.rs
- [[Disk (image file IO)]] - code - tools/mkrfs/src/main.rs
- [[InodeTable (RFS inode table)]] - code - tools/mkrfs/src/main.rs
- [[alloc_frame]] - code - src/pmm.rs
- [[alloc_frames_contiguous]] - code - src/pmm.rs
- [[alloc_user_stack_into]] - code - src/elf.rs
- [[apicinit]] - code - src/apic.rs
- [[build_boot_info]] - code - src/main.rs
- [[core_smoke (integration checklist)]] - code - src/main.rs
- [[create_file]] - code - tools/mkrfs/src/main.rs
- [[create_object]] - code - src/cap.rs
- [[create_root_cap]] - code - src/cap.rs
- [[create_symlink]] - code - tools/mkrfs/src/main.rs
- [[free_frame_count]] - code - src/pmm.rs
- [[gdtinit]] - code - src/gdt.rs
- [[heapinit]] - code - src/heap.rs
- [[idtinit]] - code - src/idt.rs
- [[ioapicinit]] - code - src/ioapic.rs
- [[ioapicmap_irq]] - code - src/ioapic.rs
- [[ipccreate_endpoint]] - code - src/ipc.rs
- [[kmain (kernel entry point)]] - code - src/main.rs
- [[load_segment_into]] - code - src/elf.rs
- [[mkrfs main (RFS_V1 formatter)]] - code - tools/mkrfs/src/main.rs
- [[pcifind_device]] - code - src/pci.rs
- [[pmminit (PMM initializer)]] - code - src/pmm.rs
- [[populate (recursive dir builder)]] - code - tools/mkrfs/src/main.rs
- [[register_irq]] - code - src/idt.rs
- [[set_bootstrap_cap_table]] - code - src/task.rs
- [[spawn_kernel_task]] - code - src/task.rs
- [[taskinit]] - code - src/task.rs
- [[tssload]] - code - src/tss.rs
- [[tss_addr]] - code - src/tss.rs
- [[userspace_smoke_task]] - code - src/main.rs
- [[virtio_blkinit]] - code - src/virtio_blk.rs
- [[write_superblock]] - code - tools/mkrfs/src/main.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Kernel_Initialization
SORT file.name ASC
```

## Connections to other communities
- 4 edges to [[_COMMUNITY_APIC & Interrupt Management]]

## Top bridge nodes
- [[kmain (kernel entry point)]] - degree 17, connects to 1 community
- [[alloc_user_stack_into]] - degree 2, connects to 1 community
- [[load_segment_into]] - degree 2, connects to 1 community