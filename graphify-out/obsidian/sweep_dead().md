---
source_file: "src/task.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L296"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Core_Subsystems
---

# sweep_dead()

## Connections
- [[PhysAddr]] - `calls` [INFERRED]
- [[VirtAddr]] - `calls` [INFERRED]
- [[block_and_yield()]] - `calls` [EXTRACTED]
- [[check_stack_canary()]] - `calls` [EXTRACTED]
- [[free_user_page_table()]] - `calls` [INFERRED]
- [[map_page()]] - `calls` [INFERRED]
- [[task.rs]] - `contains` [EXTRACTED]
- [[yield_task()]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Core_Subsystems