---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L270"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# free_user_page_table()

## Connections
- [[.address()]] - `calls` [EXTRACTED]
- [[.as_u64()_1]] - `calls` [EXTRACTED]
- [[.is_huge()]] - `calls` [EXTRACTED]
- [[.is_present()]] - `calls` [EXTRACTED]
- [[free_frame()]] - `calls` [INFERRED]
- [[sweep_dead()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch