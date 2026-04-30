---
source_file: "src/elf.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L231"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/ELF_Loader_&_Process_Launch
---

# alloc_user_stack_into()

## Connections
- [[PageFlags]] - `calls` [INFERRED]
- [[PhysAddr]] - `calls` [INFERRED]
- [[VirtAddr]] - `calls` [INFERRED]
- [[alloc_frame()]] - `calls` [INFERRED]
- [[elf.rs]] - `contains` [EXTRACTED]
- [[exec()]] - `calls` [EXTRACTED]
- [[map_page_in()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/ELF_Loader_&_Process_Launch