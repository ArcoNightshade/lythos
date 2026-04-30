---
type: community
cohesion: 0.14
members: 15
---

# Capability Docs

**Cohesion:** 0.14 - loosely connected
**Members:** 15 nodes

## Members
- [[CapHandle (opaque per-task handle)]] - code - src/cap.rs
- [[CapKind (enum MemoryIpcDeviceRollback)]] - code - src/cap.rs
- [[CapRights (bitfield)]] - code - src/cap.rs
- [[Capability (kernel capability struct)]] - code - src/cap.rs
- [[CapabilityTable (per-task cap table)]] - code - src/cap.rs
- [[IpcEndpoint (kernel IPC state)]] - code - src/ipc.rs
- [[KernelObject (enum MemoryIpcDeviceRollback)]] - code - src/cap.rs
- [[KernelObjectRef (generation-tagged index)]] - code - src/cap.rs
- [[SYS_CAP_REVOKE (5)]] - document - docs/syscalls.md
- [[SYS_MMAP (2)]] - document - docs/syscalls.md
- [[Task (scheduler task struct)]] - code - src/task.rs
- [[TaskId (u64 type alias)]] - code - src/task.rs
- [[cap_cascade_revoke operation]] - document - docs/capabilities.md
- [[cap_grant]] - code - src/cap.rs
- [[cap_revoke]] - code - src/cap.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Capability_Docs
SORT file.name ASC
```

## Connections to other communities
- 2 edges to [[_COMMUNITY_APIC & Interrupt Management]]
- 1 edge to [[_COMMUNITY_Build Plan & Exceptions]]

## Top bridge nodes
- [[Capability (kernel capability struct)]] - degree 8, connects to 1 community
- [[cap_grant]] - degree 2, connects to 1 community
- [[SYS_MMAP (2)]] - degree 2, connects to 1 community