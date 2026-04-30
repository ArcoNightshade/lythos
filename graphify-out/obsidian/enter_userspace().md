---
source_file: "src/syscall.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L941"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Kernel_Core_Subsystems
---

# enter_userspace()

## Connections
- [[current_kernel_stack_top()]] - `calls` [INFERRED]
- [[exec_trampoline()]] - `calls` [INFERRED]
- [[set_rsp0()]] - `calls` [INFERRED]
- [[syscall.rs]] - `contains` [EXTRACTED]
- [[userspace_smoke_task()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/Kernel_Core_Subsystems