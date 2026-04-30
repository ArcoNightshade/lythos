---
source_file: "src/apic.rs"
type: "code"
community: "APIC Hardware Interface"
location: "L346"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/APIC_Hardware_Interface
---

# send_tlb_shootdown_ipi()

## Connections
- [[apic.rs]] - `contains` [EXTRACTED]
- [[apic_read()]] - `calls` [EXTRACTED]
- [[apic_write()]] - `calls` [EXTRACTED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[unmap_page()]] - `calls` [INFERRED]

#graphify/code #graphify/EXTRACTED #community/APIC_Hardware_Interface