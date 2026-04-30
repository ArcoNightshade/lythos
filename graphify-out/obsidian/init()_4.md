---
source_file: "src/apic.rs"
type: "code"
community: "APIC Hardware Interface"
location: "L282"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/APIC_Hardware_Interface
---

# init()

## Connections
- [[PhysAddr]] - `calls` [INFERRED]
- [[VirtAddr]] - `calls` [INFERRED]
- [[apic.rs]] - `contains` [EXTRACTED]
- [[apic_read()]] - `calls` [EXTRACTED]
- [[apic_write()]] - `calls` [EXTRACTED]
- [[disable_pic()]] - `calls` [EXTRACTED]
- [[map_page()]] - `calls` [INFERRED]
- [[pit_wait_ms()]] - `calls` [EXTRACTED]
- [[read_apic_base()]] - `calls` [EXTRACTED]
- [[register_irq()]] - `calls` [INFERRED]

#graphify/code #graphify/EXTRACTED #community/APIC_Hardware_Interface