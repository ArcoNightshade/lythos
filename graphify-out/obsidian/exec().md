---
source_file: "src/elf.rs"
type: "code"
community: "ELF Loader & Process Launch"
location: "L357"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/ELF_Loader_&_Process_Launch
---

# exec()

## Connections
- [[VirtAddr]] - `calls` [INFERRED]
- [[alloc_user_stack_into()]] - `calls` [EXTRACTED]
- [[core_smoke()]] - `calls` [INFERRED]
- [[create_user_page_table()]] - `calls` [INFERRED]
- [[elf.rs]] - `contains` [EXTRACTED]
- [[kmain()]] - `calls` [INFERRED]
- [[load_segment_into()]] - `calls` [EXTRACTED]
- [[read_ehdr()]] - `calls` [EXTRACTED]
- [[read_phdr()]] - `calls` [EXTRACTED]
- [[spawn_userspace_task()]] - `calls` [INFERRED]
- [[syscall_dispatch()]] - `calls` [INFERRED]
- [[write_initial_stack_frame()]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/ELF_Loader_&_Process_Launch