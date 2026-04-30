---
source_file: "src/ioapic.rs"
type: "code"
community: "IOAPIC Driver"
location: "L118"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/IOAPIC_Driver
---

# init()

## Connections
- [[PhysAddr]] - `calls` [INFERRED]
- [[VirtAddr]] - `calls` [INFERRED]
- [[ioapic.rs]] - `contains` [EXTRACTED]
- [[ioapic_read()]] - `calls` [EXTRACTED]
- [[ioapic_write()]] - `calls` [EXTRACTED]
- [[map_page()]] - `calls` [INFERRED]
- [[redir_lo_reg()]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/IOAPIC_Driver