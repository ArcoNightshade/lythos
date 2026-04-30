---
source_file: "src/task.rs"
type: "code"
community: "APIC & Interrupt Management"
location: "line 362"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/APIC_&_Interrupt_Management
---

# yield_task

## Connections
- [[sweep_dead]] - `calls` [EXTRACTED]
- [[switch_cr3 (CR3 + TSSRSP update)]] - `calls` [EXTRACTED]
- [[timer_interrupt_handler]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/APIC_&_Interrupt_Management