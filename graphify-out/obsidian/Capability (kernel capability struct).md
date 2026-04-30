---
source_file: "src/cap.rs"
type: "code"
community: "Capability Docs"
location: "line 204"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Capability_Docs
---

# Capability (kernel capability struct)

## Connections
- [[CapHandle (opaque per-task handle)]] - `references` [EXTRACTED]
- [[CapKind (enum MemoryIpcDeviceRollback)]] - `references` [EXTRACTED]
- [[CapRights (bitfield)]] - `references` [EXTRACTED]
- [[Capability System]] - `semantically_similar_to` [INFERRED]
- [[CapabilityTable (per-task cap table)]] - `shares_data_with` [EXTRACTED]
- [[IpcEndpoint (kernel IPC state)]] - `shares_data_with` [EXTRACTED]
- [[KernelObjectRef (generation-tagged index)]] - `references` [EXTRACTED]
- [[SYS_MMAP (2)]] - `references` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Capability_Docs