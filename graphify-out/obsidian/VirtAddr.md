---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L33"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/ELF_Loader_&_Process_Launch
---

# VirtAddr

## Connections
- [[.as_u64()_1]] - `method` [EXTRACTED]
- [[.p1_idx()]] - `method` [EXTRACTED]
- [[.p2_idx()]] - `method` [EXTRACTED]
- [[.p3_idx()]] - `method` [EXTRACTED]
- [[.p4_idx()]] - `method` [EXTRACTED]
- [[alloc_user_stack_into()]] - `calls` [INFERRED]
- [[create_endpoint()]] - `calls` [INFERRED]
- [[exec()]] - `calls` [INFERRED]
- [[init()_10]] - `calls` [EXTRACTED]
- [[init()_1]] - `calls` [INFERRED]
- [[init()_3]] - `calls` [INFERRED]
- [[init()_4]] - `calls` [INFERRED]
- [[kmain()]] - `calls` [INFERRED]
- [[load_segment_into()]] - `calls` [INFERRED]
- [[spawn_kernel_task()]] - `calls` [INFERRED]
- [[spawn_userspace_task()]] - `calls` [INFERRED]
- [[sweep_dead()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[userspace_smoke_task()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]
- [[write_initial_stack_frame()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/ELF_Loader_&_Process_Launch