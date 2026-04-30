---
source_file: "src/task.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L407"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Core_Subsystems
---

# task_exit()

## Connections
- [[check_stack_canary()]] - `calls` [EXTRACTED]
- [[exception_handler()]] - `calls` [INFERRED]
- [[get_sched()]] - `calls` [EXTRACTED]
- [[page_fault_handler()]] - `calls` [INFERRED]
- [[switch_cr3()]] - `calls` [EXTRACTED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[task.rs]] - `contains` [EXTRACTED]
- [[task_b()]] - `calls` [INFERRED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Core_Subsystems