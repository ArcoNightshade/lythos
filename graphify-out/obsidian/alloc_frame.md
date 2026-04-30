---
source_file: "src/pmm.rs"
type: "code"
community: "Kernel Initialization"
location: "line 281"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Initialization
---

# alloc_frame

## Connections
- [[Allocator (block bitmap allocator)]] - `semantically_similar_to` [INFERRED]
- [[alloc_user_stack_into]] - `calls` [EXTRACTED]
- [[heapinit]] - `calls` [EXTRACTED]
- [[ipccreate_endpoint]] - `calls` [EXTRACTED]
- [[load_segment_into]] - `calls` [EXTRACTED]
- [[userspace_smoke_task]] - `calls` [EXTRACTED]
- [[virtio_blkinit]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Initialization