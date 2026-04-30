---
type: community
cohesion: 0.44
members: 10
---

# IOAPIC Driver

**Cohesion:** 0.44 - moderately connected
**Members:** 10 nodes

## Members
- [[entry_count()]] - code - src/ioapic.rs
- [[init()_3]] - code - src/ioapic.rs
- [[ioapic.rs]] - code - src/ioapic.rs
- [[ioapic_read()]] - code - src/ioapic.rs
- [[ioapic_write()]] - code - src/ioapic.rs
- [[map_irq()]] - code - src/ioapic.rs
- [[mask_irq()]] - code - src/ioapic.rs
- [[redir_lo_reg()]] - code - src/ioapic.rs
- [[set_phys_base()]] - code - src/ioapic.rs
- [[unmask_irq()]] - code - src/ioapic.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/IOAPIC_Driver
SORT file.name ASC
```

## Connections to other communities
- 2 edges to [[_COMMUNITY_Kernel Core Subsystems]]
- 1 edge to [[_COMMUNITY_ELF Loader & Process Launch]]

## Top bridge nodes
- [[init()_3]] - degree 7, connects to 2 communities