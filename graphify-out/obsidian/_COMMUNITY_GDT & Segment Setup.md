---
type: community
cohesion: 0.22
members: 11
---

# GDT & Segment Setup

**Cohesion:** 0.22 - loosely connected
**Members:** 11 nodes

## Members
- [[.zero()]] - code - src/tss.rs
- [[GdtPtr]] - code - src/gdt.rs
- [[GlobalGdt]] - code - src/gdt.rs
- [[GlobalTss]] - code - src/tss.rs
- [[Tss]] - code - src/tss.rs
- [[encode_tss_descriptor()]] - code - src/gdt.rs
- [[gdt.rs]] - code - src/gdt.rs
- [[init()_6]] - code - src/gdt.rs
- [[load()]] - code - src/tss.rs
- [[tss.rs]] - code - src/tss.rs
- [[tss_addr()]] - code - src/tss.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/GDT_&_Segment_Setup
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_Kernel Core Subsystems]]

## Top bridge nodes
- [[tss.rs]] - degree 5, connects to 1 community