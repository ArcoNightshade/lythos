---
source_file: "src/elf.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L1"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# elf.rs

## Connections
- [[Elf64Ehdr]] - `contains` [EXTRACTED]
- [[Elf64Phdr]] - `contains` [EXTRACTED]
- [[ElfError]] - `contains` [EXTRACTED]
- [[alloc_user_stack_into()]] - `contains` [EXTRACTED]
- [[exec()]] - `contains` [EXTRACTED]
- [[exec_trampoline()]] - `contains` [EXTRACTED]
- [[load_segment_into()]] - `contains` [EXTRACTED]
- [[read_ehdr()]] - `contains` [EXTRACTED]
- [[read_phdr()]] - `contains` [EXTRACTED]
- [[segment_flags()]] - `contains` [EXTRACTED]
- [[write_initial_stack_frame()]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch