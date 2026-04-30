---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L333"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# create_user_page_table()

## Connections
- [[.address()]] - `calls` [EXTRACTED]
- [[.as_u64()_1]] - `calls` [EXTRACTED]
- [[.table()]] - `calls` [EXTRACTED]
- [[PageTableEntry]] - `calls` [EXTRACTED]
- [[alloc_table()]] - `calls` [EXTRACTED]
- [[exec()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch