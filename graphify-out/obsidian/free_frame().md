---
source_file: "src/pmm.rs"
type: "code"
community: "Physical Memory Manager"
location: "L302"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Physical_Memory_Manager
---

# free_frame()

## Connections
- [[free_frames_contiguous()]] - `calls` [EXTRACTED]
- [[free_user_page_table()]] - `calls` [INFERRED]
- [[init()]] - `calls` [INFERRED]
- [[kmain()]] - `calls` [INFERRED]
- [[pmm.rs]] - `contains` [EXTRACTED]
- [[set_free()]] - `calls` [EXTRACTED]
- [[syscall_dispatch()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/Physical_Memory_Manager