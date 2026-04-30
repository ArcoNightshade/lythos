---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L398"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# init()

## Connections
- [[.as_u64()_1]] - `calls` [EXTRACTED]
- [[.table()]] - `calls` [EXTRACTED]
- [[PageTableEntry]] - `calls` [EXTRACTED]
- [[PhysAddr]] - `calls` [INFERRED]
- [[VirtAddr]] - `calls` [EXTRACTED]
- [[alloc_table()]] - `calls` [EXTRACTED]
- [[kmain()]] - `calls` [INFERRED]
- [[map_page()]] - `calls` [EXTRACTED]
- [[vmm.rs]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch