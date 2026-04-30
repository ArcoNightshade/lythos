---
source_file: "src/virtio_blk.rs"
type: "code"
community: "Physical Memory Manager"
location: "L336"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Physical_Memory_Manager
---

# init()

## Connections
- [[PhysAddr]] - `calls` [INFERRED]
- [[alloc_frame()]] - `calls` [INFERRED]
- [[alloc_frames_contiguous()]] - `calls` [INFERRED]
- [[find_device()]] - `calls` [INFERRED]
- [[free_frame()]] - `calls` [INFERRED]
- [[free_frames_contiguous()]] - `calls` [INFERRED]
- [[inl()]] - `calls` [EXTRACTED]
- [[outb()]] - `calls` [EXTRACTED]
- [[outl()]] - `calls` [EXTRACTED]
- [[outw()]] - `calls` [EXTRACTED]
- [[virtio_blk.rs]] - `contains` [EXTRACTED]

#graphify/code #graphify/INFERRED #community/Physical_Memory_Manager