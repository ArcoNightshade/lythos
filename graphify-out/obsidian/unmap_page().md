---
source_file: "src/vmm.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L244"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Kernel_Core_Subsystems
---

# unmap_page()

## Connections
- [[.clear()]] - `calls` [EXTRACTED]
- [[PhysAddr]] - `calls` [INFERRED]
- [[kmain()]] - `calls` [INFERRED]
- [[send_tlb_shootdown_ipi()]] - `calls` [INFERRED]
- [[spawn_kernel_task()]] - `calls` [INFERRED]
- [[spawn_userspace_task()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[vmm.rs]] - `contains` [EXTRACTED]
- [[walk_existing()]] - `calls` [EXTRACTED]

#graphify/code #graphify/INFERRED #community/Kernel_Core_Subsystems