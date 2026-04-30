---
type: community
cohesion: 0.12
members: 35
---

# Physical Memory Manager

**Cohesion:** 0.12 - loosely connected
**Members:** 35 nodes

## Members
- [[.submit()]] - code - src/virtio_blk.rs
- [[.write_desc()]] - code - src/virtio_blk.rs
- [[DevState]] - code - src/virtio_blk.rs
- [[VirtioBlkDev]] - code - src/virtio_blk.rs
- [[alloc_frames_contiguous()]] - code - src/pmm.rs
- [[avail_idx_pa()]] - code - src/virtio_blk.rs
- [[avail_pa()]] - code - src/virtio_blk.rs
- [[avail_ring_pa()]] - code - src/virtio_blk.rs
- [[capacity_sectors()]] - code - src/virtio_blk.rs
- [[desc_pa()]] - code - src/virtio_blk.rs
- [[dev_mut()]] - code - src/virtio_blk.rs
- [[dev_ref()]] - code - src/virtio_blk.rs
- [[free_frame()]] - code - src/pmm.rs
- [[free_frames_contiguous()]] - code - src/pmm.rs
- [[inb()]] - code - src/virtio_blk.rs
- [[init()_7]] - code - src/pmm.rs
- [[init()]] - code - src/virtio_blk.rs
- [[inl()]] - code - src/virtio_blk.rs
- [[is_present()]] - code - src/virtio_blk.rs
- [[is_used()]] - code - src/pmm.rs
- [[mark_range_free()]] - code - src/pmm.rs
- [[mark_range_used()]] - code - src/pmm.rs
- [[outb()]] - code - src/virtio_blk.rs
- [[outl()]] - code - src/virtio_blk.rs
- [[outw()]] - code - src/virtio_blk.rs
- [[parse_mb1()]] - code - src/pmm.rs
- [[parse_mb2()]] - code - src/pmm.rs
- [[pmm.rs]] - code - src/pmm.rs
- [[read_sector()]] - code - src/virtio_blk.rs
- [[set_free()]] - code - src/pmm.rs
- [[set_used()]] - code - src/pmm.rs
- [[used_idx_pa()]] - code - src/virtio_blk.rs
- [[used_pa()]] - code - src/virtio_blk.rs
- [[virtio_blk.rs]] - code - src/virtio_blk.rs
- [[write_sector()]] - code - src/virtio_blk.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Physical_Memory_Manager
SORT file.name ASC
```

## Connections to other communities
- 12 edges to [[_COMMUNITY_Kernel Core Subsystems]]
- 1 edge to [[_COMMUNITY_PCI Config Scanner]]
- 1 edge to [[_COMMUNITY_ELF Loader & Process Launch]]

## Top bridge nodes
- [[init()]] - degree 11, connects to 2 communities
- [[free_frame()]] - degree 7, connects to 2 communities
- [[pmm.rs]] - degree 14, connects to 1 community
- [[alloc_frames_contiguous()]] - degree 5, connects to 1 community
- [[free_frames_contiguous()]] - degree 4, connects to 1 community