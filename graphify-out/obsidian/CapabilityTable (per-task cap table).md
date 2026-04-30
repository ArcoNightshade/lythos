---
source_file: "src/cap.rs"
type: "code"
community: "Capability Docs"
location: "line 226"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Capability_Docs
---

# CapabilityTable (per-task cap table)

## Connections
- [[CapHandle (opaque per-task handle)]] - `references` [EXTRACTED]
- [[Capability (kernel capability struct)]] - `shares_data_with` [EXTRACTED]
- [[Task (scheduler task struct)]] - `shares_data_with` [EXTRACTED]
- [[cap_cascade_revoke operation]] - `references` [EXTRACTED]
- [[cap_grant]] - `references` [EXTRACTED]
- [[cap_revoke]] - `references` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Capability_Docs