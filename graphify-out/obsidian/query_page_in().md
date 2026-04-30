---
source_file: "src/vmm.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L372"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# query_page_in()

## Connections
- [[.address()]] - `calls` [EXTRACTED]
- [[.is_present()]] - `calls` [EXTRACTED]
- [[load_segment_into()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]
- [[walk_existing()]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch