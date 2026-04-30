---
source_file: "src/elf.rs"
type: "code"
community: "APIC & Interrupt Management"
location: "line 357"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/APIC_&_Interrupt_Management
---

# elf::exec (ELF64 loader and spawner)

## Connections
- [[Capability System]] - `references` [EXTRACTED]
- [[ELF Loader]] - `semantically_similar_to` [INFERRED]
- [[ELF Loader Limitations (no ASLR, no PIE, shared PML4)]] - `references` [EXTRACTED]
- [[IPC_RECEIVER_ELF]] - `references` [EXTRACTED]
- [[IPC_SENDER_ELF]] - `references` [EXTRACTED]
- [[LYTHD_ELF embedded blob]] - `references` [EXTRACTED]
- [[PT_LOAD segment loading]] - `references` [EXTRACTED]
- [[Physical Memory Manager (PMM)]] - `references` [EXTRACTED]
- [[SMOKE_ELF (hand-crafted test binary)]] - `references` [EXTRACTED]
- [[SYS_EXEC (10)]] - `references` [EXTRACTED]
- [[Virtual Memory Manager (VMM)]] - `references` [EXTRACTED]
- [[alloc_user_stack]] - `calls` [EXTRACTED]
- [[alloc_user_stack_into]] - `calls` [EXTRACTED]
- [[exec_trampoline]] - `calls` [EXTRACTED]
- [[kmain (kernel entry point)]] - `calls` [EXTRACTED]
- [[load_segment_into]] - `calls` [EXTRACTED]
- [[spawn_userspace_task]] - `calls` [EXTRACTED]
- [[write_initial_stack_frame]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/APIC_&_Interrupt_Management