---
source_file: "src/heap.rs"
type: "code"
community: "Kernel Heap Allocator"
location: "L1"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Heap_Allocator
---

# heap.rs

## Connections
- [[FreeBlock]] - `contains` [EXTRACTED]
- [[KernelAllocator]] - `contains` [EXTRACTED]
- [[__rust_alloc_error_handler()]] - `contains` [EXTRACTED]
- [[align_up()]] - `contains` [EXTRACTED]
- [[init()_1]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Heap_Allocator