---
type: community
cohesion: 0.09
members: 26
---

# Architecture Docs

**Cohesion:** 0.09 - loosely connected
**Members:** 26 nodes

## Members
- [[Critical Architecture Invariants]] - document - docs/architecture.md
- [[IOAPIC Driver (srcioapic.rs)]] - document - docs/drivers.md
- [[IOAPIC Redirection Table Entry (RTE)]] - document - docs/drivers.md
- [[IOAPIC driver]] - document - docs/architecture.md
- [[Local APIC]] - document - CLAUDE.md
- [[Lythos Architecture]] - document - docs/architecture.md
- [[PCI Config Space Scanner (srcpci.rs)]] - document - docs/drivers.md
- [[PCI config space scanner]] - document - docs/architecture.md
- [[Planned VFS Syscalls (SYS_OPEN=22 through SYS_READDIR=27)]] - document - docs/syscalls.md
- [[RFS (Raptor File System)]] - document - docs/rfs.md
- [[RFS Block Bitmap]] - document - docs/rfs.md
- [[RFS Directory Entry]] - document - docs/rfs.md
- [[RFS Extent (16 bytes)]] - document - docs/rfs.md
- [[RFS Fast Symlink (inline target)]] - document - docs/rfs.md
- [[RFS Inode (128 bytes)]] - document - docs/rfs.md
- [[RFS Kernel Driver (planned, srcrfs.rs)]] - document - docs/rfs.md
- [[RFS Overflow Extent Block]] - document - docs/rfs.md
- [[RFS Superblock]] - document - docs/rfs.md
- [[SYS_BLK_READ (20)]] - document - docs/syscalls.md
- [[SYS_BLK_WRITE (21)]] - document - docs/syscalls.md
- [[SpinLock (interrupt-safe)]] - document - CLAUDE.md
- [[Userspace (ring 3) — lythd, lythdist, lysh]] - document - docs/architecture.md
- [[VirtIO 3-Descriptor IO Chain]] - document - docs/drivers.md
- [[VirtIO Block Device Driver (srcvirtio_blk.rs)]] - document - docs/drivers.md
- [[VirtIO Block Driver]] - document - docs/architecture.md
- [[VirtIO Virtqueue Layout]] - document - docs/drivers.md

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Architecture_Docs
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_APIC & Interrupt Management]]

## Top bridge nodes
- [[Lythos Architecture]] - degree 6, connects to 1 community