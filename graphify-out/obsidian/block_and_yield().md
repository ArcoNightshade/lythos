---
source_file: "src/task.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L692"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Core_Subsystems
---

# block_and_yield()

## Connections
- [[check_stack_canary()]] - `calls` [EXTRACTED]
- [[get_sched()]] - `calls` [EXTRACTED]
- [[recv()]] - `calls` [INFERRED]
- [[recv_cap()]] - `calls` [INFERRED]
- [[send()]] - `calls` [INFERRED]
- [[send_cap()]] - `calls` [INFERRED]
- [[sweep_dead()]] - `calls` [EXTRACTED]
- [[switch_cr3()]] - `calls` [EXTRACTED]
- [[task.rs]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Core_Subsystems