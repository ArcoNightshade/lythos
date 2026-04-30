---
source_file: "src/main.rs"
type: "code"
community: "Kernel Initialization"
location: "line 40"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Initialization
---

# kmain (kernel entry point)

## Connections
- [[apicinit]] - `calls` [EXTRACTED]
- [[build_boot_info]] - `calls` [EXTRACTED]
- [[core_smoke (integration checklist)]] - `calls` [EXTRACTED]
- [[create_object]] - `calls` [EXTRACTED]
- [[create_root_cap]] - `calls` [EXTRACTED]
- [[elfexec (ELF64 loader and spawner)]] - `calls` [EXTRACTED]
- [[gdtinit]] - `calls` [EXTRACTED]
- [[heapinit]] - `calls` [EXTRACTED]
- [[idtinit]] - `calls` [EXTRACTED]
- [[ioapicinit]] - `calls` [EXTRACTED]
- [[ipccreate_endpoint]] - `calls` [EXTRACTED]
- [[ipcsend]] - `calls` [EXTRACTED]
- [[pmminit (PMM initializer)]] - `calls` [EXTRACTED]
- [[set_bootstrap_cap_table]] - `calls` [EXTRACTED]
- [[spawn_kernel_task]] - `calls` [EXTRACTED]
- [[taskinit]] - `calls` [EXTRACTED]
- [[virtio_blkinit]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Initialization