---
source_file: "src/pmm.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L281"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Kernel_Core_Subsystems
---

# alloc_frame()

## Connections
- [[PhysAddr]] - `calls` [EXTRACTED]
- [[alloc_table()]] - `calls` [INFERRED]
- [[alloc_user_stack_into()]] - `calls` [INFERRED]
- [[create_endpoint()]] - `calls` [INFERRED]
- [[init()]] - `calls` [INFERRED]
- [[init()_1]] - `calls` [INFERRED]
- [[kmain()]] - `calls` [INFERRED]
- [[load_segment_into()]] - `calls` [INFERRED]
- [[pmm.rs]] - `contains` [EXTRACTED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[userspace_smoke_task()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/Kernel_Core_Subsystems