---
source_file: "src/task.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L362"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Core_Subsystems
---

# yield_task()

## Connections
- [[check_stack_canary()]] - `calls` [EXTRACTED]
- [[core_smoke()]] - `calls` [INFERRED]
- [[get_sched()]] - `calls` [EXTRACTED]
- [[kmain()]] - `calls` [INFERRED]
- [[sweep_dead()]] - `calls` [EXTRACTED]
- [[switch_cr3()]] - `calls` [EXTRACTED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[task.rs]] - `contains` [EXTRACTED]
- [[task_b()]] - `calls` [INFERRED]
- [[timer_interrupt_handler()]] - `calls` [INFERRED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Core_Subsystems