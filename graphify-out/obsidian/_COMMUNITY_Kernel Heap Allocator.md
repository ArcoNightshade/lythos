---
type: community
cohesion: 0.32
members: 8
---

# Kernel Heap Allocator

**Cohesion:** 0.32 - loosely connected
**Members:** 8 nodes

## Members
- [[.alloc()_2]] - code - src/heap.rs
- [[.dealloc()]] - code - src/heap.rs
- [[.new()_3]] - code - src/heap.rs
- [[FreeBlock]] - code - src/heap.rs
- [[KernelAllocator]] - code - src/heap.rs
- [[__rust_alloc_error_handler()]] - code - src/heap.rs
- [[align_up()]] - code - src/heap.rs
- [[heap.rs]] - code - src/heap.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Kernel_Heap_Allocator
SORT file.name ASC
```

## Connections to other communities
- 2 edges to [[_COMMUNITY_Kernel Core Subsystems]]

## Top bridge nodes
- [[heap.rs]] - degree 5, connects to 1 community
- [[.new()_3]] - degree 2, connects to 1 community