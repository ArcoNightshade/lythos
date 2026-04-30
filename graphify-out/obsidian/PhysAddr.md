---
source_file: "src/pmm.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L46"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Kernel_Core_Subsystems
---

# PhysAddr

## Connections
- [[.address()]] - `calls` [INFERRED]
- [[.as_u64()]] - `method` [EXTRACTED]
- [[alloc_frame()]] - `calls` [EXTRACTED]
- [[alloc_frames_contiguous()]] - `calls` [EXTRACTED]
- [[alloc_user_stack_into()]] - `calls` [INFERRED]
- [[free_frames_contiguous()]] - `calls` [EXTRACTED]
- [[init()]] - `calls` [INFERRED]
- [[init()_3]] - `calls` [INFERRED]
- [[init()_4]] - `calls` [INFERRED]
- [[init()_10]] - `calls` [INFERRED]
- [[kernel_pml4()]] - `calls` [INFERRED]
- [[kmain()]] - `calls` [INFERRED]
- [[map_page()]] - `calls` [INFERRED]
- [[pmm.rs]] - `contains` [EXTRACTED]
- [[query_page()]] - `calls` [INFERRED]
- [[sweep_dead()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[unmap_page()]] - `calls` [INFERRED]
- [[update_page_flags()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/Kernel_Core_Subsystems