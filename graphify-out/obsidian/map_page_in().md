---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L365"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# map_page_in()

## Connections
- [[.set()]] - `calls` [EXTRACTED]
- [[alloc_user_stack_into()]] - `calls` [INFERRED]
- [[load_segment_into()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]
- [[walk_or_create()]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch