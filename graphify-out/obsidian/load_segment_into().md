---
source_file: "src/elf.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L168"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/ELF_Loader_&_Process_Launch
---

# load_segment_into()

## Connections
- [[VirtAddr]] - `calls` [INFERRED]
- [[alloc_frame()]] - `calls` [INFERRED]
- [[elf.rs]] - `contains` [EXTRACTED]
- [[exec()]] - `calls` [EXTRACTED]
- [[map_page_in()]] - `calls` [INFERRED]
- [[query_page_in()]] - `calls` [INFERRED]
- [[segment_flags()]] - `calls` [EXTRACTED]
- [[update_page_flags_in()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/ELF_Loader_&_Process_Launch