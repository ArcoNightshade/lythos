---
type: community
cohesion: 0.32
members: 13
---

# APIC Hardware Interface

**Cohesion:** 0.32 - loosely connected
**Members:** 13 nodes

## Members
- [[apic.rs]] - code - src/apic.rs
- [[apic_read()]] - code - src/apic.rs
- [[apic_write()]] - code - src/apic.rs
- [[disable_pic()]] - code - src/apic.rs
- [[eoi()]] - code - src/apic.rs
- [[inb()_1]] - code - src/apic.rs
- [[init()_4]] - code - src/apic.rs
- [[outb()_1]] - code - src/apic.rs
- [[pit_wait_ms()]] - code - src/apic.rs
- [[read_apic_base()]] - code - src/apic.rs
- [[send_tlb_shootdown_ipi()]] - code - src/apic.rs
- [[timer_interrupt_handler()]] - code - src/apic.rs
- [[tlb_shootdown_handler()]] - code - src/apic.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/APIC_Hardware_Interface
SORT file.name ASC
```

## Connections to other communities
- 6 edges to [[_COMMUNITY_Kernel Core Subsystems]]
- 1 edge to [[_COMMUNITY_IDT & Interrupt Descriptors]]
- 1 edge to [[_COMMUNITY_ELF Loader & Process Launch]]

## Top bridge nodes
- [[init()_4]] - degree 10, connects to 3 communities
- [[apic.rs]] - degree 13, connects to 1 community
- [[send_tlb_shootdown_ipi()]] - degree 5, connects to 1 community
- [[timer_interrupt_handler()]] - degree 3, connects to 1 community