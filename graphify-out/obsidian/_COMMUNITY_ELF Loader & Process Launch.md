---
type: community
cohesion: 0.12
members: 41
---

# ELF Loader & Process Launch

**Cohesion:** 0.12 - loosely connected
**Members:** 41 nodes

## Members
- [[.address()]] - code - src/vmm.rs
- [[.as_u64()_1]] - code - src/vmm.rs
- [[.bitor()]] - code - src/vmm.rs
- [[.clear()]] - code - src/vmm.rs
- [[.is_huge()]] - code - src/vmm.rs
- [[.is_present()]] - code - src/vmm.rs
- [[.p1_idx()]] - code - src/vmm.rs
- [[.p2_idx()]] - code - src/vmm.rs
- [[.p3_idx()]] - code - src/vmm.rs
- [[.p4_idx()]] - code - src/vmm.rs
- [[.set()]] - code - src/vmm.rs
- [[.table()]] - code - src/vmm.rs
- [[.zero()_1]] - code - src/vmm.rs
- [[Elf64Ehdr]] - code - src/elf.rs
- [[Elf64Phdr]] - code - src/elf.rs
- [[ElfError]] - code - src/elf.rs
- [[PageFlags]] - code - src/vmm.rs
- [[PageTable]] - code - src/vmm.rs
- [[PageTableEntry]] - code - src/vmm.rs
- [[VirtAddr]] - code - src/vmm.rs
- [[alloc_table()]] - code - src/vmm.rs
- [[alloc_user_stack_into()]] - code - src/elf.rs
- [[create_user_page_table()]] - code - src/vmm.rs
- [[elf.rs]] - code - src/elf.rs
- [[exec()]] - code - src/elf.rs
- [[free_user_page_table()]] - code - src/vmm.rs
- [[init()_10]] - code - src/vmm.rs
- [[load_segment_into()]] - code - src/elf.rs
- [[map_page_in()]] - code - src/vmm.rs
- [[query_page()]] - code - src/vmm.rs
- [[query_page_in()]] - code - src/vmm.rs
- [[read_ehdr()]] - code - src/elf.rs
- [[read_phdr()]] - code - src/elf.rs
- [[segment_flags()]] - code - src/elf.rs
- [[unmap_page_in()]] - code - src/vmm.rs
- [[update_page_flags()]] - code - src/vmm.rs
- [[update_page_flags_in()]] - code - src/vmm.rs
- [[vmm.rs]] - code - src/vmm.rs
- [[walk_existing()]] - code - src/vmm.rs
- [[walk_or_create()]] - code - src/vmm.rs
- [[write_initial_stack_frame()]] - code - src/elf.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/ELF_Loader_&_Process_Launch
SORT file.name ASC
```

## Connections to other communities
- 38 edges to [[_COMMUNITY_Kernel Core Subsystems]]
- 1 edge to [[_COMMUNITY_IOAPIC Driver]]
- 1 edge to [[_COMMUNITY_APIC Hardware Interface]]
- 1 edge to [[_COMMUNITY_Physical Memory Manager]]

## Top bridge nodes
- [[VirtAddr]] - degree 21, connects to 3 communities
- [[free_user_page_table()]] - degree 7, connects to 2 communities
- [[vmm.rs]] - degree 19, connects to 1 community
- [[exec()]] - degree 12, connects to 1 community
- [[elf.rs]] - degree 11, connects to 1 community