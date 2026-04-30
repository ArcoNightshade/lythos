---
type: community
cohesion: 0.38
members: 7
---

# Syscall Entry & Dispatch

**Cohesion:** 0.38 - loosely connected
**Members:** 7 nodes

## Members
- [[SyscallFrame]] - code - src/syscall.rs
- [[init()_9]] - code - src/syscall.rs
- [[rdmsr()]] - code - src/syscall.rs
- [[syscall.rs]] - code - src/syscall.rs
- [[valid_user_range()]] - code - src/syscall.rs
- [[with_user_access()]] - code - src/syscall.rs
- [[wrmsr()]] - code - src/syscall.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Syscall_Entry_&_Dispatch
SORT file.name ASC
```

## Connections to other communities
- 4 edges to [[_COMMUNITY_Kernel Core Subsystems]]

## Top bridge nodes
- [[syscall.rs]] - degree 8, connects to 1 community
- [[valid_user_range()]] - degree 2, connects to 1 community
- [[with_user_access()]] - degree 2, connects to 1 community