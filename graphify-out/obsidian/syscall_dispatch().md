---
source_file: "src/syscall.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L339"
tags:
  - graphify/code
  - graphify/INFERRED
  - community/Kernel_Core_Subsystems
---

# syscall_dispatch()

## Connections
- [[.is_present()]] - `calls` [INFERRED]
- [[.new()_5]] - `calls` [INFERRED]
- [[CapHandle]] - `calls` [INFERRED]
- [[CapRights]] - `calls` [INFERRED]
- [[PageFlags]] - `calls` [INFERRED]
- [[PhysAddr]] - `calls` [INFERRED]
- [[VirtAddr]] - `calls` [INFERRED]
- [[alloc_frame()]] - `calls` [INFERRED]
- [[cap_cascade_revoke()]] - `calls` [INFERRED]
- [[cap_grant()]] - `calls` [INFERRED]
- [[cap_table_ptr()]] - `calls` [INFERRED]
- [[core_smoke()]] - `calls` [INFERRED]
- [[create_endpoint()]] - `calls` [INFERRED]
- [[create_object()]] - `calls` [INFERRED]
- [[create_root_cap()]] - `calls` [INFERRED]
- [[current_page_table()]] - `calls` [INFERRED]
- [[current_task_id()]] - `calls` [INFERRED]
- [[exec()]] - `calls` [INFERRED]
- [[for_each_task()]] - `calls` [INFERRED]
- [[free_frame()]] - `calls` [INFERRED]
- [[free_frame_count()]] - `calls` [INFERRED]
- [[get_object()]] - `calls` [INFERRED]
- [[kill_task()]] - `calls` [INFERRED]
- [[map_page()]] - `calls` [INFERRED]
- [[map_page_in()]] - `calls` [INFERRED]
- [[query_page_in()]] - `calls` [INFERRED]
- [[read_sector()]] - `calls` [INFERRED]
- [[recv()]] - `calls` [INFERRED]
- [[recv_cap()]] - `calls` [INFERRED]
- [[send()]] - `calls` [INFERRED]
- [[send_cap()]] - `calls` [INFERRED]
- [[send_tlb_shootdown_ipi()]] - `calls` [INFERRED]
- [[syscall.rs]] - `contains` [EXTRACTED]
- [[task_exit()]] - `calls` [INFERRED]
- [[task_status_raw()]] - `calls` [INFERRED]
- [[ticks()]] - `calls` [INFERRED]
- [[unmap_page()]] - `calls` [INFERRED]
- [[unmap_page_in()]] - `calls` [INFERRED]
- [[valid_user_range()]] - `calls` [EXTRACTED]
- [[vma_insert()]] - `calls` [INFERRED]
- [[vma_remove()]] - `calls` [INFERRED]
- [[with_user_access()]] - `calls` [EXTRACTED]
- [[write_sector()]] - `calls` [INFERRED]
- [[yield_task()]] - `calls` [INFERRED]

#graphify/code #graphify/INFERRED #community/Kernel_Core_Subsystems