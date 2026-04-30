---
source_file: "src/task.rs"
type: "code"
community: "APIC & Interrupt Management"
location: "line 279"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/APIC_&_Interrupt_Management
---

# switch_cr3 (CR3 + TSS/RSP update)

## Connections
- [[block_and_yield]] - `calls` [EXTRACTED]
- [[send_tlb_shootdown_ipi]] - `conceptually_related_to` [INFERRED]
- [[set_rsp0]] - `calls` [EXTRACTED]
- [[task_exit]] - `calls` [EXTRACTED]
- [[yield_task]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/APIC_&_Interrupt_Management