---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L215"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# query_page()

## Connections
- [[.address()]] - `calls` [EXTRACTED]
- [[.is_present()]] - `calls` [EXTRACTED]
- [[PhysAddr]] - `calls` [INFERRED]
- [[spawn_kernel_task()]] - `calls` [INFERRED]
- [[spawn_userspace_task()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]
- [[walk_existing()]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch