---
type: community
cohesion: 0.50
members: 8
---

# PCI Config Scanner

**Cohesion:** 0.50 - moderately connected
**Members:** 8 nodes

## Members
- [[PciDevice]] - code - src/pci.rs
- [[cfg_read32()]] - code - src/pci.rs
- [[cfg_write32()]] - code - src/pci.rs
- [[config_addr()]] - code - src/pci.rs
- [[find_device()]] - code - src/pci.rs
- [[inl()_1]] - code - src/pci.rs
- [[outl()_1]] - code - src/pci.rs
- [[pci.rs]] - code - src/pci.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/PCI_Config_Scanner
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_Physical Memory Manager]]

## Top bridge nodes
- [[find_device()]] - degree 4, connects to 1 community