---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L48"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/ELF_Loader_&_Process_Launch
---

# PageFlags

## Connections
- [[.bitor()]] - `method` [EXTRACTED]
- [[alloc_user_stack_into()]] - `calls` [INFERRED]
- [[segment_flags()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]

#graphify/code #graphify/INFERRED #community/ELF_Loader_&_Process_Launch